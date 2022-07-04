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
use ui::Tag;

pub struct Ctx<'a> {
    pub state: &'a store::State,
    pub action_dispacher: ActionDispacher<'a>,
    pub canvas_ctx: &'a CanvasRenderingContext2d,
    pub se: &'a ui::SE<'a>,
}

#[wasm_bindgen]
pub async fn run(se: Vec<JsValue>) {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let se: Vec<AudioBuffer> = se
        .into_iter()
        .map(|jsv| JsCast::dyn_into::<AudioBuffer>(jsv).unwrap())
        .collect();

    let store = Rc::new(Store::create(store::State::create(), reducer));
    // store.subscribe(Box::new(|state| {}));
    let h = ui::HTML::new();
    let h = Rc::new(h);
    let canvas_el: HtmlCanvasElement = Tag::cast(Tag::name("canvas").unwrap());
    let canvas = ui::Canvas::create(canvas_el);
    let field = ui::Field::create(
        &canvas.el,
        blocks::BOARD_COL,
        blocks::BOARD_RAW,
        ui::WIDTH,
        ui::HEIGHT,
    );

    let actx = Rc::new(AudioContext::new().unwrap());
    let se = Rc::new(ui::SE {
        cancel: ui::Sound::from(se[0].clone(), Rc::clone(&actx)),
        change: ui::Sound::from(se[1].clone(), Rc::clone(&actx)),
        delete: ui::Sound::from(se[2].clone(), Rc::clone(&actx)),
        landing: ui::Sound::from(se[3].clone(), Rc::clone(&actx)),
        ok: ui::Sound::from(se[4].clone(), Rc::clone(&actx)),
    });

    let particle_render = Rc::new(RefCell::new(ui::ParticleRender::create()));
    let pr = Rc::clone(&particle_render);

    let scheduler = Rc::new(RefCell::new(
        ui::TaskScheduler::<ui::ParticleAction>::create(
            Box::new(move |task_name, task| {
                pr.borrow_mut()
                    .dispatch_with_name(task_name.to_string(), task);
            }),
            Box::new(move |task_name| {
                // end queue
            }),
        ),
    ));

    {
        let store = Rc::clone(&store);
        let prender = Rc::clone(&particle_render);
        let scheduler = Rc::clone(&scheduler);
        let se = Rc::clone(&se);
        FrameEngine::new(Box::new(move || {
            let state = &store.get_state();
            let action_dispacher = ActionDispacher::new(&store);
            let ctx = Ctx {
                se: se.as_ref(),
                state,
                canvas_ctx: &canvas.ctx,
                action_dispacher,
            };

            field.render(&ctx);
            prender.borrow_mut().render(&ctx);
            scheduler.borrow_mut().put(
                "fall",
                Box::new(move |ctx| {
                    let state = ctx.state;
                    let (_, moves) = blocks::fall_scanning(&state.blocks);
                    if moves.is_empty() {
                        None
                    } else {
                        Some(ui::ParticleAction::Fall(moves))
                    }
                }),
            );

            // TODO: やっつけ…
            scheduler.borrow_mut().put(
                "delete",
                Box::new(|ctx| {
                    let state = ctx.state;
                    let (gps, _, _) = blocks::scanning(&state.blocks);
                    let dels = blocks::delete_points(&gps);
                    let (_, moves) = blocks::fall_scanning(&state.blocks);
                    if dels.is_empty()
                        || !moves.is_empty()
                        || !state.falling_point.is_empty()
                        || !state.changing_point.is_empty()
                    {
                        None
                    } else {
                        Some(ui::ParticleAction::Delete(dels))
                    }
                }),
            );

            scheduler.borrow_mut().exec(&ctx);
        }))
        .start();
    }

    let handler = {
        let store = Rc::clone(&store);
        let scheduler = Rc::clone(&scheduler);
        let particle_render = Rc::clone(&particle_render);
        Closure::wrap(Box::new(move |e: MouseEvent| {
            let offset_x = e.offset_x();
            let offset_y = e.offset_y();
            let target = ui::Field::offset_to_point((offset_x, offset_y));
            let state = store.get_state();
            let mut qs = scheduler.borrow_mut();
            let mut pr = particle_render.borrow_mut();

            if blocks::is_over(&state.blocks) {
                dom::location().reload();
                return;
            }

            pr.dispatch(ui::ParticleAction::Touch(target));

            qs.put(
                "change",
                Box::new(move |ctx| {
                    let deleting_point = ctx.state.deleting_point.clone();
                    let falling_point = ctx.state.falling_point.clone();
                    let mut ignore_points: Vec<board::Point> = deleting_point.into_iter().collect();
                    let mut falling_points: Vec<board::Point> = falling_point.into_iter().collect();

                    ignore_points.append(&mut falling_points);

                    if let Some((a, b)) =
                        blocks::should_changed(&ctx.state.blocks, target, ignore_points)
                    {
                        Some(ui::ParticleAction::Change(a, b))
                    } else {
                        None
                    }
                }),
            );
        }) as Box<dyn FnMut(_)>)
    };

    let _ = &canvas
        .el
        .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
        .unwrap();
    handler.forget();

    {
        #[rustfmt::skip]
        h.render(h.node(
            Tag::name("div").as_ref(),
            vec![
                h.node(Tag::name("div").as_ref(), vec![&canvas.el])
            ],
        ));
    }
}
