use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
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

pub fn rcel<T>(data: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(data))
}

#[wasm_bindgen(start)]
pub async fn run() {
    let mut store = Store::create(create_state(), reducer);
    store.subscribe(Box::new(|state| {
        // log!("changing: {:?}", state.changing);
        // log!("falling: {:?}", state.falling);
        // log!("deleting: {:?}", state.deleting);
    }));

    let h = ui::HTML::new();
    let h = Rc::new(h);
    let canvas_el = JsCast::dyn_into::<HtmlCanvasElement>(h.el("canvas")).unwrap();
    let canvas = ui::Canvas::create(canvas_el);

    // 内部可変性 + 参照カウント
    let store = rcel(store);
    let particle = rcel(ui::ParticleDrawer::create());

    let field = ui::Field::create(
        &canvas.el,
        blocks::BOARD_COL,
        blocks::BOARD_RAW,
        ui::WIDTH,
        ui::HEIGHT,
    );

    {
        let store_ = Rc::clone(&store);
        let particle_ = Rc::clone(&particle);

        FrameEngine::new(rcel(move || {
            let state = store_.borrow().get_state();
            field.render(&canvas.ctx, &state);
            particle_.borrow_mut().render(&canvas.ctx, &state);
        }))
        .start();
    }

    {
        let store_ = Rc::clone(&store);
        let store__ = Rc::clone(&store);
        let particle_ = Rc::clone(&particle);

        let handler = Closure::wrap(Box::new(move |e: MouseEvent| {
            let state = store_.borrow().get_state();
            let mut particle = particle_.borrow_mut();

            let offset_x = e.offset_x();
            let offset_y = e.offset_y();

            let a = ui::Field::point((offset_x, offset_y));

            if state.blocks.has(a) {
                let b = state.blocks.right(a).or(state.blocks.left(a));
                let st = Rc::clone(&store__);
                let st2 = Rc::clone(&store__);
                if let Some((b, _)) = b {
                    let change_particle = Box::new(ui::ChangeParticle::create(
                        a,
                        b,
                        Box::new(move |a, b| {
                            let mut s = st.borrow_mut();
                            ActionDispacher::new(&mut (*s)).will_change(a, b);
                        }),
                        Box::new(move |a, b| {
                            let mut s = st2.borrow_mut();
                            ActionDispacher::new(&mut (*s)).change(a, b);
                        }),
                    ));

                    particle.draw(change_particle);
                    particle.draw(Box::new(ui::TouchParticle::create(a)));
                }
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
        let particle_ = Rc::clone(&particle);
        let store_ = Rc::clone(&store);
        let store__ = Rc::clone(&store);

        let button = h.el("button");
        button.set_text_content(Some("fall"));
        let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let state = store_.borrow().get_state();
            let mut particle = particle_.borrow_mut();

            let (_, moves) = blocks::fall_scanning(&state.blocks);
            for (from, to) in moves {
                let st = Rc::clone(&store__);
                let st2 = Rc::clone(&store__);
                let fall_particle = Box::new(ui::FallParticle::create(
                    from,
                    to,
                    Box::new(move |from, _| {
                        let mut s = st.borrow_mut();
                        ActionDispacher::new(&mut (*s)).will_fall(from);
                    }),
                    Box::new(move |from, to| {
                        let mut s = st2.borrow_mut();
                        ActionDispacher::new(&mut (*s)).fall(from, to);
                    }),
                ));

                particle.draw(fall_particle);
            }
        }) as Box<dyn FnMut(_)>);

        button
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();

        //-------------------

        let particle_ = Rc::clone(&particle);
        let store_ = Rc::clone(&store);
        let store__ = Rc::clone(&store);

        let button2 = h.el("button");
        button2.set_text_content(Some("delete"));
        let handler = Closure::wrap(Box::new(move |_: MouseEvent| {
            let state = store_.borrow().get_state();
            let mut particle = particle_.borrow_mut();

            let (gps, _, _) = blocks::scanning(&state.blocks);
            let dels = blocks::delete_points(&gps);
            let st = Rc::clone(&store__);
            let st2 = Rc::clone(&store__);

            let delete_particle = Box::new(ui::DeleteParticle::create(
                dels.clone(),
                Box::new(move |dels| {
                    let mut s = st.borrow_mut();
                    ActionDispacher::new(&mut (*s)).will_delete(dels);
                }),
                Box::new(move |dels| {
                    let mut s = st2.borrow_mut();
                    ActionDispacher::new(&mut (*s)).delete(dels);
                }),
            ));

            particle.draw(delete_particle);
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
