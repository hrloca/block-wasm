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

const CANVAS_NAME: &str = "board";

fn delete(canvas: &mut ui::Canvas, dels: Vec<board::Point>) {
    canvas.draw_particle(Box::new(ui::DeleteParticle::create(
        dels.clone(),
        Box::new(|action, dels| {
            action.will_delete(dels);
        }),
        Box::new(|action, dels| {
            action.delete(dels);
        }),
    )));
}

fn fall(canvas: &mut ui::Canvas, from: board::Point, to: board::Point) {
    canvas.draw_particle(Box::new(ui::FallParticle::create(
        from,
        to,
        Box::new(|action, from, _| {
            action.will_fall(from);
        }),
        Box::new(|action, from, to| {
            action.fall(from, to);
        }),
    )));
}

fn change(canvas: &mut ui::Canvas, a: board::Point, b: board::Point) {
    canvas.draw_particle(Box::new(ui::ChangeParticle::create(
        a,
        b,
        Box::new(|action, a, b| {
            action.will_change(a, b);
        }),
        Box::new(|action, a, b| {
            action.change(a, b);
        }),
    )));

    canvas.draw_particle(Box::new(ui::TouchParticle::create(a)));
}

#[wasm_bindgen(start)]
pub async fn run() {
    let h = ui::HTML::new();
    let h = Rc::new(h);
    let mut store = Store::create(create_state(), reducer);
    store.subscribe(Box::new(|state| {
        log!("changing: {:?}", state.changing);
        log!("falling: {:?}", state.falling);
        log!("deleting: {:?}", state.deleting);
    }));

    let store = rcel(store);
    let canvas_el = Rc::new(JsCast::dyn_into::<HtmlCanvasElement>(h.el("canvas")).unwrap());
    canvas_el.set_attribute("id", CANVAS_NAME).unwrap();
    let canvas_el_ = Rc::clone(&canvas_el);
    let canvas = ui::Canvas::create(canvas_el_);
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
            let store = store_.borrow_mut();
            let state = store.get_state();
            let mut canvas = canvas_.borrow_mut();

            let offset_x = e.offset_x();
            let offset_y = e.offset_y();

            let a = canvas.with_point((offset_x, offset_y));

            if state.blocks.has(a) {
                let next = board::Point::of(a.x + 1, a.y);
                let b = state
                    .blocks
                    .within(next)
                    .or_else(|| Some(board::Point::of(a.x - 1, a.y)));

                if let Some(b) = b {
                    change(&mut canvas, a, b);
                }
            }
        }) as Box<dyn FnMut(_)>);

        canvas_el
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();
    }

    {
        let store_ = Rc::clone(&store);
        let canvas_ = Rc::clone(&canvas);
        let handler = Closure::wrap(Box::new(move |_: MouseEvent| {
            let store = store_.borrow_mut();
            let state = store.get_state();
            let mut canvas = canvas_.borrow_mut();
            let (_, moves) = blocks::fall_scanning(&state.blocks);
            for (from, to) in moves {
                fall(&mut canvas, from, to)
            }
        }) as Box<dyn FnMut(_)>);

        canvas_el
            .add_event_listener_with_callback("fall", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();
    }

    {
        let store_ = Rc::clone(&store);
        let canvas_ = Rc::clone(&canvas);
        let handler = Closure::wrap(Box::new(move |_: MouseEvent| {
            let store = store_.borrow_mut();
            let state = store.get_state();
            let mut canvas = canvas_.borrow_mut();

            let (gps, _, _) = blocks::scanning(&state.blocks);
            let dels = blocks::delete_points(&gps);
            delete(&mut canvas, dels);
        }) as Box<dyn FnMut(_)>);

        canvas_el
            .add_event_listener_with_callback("delete", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();
    }

    {
        // ---------------------
        let button = h.el("button");
        button.set_text_content(Some("fall"));
        let event = h.ev("fall");
        let ht = Rc::clone(&h);
        let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let canvas_el = ht.get_by_id(CANVAS_NAME);
            canvas_el.dispatch_event(&event).unwrap();
        }) as Box<dyn FnMut(_)>);

        button
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();

        //-------------------

        let button2 = h.el("button");
        button2.set_text_content(Some("delete"));
        let ht = Rc::clone(&h);
        let delete_event = h.ev("delete");
        let handler = Closure::wrap(Box::new(move |_: MouseEvent| {
            let canvas_el = ht.get_by_id(CANVAS_NAME);
            canvas_el.dispatch_event(&delete_event).unwrap();
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
        .map(|c| match c {
            'x' => format!("{:x}", (rng.gen::<f64>() * 16.0).floor() as usize),
            'y' => format!("{:x}", (rng.gen::<f64>() * 4.0).floor() as usize + 8),
            _ => c.to_string(),
        })
        .collect()
}
