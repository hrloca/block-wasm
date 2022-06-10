use crate::dom;
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
        let el = self.document.create_element(name).unwrap();
        el
    }

    pub fn node<'a>(&self, el: &'a Element, childs: Vec<&Element>) -> &'a Element {
        for child in childs.iter() {
            el.append_child(child).unwrap();
        }
        el
    }
}
