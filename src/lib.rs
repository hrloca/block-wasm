#![feature(drain_filter)]

use std::{cell::RefCell, panic, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::*;

pub mod blocks;
pub mod board;
pub mod dom;
pub mod libs;
pub mod store;
pub mod tools;
pub mod ui;

mod playground;

use libs::*;
use store::*;
use tools::frame_engine::FrameEngine;
use tools::store::Store;

pub struct Ctx<'a> {
    pub state: &'a store::State,
    pub action_dispacher: ActionDispacher<'a>,
    pub canvas_ctx: &'a CanvasRenderingContext2d,
}

#[wasm_bindgen()]
pub async fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let store = Rc::new(Store::create(store::State::create(), reducer));

    store.subscribe(Box::new(|state| {
        log!("store.subscribe: {:?}", state.falling_point);
    }));

    let h = ui::HTML::new();
    let h = Rc::new(h);
    let canvas_el = JsCast::dyn_into::<HtmlCanvasElement>(h.el("canvas")).unwrap();
    let canvas = ui::Canvas::create(canvas_el);
    let field = ui::Field::create(
        &canvas.el,
        blocks::BOARD_COL,
        blocks::BOARD_RAW,
        ui::WIDTH,
        ui::HEIGHT,
    );

    let particle_render = Rc::new(RefCell::new(ui::ParticleRender::create()));
    let pr = Rc::clone(&particle_render);

    let scheduler = Rc::new(RefCell::new(
        ui::TaskScheduler::<ui::ParticleAction>::create(Box::new(move |task_id, task| {
            pr.borrow_mut().dispatch_with(task_id, task);
        })),
    ));

    {
        let store = Rc::clone(&store);
        let prender = Rc::clone(&particle_render);
        let scheduler = Rc::clone(&scheduler);

        FrameEngine::new(Box::new(move || {
            let state = &store.get_state();
            let action_dispacher = ActionDispacher::new(&store);
            let ctx = Ctx {
                state,
                canvas_ctx: &canvas.ctx,
                action_dispacher,
            };

            field.render(&ctx);
            prender.borrow_mut().render(&ctx);
            scheduler
                .borrow_mut()
                .processing_queue(&state.complete_tasks, &ctx);
        }))
        .start();
    }

    {
        let scheduler = Rc::clone(&scheduler);
        let handler = Closure::wrap(Box::new(move |e: MouseEvent| {
            let offset_x = e.offset_x();
            let offset_y = e.offset_y();
            let target = ui::Field::offset_to_point((offset_x, offset_y));
            let mut scheduler = scheduler.borrow_mut();

            scheduler.put(Box::new(move |ctx| {
                let deleting_point = ctx.state.deleting_point.clone();
                let falling_point = ctx.state.falling_point.clone();
                let mut ignore_points: Vec<board::Point> = deleting_point.into_iter().collect();
                let mut falling_points: Vec<board::Point> = falling_point.into_iter().collect();

                ignore_points.append(&mut falling_points);
                let final_ignore = blocks::should_not_change(&ctx.state.blocks, ignore_points);

                log!("{:?}", final_ignore);

                if let Some((a, b)) =
                    blocks::should_changed(&ctx.state.blocks, target, final_ignore)
                {
                    Some(ui::ParticleAction::Change(a, b))
                } else {
                    None
                }
            }));

            scheduler.put(Box::new(move |ctx| {
                let state = ctx.state;
                let (_, moves) = blocks::fall_scanning(&state.blocks);
                if moves.is_empty() {
                    None
                } else {
                    Some(ui::ParticleAction::Fall(moves))
                }
            }));

            scheduler.put(Box::new(|ctx| {
                let state = ctx.state;
                let (gps, _, _) = blocks::scanning(&state.blocks);
                let dels = blocks::delete_points(&gps);
                if dels.is_empty() {
                    None
                } else {
                    Some(ui::ParticleAction::Delete(dels))
                }
            }));

            scheduler.put(Box::new(move |ctx| {
                let state = ctx.state;
                let (_, moves) = blocks::fall_scanning(&state.blocks);
                if moves.is_empty() {
                    None
                } else {
                    Some(ui::ParticleAction::Fall(moves))
                }
            }));
        }) as Box<dyn FnMut(_)>);

        let _ = &canvas
            .el
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();
    }

    {
        #[rustfmt::skip]
        h.render(h.node(
            &h.el("div"),
            vec![
                h.node(&h.el("div"), vec![&canvas.el])
            ],
        ));
    }
}
