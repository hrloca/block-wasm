use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub mod blocks;
pub mod board;
pub mod dom;
pub mod store;
pub mod tools;
pub mod ui;

pub fn type_of<T>(_: T) -> () {
    println!("{}", std::any::type_name::<T>());
}

#[wasm_bindgen(start)]
pub async fn run() {
    let store = Rc::new(RefCell::new(tools::store::Store::create(
        store::create_state(),
        store::reducer,
    )));

    let ui = Rc::new(RefCell::new(ui::UI::new()));

    {
        let ui = Rc::clone(&ui);
        let store = Rc::clone(&store);

        let mut fe = tools::frame_engine::FrameEngine::new(Rc::new(RefCell::new(move || {
            let store = store.as_ref().borrow_mut();
            let ui = ui.as_ref().borrow_mut();
            let state = store.get_state();

            ui.draw(state);
        })));

        fe.start();
    }

    {
        let ui = ui.borrow_mut();
        let store = Rc::clone(&store);
        let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            store.borrow_mut().dispatch(store::Actions::Change(
                board::Point::of(1, 0),
                board::Point::of(2, 0),
            ));
        }) as Box<dyn FnMut(_)>);

        let button = ui.el("button", None);
        button.set_text_content(Some("exec"));
        button
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();

        ui.render(ui.el("div", Some(vec![button])));
        ui.set_canvas();

        handler.forget();
    }
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (dom::log(&format_args!($($t)*).to_string()))
}
