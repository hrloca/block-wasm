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
    let store = rcel(Store::create(create_state(), reducer));
    let canvas = rcel(
        ui::Canvas::create(JsCast::dyn_into::<HtmlCanvasElement>(h.el("canvas")).unwrap())
            .initialize(),
    );

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
        button.set_text_content(Some("exec"));
        let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let mut store = store_.borrow_mut();
            let mut action = ActionDispacher::new(&mut (*store));
            let mut canvas = canvas_.borrow_mut();

            let a = board::Point::of(1, 0);
            let b = board::Point::of(2, 0);

            action.lock(vec![a, b]);
            canvas.draw_particle(ui::ChangeParticle::new(
                a,
                b,
                Box::new(|action, from, to| {
                    action.change(from, to);
                    action.unlock(vec![from, to]);
                }),
            ));
        }) as Box<dyn FnMut(_)>);

        button
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();

        let canvas_ = Rc::clone(&canvas);

        h.render(h.node(
            &h.el("div"),
            vec![
                &button,
                h.node(&h.el("div"), vec![canvas_.borrow_mut().export()]),
            ],
        ));
    }
}
