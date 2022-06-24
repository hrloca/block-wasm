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

#[wasm_bindgen(start)]
pub async fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let store = Rc::new(Store::create(create_state(), reducer));
    store.subscribe(Box::new(|state| {
        log!("{:?}", state.complete_tasks);
        // log!("{:?}", state.changing);
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
    let particle_scheduler = Rc::new(RefCell::new(ui::TaskScheduler::create()));

    {
        let store = Rc::clone(&store);
        let pr = Rc::clone(&particle_render);
        let ps = Rc::clone(&particle_scheduler);

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
            ps.borrow_mut().exec(&state.complete_tasks, &ctx);
        }))
        .start();
    }

    {
        let store = Rc::clone(&store);
        let pr = Rc::clone(&particle_render);
        let ps = Rc::clone(&particle_scheduler);
        let handler = Closure::wrap(Box::new(move |e: MouseEvent| {
            let offset_x = e.offset_x();
            let offset_y = e.offset_y();
            let a = ui::Field::point((offset_x, offset_y));
            let state = store.get_state();

            if state.blocks.has(a) {
                let b = state.blocks.right(a).or(state.blocks.left(a));
                if let Some((b, _)) = b {
                    let s1 = Rc::clone(&store);
                    store.dispatch(Actions::Changing(a, b));
                    let change = pr.borrow_mut().dispatch(ui::Particles::Change(a, b));

                    ps.borrow_mut().then(
                        change,
                        Box::new(move |_| {
                            s1.dispatch(Actions::Change(a, b));
                        }),
                    );

                    pr.borrow_mut().dispatch(ui::Particles::Touch(a));
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
        // ---------------------
        let store_ = Rc::clone(&store);
        let button = h.el("button");
        let pr_ = Rc::clone(&particle_render);
        let ps_ = Rc::clone(&particle_scheduler);
        button.set_text_content(Some("fall"));
        let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let state = store_.get_state();
            let (_, moves) = blocks::fall_scanning(&state.blocks);
            for (from, to) in moves {
                let st1 = Rc::clone(&store_);
                store_.dispatch(Actions::Falling(from));
                let fall = pr_.borrow_mut().dispatch(ui::Particles::Fall(from, to));
                ps_.borrow_mut().then(
                    fall,
                    Box::new(move |_| {
                        st1.dispatch(Actions::Fall(from, to));
                    }),
                );
            }
        }) as Box<dyn FnMut(_)>);

        button
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();

        //-------------------

        let store_ = Rc::clone(&store);
        let button2 = h.el("button");
        let pr_ = Rc::clone(&particle_render);
        let ps_ = Rc::clone(&particle_scheduler);
        button2.set_text_content(Some("delete"));
        let handler = Closure::wrap(Box::new(move |_: MouseEvent| {
            let state = store_.get_state();
            let (gps, _, _) = blocks::scanning(&state.blocks);
            let dels = blocks::delete_points(&gps);

            store.dispatch(Actions::Deleting(dels.clone()));
            let delete = pr_
                .borrow_mut()
                .dispatch(ui::Particles::Delete(dels.clone()));
            let st1 = Rc::clone(&store_);

            ps_.borrow_mut().then(
                delete,
                Box::new(move |_| {
                    st1.dispatch(Actions::Delete(dels.clone()));
                }),
            );
        }) as Box<dyn FnMut(_)>);

        button2
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();

        #[rustfmt::skip]
        h.render(h.node(
            &h.el("div"),
            vec![
                &button2,
                &button,
                h.node(&h.el("div"), vec![&canvas.el])
            ],
        ));
    }
}
