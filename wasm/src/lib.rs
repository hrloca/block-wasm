use std::panic;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

use rand::prelude::*;

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

fn delete(store: &Rc<Store<store::State, store::Actions>>) -> Box<ui::DeleteParticle> {
    let state = store.get_state();

    let (gps, _, _) = blocks::scanning(&state.blocks);
    let dels = blocks::delete_points(&gps);

    let s1 = Rc::clone(&store);
    let s2 = Rc::clone(&store);
    Box::new(ui::DeleteParticle::create(
        dels.clone(),
        Box::new(move |dels| {
            ActionDispacher::new(&s1).will_delete(dels);
        }),
        Box::new(move |dels| {
            ActionDispacher::new(&s2).delete(dels);
        }),
    ))
}

fn fall(
    store: &Rc<Store<store::State, store::Actions>>,
    from: board::Point,
    to: board::Point,
) -> Box<ui::FallParticle> {
    let s1 = Rc::clone(&store);
    let s2 = Rc::clone(&store);
    Box::new(ui::FallParticle::create(
        from,
        to,
        Box::new(move |from, _| {
            ActionDispacher::new(&s1).will_fall(from);
        }),
        Box::new(move |from, to| {
            ActionDispacher::new(&s2).fall(from, to);
        }),
    ))
}

#[wasm_bindgen(start)]
pub async fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let store = Rc::new(Store::create(create_state(), reducer));
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

    {
        let store = Rc::clone(&store);
        FrameEngine::new(Box::new(move || {
            let state = store.get_state();
            field.render(&canvas.ctx, &state);
            ui::particle_render(&canvas.ctx, &state);
        }))
        .start();
    }

    {
        let store = Rc::clone(&store);
        let handler = Closure::wrap(Box::new(move |e: MouseEvent| {
            let offset_x = e.offset_x();
            let offset_y = e.offset_y();
            let a = ui::Field::point((offset_x, offset_y));
            let state = store.get_state();
            if state.blocks.has(a) {
                let b = state.blocks.right(a).or(state.blocks.left(a));
                if let Some((b, _)) = b {
                    let s1 = Rc::clone(&store);
                    let s2 = Rc::clone(&store);
                    let change_particle = Box::new(ui::ChangeParticle::create(
                        a,
                        b,
                        Box::new(move |a, b| {
                            s1.dispatch(Actions::Changing(a, b));
                        }),
                        Box::new(move |a, b| {
                            s2.dispatch(Actions::Change(a, b));
                            let change_particle = Box::new(ui::ChangeParticle::create(
                                a,
                                b,
                                Box::new(move |a, b| {}),
                                Box::new(move |a, b| {}),
                            ));
                            ui::particle_draw(change_particle);
                        }),
                    ));
                    ui::particle_draw(change_particle);
                    ui::particle_draw(Box::new(ui::TouchParticle::create(a)));
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
        button.set_text_content(Some("fall"));
        let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let state = store_.get_state();
            let (_, moves) = blocks::fall_scanning(&state.blocks);
            for (from, to) in moves {
                ui::particle_draw(fall(&store_, from, to));
            }
        }) as Box<dyn FnMut(_)>);

        button
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();

        //-------------------

        let store_ = Rc::clone(&store);
        let button2 = h.el("button");
        button2.set_text_content(Some("delete"));
        let handler = Closure::wrap(Box::new(move |_: MouseEvent| {
            ui::particle_draw(delete(&store_));
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
