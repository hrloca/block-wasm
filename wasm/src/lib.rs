use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

pub mod blocks;
pub mod board;
pub mod dom;
pub mod libs;
pub mod store;
pub mod tools;
pub mod ui;

use libs::*;
use store::*;
use tools::frame_engine::FrameEngine;
use tools::store::Store;

pub fn rcel<T>(data: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(data))
}

pub fn type_of<T>(_: T) -> () {
    println!("{}", std::any::type_name::<T>());
}

fn delete(particle: &mut ui::ParticleDrawer, dels: Vec<board::Point>) {
    particle.draw(Box::new(ui::DeleteParticle::create(
        dels.clone(),
        Box::new(|action, dels| {
            action.will_delete(dels);
        }),
        Box::new(|action, dels| {
            action.delete(dels);
        }),
    )));
}

fn fall(particle: &mut ui::ParticleDrawer, from: board::Point, to: board::Point) {
    particle.draw(Box::new(ui::FallParticle::create(
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

fn change(particle: &mut ui::ParticleDrawer, a: board::Point, b: board::Point) {
    particle.draw(Box::new(ui::ChangeParticle::create(
        a,
        b,
        Box::new(|action, a, b| {
            action.will_change(a, b);
        }),
        Box::new(|action, a, b| {
            action.change(a, b);
        }),
    )));

    particle.draw(Box::new(ui::TouchParticle::create(a)));
}

#[wasm_bindgen(start)]
pub async fn run() {
    let mut store = Store::create(create_state(), reducer);
    store.subscribe(Box::new(|state| {
        log!("changing: {:?}", state.changing);
        log!("falling: {:?}", state.falling);
        log!("deleting: {:?}", state.deleting);
    }));

    let h = ui::HTML::new();
    let h = Rc::new(h);
    let canvas_el = JsCast::dyn_into::<HtmlCanvasElement>(h.el("canvas")).unwrap();
    let canvas = ui::Canvas::create(canvas_el);

    // 内部可変性 + 参照カウント
    let store = rcel(store);
    let particle = rcel(ui::ParticleDrawer::create());

    let field = ui::Field::create(
        &canvas.el,
        blocks::BOARD_COL,
        blocks::BOARD_RAW,
        ui::WIDTH,
        ui::HEIGHT,
    );

    {
        let store = Rc::clone(&store);
        let particle = Rc::clone(&particle);

        FrameEngine::new(rcel(move || {
            let mut store = store.borrow_mut();
            let state = store.get_state();
            let mut action = ActionDispacher::new(&mut (*store));
            let mut particle = particle.borrow_mut();

            field.render(&canvas.ctx, &state);
            particle.render(&canvas.ctx, &state, &mut action);
            particle.drop();
        }))
        .start();
    }

    {
        let store = Rc::clone(&store);
        let particle = Rc::clone(&particle);

        let handler = Closure::wrap(Box::new(move |e: MouseEvent| {
            let store = store.borrow_mut();
            let state = store.get_state();
            let mut particle = particle.borrow_mut();

            let offset_x = e.offset_x();
            let offset_y = e.offset_y();

            let a = ui::Field::point((offset_x, offset_y));

            if state.blocks.has(a) {
                let next = board::Point::of(a.x + 1, a.y);
                let b = state
                    .blocks
                    .within(next)
                    .or_else(|| Some(board::Point::of(a.x - 1, a.y)));

                if let Some(b) = b {
                    change(&mut particle, a, b);
                }
            }
        }) as Box<dyn FnMut(_)>);

        let _ = &canvas
            .el
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();
    }

    {
        // ---------------------
        let store_ = Rc::clone(&store);
        let particle_ = Rc::clone(&particle);

        let button = h.el("button");
        button.set_text_content(Some("fall"));
        let handler = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            let store = store_.borrow_mut();
            let state = store.get_state();
            let mut particle = particle_.borrow_mut();

            let (_, moves) = blocks::fall_scanning(&state.blocks);
            for (from, to) in moves {
                fall(&mut particle, from, to)
            }
        }) as Box<dyn FnMut(_)>);

        button
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();

        //-------------------

        let store_ = Rc::clone(&store);
        let particle_ = Rc::clone(&particle);

        let button2 = h.el("button");
        button2.set_text_content(Some("delete"));
        let handler = Closure::wrap(Box::new(move |_: MouseEvent| {
            let store = store_.borrow_mut();
            let state = store.get_state();
            let mut particle = particle_.borrow_mut();

            let (gps, _, _) = blocks::scanning(&state.blocks);
            let dels = blocks::delete_points(&gps);
            delete(&mut particle, dels);
        }) as Box<dyn FnMut(_)>);

        button2
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();

        h.render(h.node(
            &h.el("div"),
            vec![&button2, &button, h.node(&h.el("div"), vec![&canvas.el])],
        ));
    }
}
