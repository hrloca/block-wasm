use rand::prelude::*;
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
    store.subscribe(Box::new(|state| {
        log!("changing: {:?}", state.changing);
        log!("falling: {:?}", state.falling);
        log!("deleting: {:?}", state.deleting);
    }));

    let store = rcel(store);
    let canvas_el = JsCast::dyn_into::<HtmlCanvasElement>(h.el("canvas")).unwrap();
    let canvas = ui::Canvas::create(&canvas_el);
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
        let store_ = Rc::clone(&store);
        let canvas_ = Rc::clone(&canvas);
        let handler = Closure::wrap(Box::new(move |e: MouseEvent| {
            let mut store = store_.borrow_mut();
            let state = store.get_state();
            let mut action = ActionDispacher::new(&mut (*store));
            let mut canvas = canvas_.borrow_mut();

            let offset_x = e.offset_x();
            let offset_y = e.offset_y();

            let a = canvas.with_point((offset_x, offset_y));

            if state.blocks.has(a) {
                let next = board::Point::of(a.x + 1, a.y);
                let or_prev = board::Point::of(a.x - 1, a.y);
                let next = state.blocks.within(next).or(state.blocks.within(or_prev));

                if let Some(next) = next {
                    action.will_change(a, next);
                    canvas.draw_particle(Box::new(ui::ChangeParticle::create(
                        a,
                        next,
                        Box::new(|action, a, b| {
                            action.change(a, b);
                        }),
                    )));

                    canvas
                        .draw_particle(Box::new(ui::TouchParticle::create(a, Box::new(|_, _| {}))));
                }
            }
        }) as Box<dyn FnMut(_)>);

        canvas_el
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();
        handler.forget();
    }

    {
        // ---------------------
        let store_ = Rc::clone(&store);
        let canvas_ = Rc::clone(&canvas);

        let button = h.el("button");
        button.set_text_content(Some("fall"));
        let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let mut store = store_.borrow_mut();
            let state = store.get_state();
            let mut action = ActionDispacher::new(&mut (*store));
            let mut canvas = canvas_.borrow_mut();

            let (_, moves) = blocks::fall_scanning(&state.blocks);

            for (from, to) in moves {
                action.will_fall(from);
                canvas.draw_particle(Box::new(ui::FallParticle::create(
                    from,
                    to,
                    Box::new(|action, from, to| {
                        action.fall(from, to);
                    }),
                )));
            }
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
        let handler = Closure::wrap(Box::new(move |_: MouseEvent| {
            let mut store = store_.borrow_mut();
            let state = store.get_state();
            let mut action = ActionDispacher::new(&mut (*store));
            let mut canvas = canvas_.borrow_mut();

            let (gps, _, _) = blocks::scanning(&state.blocks);
            let dels = blocks::delete_points(&gps);

            action.will_delete(dels.clone());
            canvas.draw_particle(Box::new(ui::DeleteParticle::create(
                dels.clone(),
                Box::new(|action, dels| {
                    action.delete(dels.clone());
                }),
            )));
        }) as Box<dyn FnMut(_)>);

        button2
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();

        h.render(h.node(
            &h.el("div"),
            vec![&button2, &button, h.node(&h.el("div"), vec![&canvas_el])],
        ));
    }
}
pub fn uuid() -> String {
    let mut rng = rand::thread_rng();
    "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx"
        .chars()
        .map(|c| {
            if c == 'x' {
                format!("{:x}", (rng.gen::<f64>() * 16.0).floor() as usize)
            } else if c == 'y' {
                format!("{:x}", (rng.gen::<f64>() * 4.0).floor() as usize + 8)
            } else {
                c.to_string()
            }
        })
        .collect()
}
