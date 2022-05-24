use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub mod blocks;
pub mod dom;
pub mod store;
pub mod system;
pub mod ui;
pub mod util;

#[wasm_bindgen(start)]
pub async fn run() {
    let document = dom::document();
    let body = dom::body();
    let stop = document.create_element("p").unwrap();
    let start = document.create_element("p").unwrap();
    stop.set_text_content(Some("STOP"));
    start.set_text_content(Some("START"));
    body.append_child(&stop).unwrap();
    body.append_child(&start).unwrap();

    let frame_engine = Rc::new(RefCell::new(system::FrameEngine::new(|| {
        dom::log("out: update");
    })));

    let a = Rc::clone(&frame_engine);
    let start_handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
        a.borrow_mut().start();
    }) as Box<dyn FnMut(_)>);

    start
        .add_event_listener_with_callback("mousedown", start_handler.as_ref().unchecked_ref())
        .unwrap();

    start_handler.forget();

    let b = Rc::clone(&frame_engine);
    let stop_handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
        b.borrow_mut().stop();
    }) as Box<dyn FnMut(_)>);

    stop.add_event_listener_with_callback("mousedown", stop_handler.as_ref().unchecked_ref())
        .unwrap();

    stop_handler.forget();
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (dom::log(&format_args!($($t)*).to_string()))
}
