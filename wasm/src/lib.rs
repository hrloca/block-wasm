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
use rand::prelude::*;
use store::*;
use tools::frame_engine::FrameEngine;
use tools::store::Store;

pub struct Ctx<'a> {
    pub state: &'a store::State,
    pub action_dispacher: ActionDispacher<'a>,
    pub canvas_ctx: &'a CanvasRenderingContext2d,
}

#[wasm_bindgen(start)]
pub async fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let store = Rc::new(Store::create(create_state(), reducer));
    store.subscribe(Box::new(|state| {
        log!("{:?}", state.complete_tasks);
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

    let ts = Rc::new(RefCell::new(ui::TaskScheduler::<ui::Particles>::create(
        Box::new(move |task_id, task| {
            pr.borrow_mut().dispatch_with(task_id, task);
        }),
    )));

    {
        let store = Rc::clone(&store);
        let pr = Rc::clone(&particle_render);
        let ts = Rc::clone(&ts);

        FrameEngine::new(Box::new(move || {
            let state = &store.get_state();
            let action_dispacher = ActionDispacher::new(&store);
            let ctx = Ctx {
                state,
                canvas_ctx: &canvas.ctx,
                action_dispacher,
            };

            field.render(&ctx);
            pr.borrow_mut().render(&ctx);
            ts.borrow_mut().exec(&state.complete_tasks, &ctx);
        }))
        .start();
    }

    {
        let store = Rc::clone(&store);
        let ts = Rc::clone(&ts);
        let handler = Closure::wrap(Box::new(move |e: MouseEvent| {
            let offset_x = e.offset_x();
            let offset_y = e.offset_y();
            let a = ui::Field::point((offset_x, offset_y));
            let state = store.get_state();

            if state.blocks.has(a) {
                let b = state.blocks.right(a).or(state.blocks.left(a));
                if let Some((b, _)) = b {
                    let first = ts
                        .borrow_mut()
                        .register(Box::new(move |_| Some(ui::Particles::Change(a, b))));

                    let second = ts.borrow_mut().then(
                        first,
                        Box::new(move |_| {
                            let mut rng = rand::thread_rng();
                            if rng.gen::<f64>() > 0.5 {
                                Some(ui::Particles::Change(
                                    board::Point::of(0, 0),
                                    board::Point::of(1, 0),
                                ))
                            } else {
                                None
                            }
                        }),
                    );

                    let third = ts
                        .borrow_mut()
                        .then(second, Box::new(move |_| Some(ui::Particles::Change(a, b))));

                    ts.borrow_mut().jump(third, second);

                    store.dispatch(store::Actions::AddCompleteTask(ts.borrow_mut().run(first)));
                };
            }
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
