use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    pub fn requestAnimationFrame(closure: &Closure<dyn FnMut()>) -> u32;
    pub fn cancelAnimationFrame(id: u32);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub fn window() -> web_sys::Window {
    web_sys::window().unwrap()
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap()
}

pub fn cancel_animation_frame(id: i32) {
    window().cancel_animation_frame(id).unwrap()
}

pub fn document() -> web_sys::Document {
    window().document().unwrap()
}

pub fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

pub struct AnimationFrameHandle {
    animation_id: i32,
    _closure: Closure<dyn FnMut()>,
}

impl Drop for AnimationFrameHandle {
    fn drop(&mut self) {
        cancel_animation_frame(self.animation_id);
    }
}

pub fn looping() {
    let closure = Rc::new(RefCell::new(None));
    let cloned_closure = closure.clone();
    let mut _id: Option<i32> = None;
    *cloned_closure.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        _id = Some(request_animation_frame(closure.borrow().as_ref().unwrap()))
    }) as Box<dyn FnMut()>));

    Some(request_animation_frame(
        cloned_closure.borrow().as_ref().unwrap(),
    ));
}

pub fn run() {
    let closure = Rc::new(RefCell::new(None));
    let cloned_closure = closure.clone();

    let mut i = 0;
    let mut _id: Option<i32> = None;

    *cloned_closure.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i > 300 {
            log("done!");
            cancel_animation_frame(_id.unwrap());
            return;
        }

        i += 1;

        log(&(i.to_string()));

        _id = Some(request_animation_frame(closure.borrow().as_ref().unwrap()));
    }) as Box<dyn FnMut()>));

    _id = Some(request_animation_frame(
        cloned_closure.borrow().as_ref().unwrap(),
    ));
}
