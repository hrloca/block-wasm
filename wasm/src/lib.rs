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

pub fn type_of<T>(_: T) -> () {
    println!("{}", std::any::type_name::<T>());
}

use store::*;
use tools::frame_engine::FrameEngine;
use tools::store::Store;

fn cel<T>(data: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(data))
}

fn rc<T>(data: T) -> Rc<T> {
    Rc::new(data)
}

#[wasm_bindgen(start)]
pub async fn run() {
    let store = cel(Store::create(create_state(), reducer));
    let h = rc(ui::HTML::new());
    let canvas = rc(ui::Canvas::create(
        JsCast::dyn_into::<HtmlCanvasElement>(h.el("canvas")).unwrap(),
    )
    .initialize());

    {
        let canvas = Rc::clone(&canvas);
        let store = Rc::clone(&store);
        let mut fe = FrameEngine::new(cel(move || {
            let store = store.as_ref().borrow_mut();
            let state = store.get_state();
            canvas.render(state);
        }));

        fe.start();
    }

    let button = h.el("button");
    button.set_text_content(Some("exec"));

    let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
        store.borrow_mut().dispatch(Actions::Change(
            board::Point::of(1, 0),
            board::Point::of(2, 0),
        ));
    }) as Box<dyn FnMut(_)>);

    button
        .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
        .unwrap();

    handler.forget();

    let root = h.el("div");
    let wrapper = h.el("div");

    #[rustfmt::skip]
    h.render(
        h.node(&root, vec![
            &button,
            h.node(&wrapper, vec![
                canvas.export(),
            ]) 
        ])
    );
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (dom::log(&format_args!($($t)*).to_string()))
}
