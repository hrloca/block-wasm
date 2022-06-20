use crate::dom;
use crate::log;
use web_sys::*;

pub struct HTML {
    document: Document,
    window: Window,
    body: HtmlElement,
}

impl HTML {
    pub fn new() -> HTML {
        let window = dom::window();
        let document = dom::document();
        let body = dom::body();

        HTML {
            window,
            document,
            body,
        }
    }

    pub fn render(&self, el: &Element) {
        self.body.append_child(el).unwrap();
    }

    pub fn el(&self, name: &str) -> Element {
        self.document.create_element(name).unwrap()
    }

    pub fn get_by_id(&self, name: &str) -> Element {
        self.document.get_element_by_id(name).unwrap()
    }

    pub fn ev(&self, name: &str) -> Event {
        let ev = self.document.create_event("Event").unwrap();
        ev.init_event(name);
        ev
    }

    pub fn node<'a>(&self, el: &'a Element, childs: Vec<&Element>) -> &'a Element {
        for child in childs.iter() {
            el.append_child(child).unwrap();
        }
        el
    }
}
