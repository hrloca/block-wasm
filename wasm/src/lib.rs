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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub fn type_of<T>(_: T) -> () {
    println!("{}", std::any::type_name::<T>());
}

use store::*;
use tools::frame_engine::FrameEngine;
use tools::store::Store;

fn rcel<T>(data: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(data))
}

#[wasm_bindgen(start)]
pub async fn run() {
    let mut store = Store::create(create_state(), reducer);

    store.subscribe(Box::new(|state| {
        // console_log!("{:?}", state.locked);
    }));

    let store = rcel(store);
    let h = Rc::new(ui::HTML::new());
    let canvas = ui::Canvas::create(JsCast::dyn_into::<HtmlCanvasElement>(h.el("canvas")).unwrap())
        .initialize();

    let canvas = rcel(canvas);

    {
        let canvas2 = Rc::clone(&canvas);
        let store = Rc::clone(&store);
        let mut fe = FrameEngine::new(rcel(move || {
            let mut canvas = canvas2.as_ref().borrow_mut();
            let store = store.as_ref().borrow_mut();
            let state = store.get_state();

            canvas.render(state);
        }));

        fe.start();
    }

    // ---------------------
    let button = h.el("button");
    button.set_text_content(Some("exec"));

    let canvas1 = Rc::clone(&canvas);
    let store1 = Rc::clone(&store);
    let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
        let mut canvas = canvas1.borrow_mut();
        let mut store = store1.borrow_mut();
        let a = board::Point::of(1, 0);
        let b = board::Point::of(2, 0);

        let particle = ui::ChangeParticle::new(board::Point::of(1, 0), board::Point::of(2, 0));
        canvas.set_particles(particle);
        store.dispatch(Actions::Lock(vec![a, b]));

        console_log!("{:?}", canvas.get_particles());
    }) as Box<dyn FnMut(_)>);

    button
        .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
        .unwrap();

    handler.forget();
    // ---------------------

    // ---------------------
    let btn2 = h.el("button");
    btn2.set_text_content(Some("unlock"));
    let store2 = Rc::clone(&store);

    let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
        let a = board::Point::of(1, 0);
        let b = board::Point::of(2, 0);
        store2.borrow_mut().dispatch(Actions::Change(a, b));
        store2.borrow_mut().dispatch(Actions::UnLock(vec![a, b]));
    }) as Box<dyn FnMut(_)>);
    btn2.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
        .unwrap();

    handler.forget();
    // ---------------------

    let canvas = Rc::clone(&canvas);

    #[rustfmt::skip]
    h.render(
        h.node(&h.el("div"), vec![
            &button,
            &btn2,
            h.node(&h.el("div"), vec![
                canvas.borrow_mut().export(),
            ]) 
        ])
    );
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::dom::log(&format_args!($($t)*).to_string()))
}
