use crate::dom;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub struct FrameEngine {
    timer_id: Rc<RefCell<Option<u32>>>,
    is_active: bool,
    update: Rc<RefCell<dyn Fn() -> ()>>,
}

impl FrameEngine {
    pub fn new(updater: Rc<RefCell<dyn Fn() -> ()>>) -> FrameEngine {
        FrameEngine {
            timer_id: Rc::new(RefCell::new(None)),
            is_active: false,
            update: updater,
        }
    }

    pub fn step(&mut self) {
        if self.is_active {
            let closure = Rc::new(RefCell::new(None));
            let cloned_closure = closure.clone();

            let timer = self.timer_id.clone();
            let timer2 = self.timer_id.clone();
            let update = Rc::clone(&self.update);

            *cloned_closure.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                update.borrow_mut()();
                *timer.borrow_mut() = Some(dom::requestAnimationFrame(
                    closure.borrow().as_ref().unwrap(),
                ));
            }) as Box<dyn FnMut()>));

            *timer2.borrow_mut() = Some(dom::requestAnimationFrame(
                cloned_closure.borrow().as_ref().unwrap(),
            ));
        }
    }

    pub fn start(&mut self) {
        self.is_active = true;
        self.step()
    }

    pub fn stop(&mut self) {
        if self.is_active {
            self.is_active = false;
            dom::cancelAnimationFrame(*self.timer_id.borrow().as_ref().unwrap());
        }
    }
}
