use crate::dom;
use wasm_bindgen::JsCast;
use web_sys::*;

pub struct Tag {
    element: Element,
}

impl Tag {
    pub fn name(name: &str) -> Self {
        let el = dom::document().create_element(name).unwrap();
        Tag { element: el }
    }

    pub fn class(&self, name: &str) -> &Self {
        self.element.set_class_name(name);
        &self
    }

    pub fn unwrap(self) -> Element {
        self.element
    }

    pub fn as_ref(&self) -> &Element {
        &self.element
    }

    pub fn cast<T: JsCast>(el: Element) -> T {
        JsCast::dyn_into::<T>(el).unwrap()
    }
}

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

    pub fn get_by_id(&self, name: &str) -> Element {
        self.document.get_element_by_id(name).unwrap()
    }

    pub fn query_selector(&self, selectors: &str) -> Element {
        self.document.query_selector(selectors).unwrap().unwrap()
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
