use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

pub mod blocks;
pub mod board;
pub mod dom;
pub mod store;
pub mod tools;
pub mod ui;

use store::*;
use tools::frame_engine::FrameEngine;
use tools::store::Store;

pub fn rcel<T>(data: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(data))
}

pub fn type_of<T>(_: T) -> () {
    println!("{}", std::any::type_name::<T>());
}

#[wasm_bindgen(start)]
pub async fn run() {
    let h = ui::HTML::new();
    let mut store = Store::create(create_state(), reducer);
    store.subscribe(Box::new(|state| log!("{:?}", state.locked)));
    let store = rcel(store);
    let canvas = ui::Canvas::create(JsCast::dyn_into::<HtmlCanvasElement>(h.el("canvas")).unwrap())
        .initialize();
    let canvas = rcel(canvas);

    {
        let store_ = Rc::clone(&store);
        let canvas_ = Rc::clone(&canvas);

        FrameEngine::new(rcel(move || {
            let mut store = store_.borrow_mut();
            let state = store.get_state();
            let mut action = ActionDispacher::new(&mut (*store));
            let mut canvas = canvas_.borrow_mut();
            canvas.render(&state, &mut action);
        }))
        .start();
    }

    {
        // ---------------------
        let store_ = Rc::clone(&store);
        let canvas_ = Rc::clone(&canvas);

        let button = h.el("button");
        button.set_text_content(Some("fall"));
        let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let mut store = store_.borrow_mut();
            let mut action = ActionDispacher::new(&mut (*store));
            let mut canvas = canvas_.borrow_mut();

            let from = board::Point::of(1, 0);
            let to = board::Point::of(1, 3);

            action.lock(vec![from]);
            canvas.draw_particle(Box::new(ui::FallParticle::create(
                from,
                to,
                Box::new(|action, from, to| {
                    action.move_(from, to);
                }),
            )));
        }) as Box<dyn FnMut(_)>);

        button
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();

        //-------------------

        let store_ = Rc::clone(&store);
        let canvas_ = Rc::clone(&canvas);
        let button2 = h.el("button");
        button2.set_text_content(Some("delete"));
        let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let mut store = store_.borrow_mut();
            let mut action = ActionDispacher::new(&mut (*store));
            let mut canvas = canvas_.borrow_mut();

            let a = board::Point::of(1, 1);
            let b = board::Point::of(1, 2);
            let c = board::Point::of(1, 3);
            let d = board::Point::of(0, 3);

            action.lock(vec![a, b, c, d]);
            canvas.draw_particle(Box::new(ui::DeleteParticle::create(
                vec![a, b, c, d],
                Box::new(|action, dels| {
                    action.delete(dels.clone());
                }),
            )));
        }) as Box<dyn FnMut(_)>);

        button2
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();

        let store_ = Rc::clone(&store);
        let canvas_ = Rc::clone(&canvas);
        let button3 = h.el("button");
        button3.set_text_content(Some("change"));
        let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let mut store = store_.borrow_mut();
            let mut action = ActionDispacher::new(&mut (*store));
            let mut canvas = canvas_.borrow_mut();

            let a = board::Point::of(1, 3);
            let b = board::Point::of(2, 3);

            action.lock(vec![a, b]);
            canvas.draw_particle(Box::new(ui::ChangeParticle::create(
                a,
                b,
                Box::new(|action, a, b| {
                    action.change(a, b);
                }),
            )));
        }) as Box<dyn FnMut(_)>);

        button3
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();

        let canvas_ = Rc::clone(&canvas);

        h.render(h.node(
            &h.el("div"),
            vec![
                &button3,
                &button2,
                &button,
                h.node(&h.el("div"), vec![canvas_.borrow_mut().export()]),
            ],
        ));
    }
}
