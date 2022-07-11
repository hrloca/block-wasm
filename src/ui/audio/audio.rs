use super::*;
use js_sys::{ArrayBuffer, Uint8Array};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::AudioBuffer;
use web_sys::AudioContext;
use web_sys::*;

pub struct Audio<'a> {
    path: Option<&'a str>,
    array_buffer: Rc<RefCell<Option<ArrayBuffer>>>,
    audio_buffer: Rc<RefCell<Option<AudioBuffer>>>,
    context: Rc<AudioContext>,
}

// TODO: safariで音出ない時ある
impl<'a> Audio<'a> {
    pub fn from(ab: AudioBuffer, ctx: Rc<AudioContext>) -> Self {
        Audio {
            path: None,
            array_buffer: Rc::new(RefCell::new(None)),
            audio_buffer: Rc::new(RefCell::new(Some(ab))),
            context: ctx,
        }
    }

    pub fn play(&self) {
        match self.audio_buffer.borrow().as_ref() {
            Some(audio_buffer) => {
                let source = self.context.create_buffer_source().unwrap();
                source.set_buffer(Some(audio_buffer));
                source.connect_with_audio_node(&self.context.destination());
                source.start();
            }
            None => {
                crate::log!("decoding");
                self.decode();
            }
        };
    }

    pub fn decode(&self) {
        if let Some(buf) = self.array_buffer.borrow().as_ref() {
            if buf.byte_length() <= 0 {
                return;
            }
            let success = {
                let abf = Rc::clone(&self.audio_buffer);
                Closure::wrap(Box::new(move |audio_buf: AudioBuffer| {
                    *abf.borrow_mut() = Some(audio_buf);
                }) as Box<dyn FnMut(_)>)
            };

            self.context
                .decode_audio_data_with_success_callback(&buf, success.as_ref().unchecked_ref());
            success.forget();
        }
    }

    fn load_src(&self) {
        match self.path {
            Some(path) => {
                let xhr = Rc::new(XmlHttpRequest::new().unwrap());
                xhr.set_response_type(XmlHttpRequestResponseType::Arraybuffer);
                xhr.open("GET", &path);
                let onload = {
                    let cxhr = Rc::clone(&xhr);
                    let bf = Rc::clone(&self.array_buffer);
                    Closure::wrap(Box::new(move |_: Event| {
                        let res: ArrayBuffer = cxhr.response().unwrap_throw().unchecked_into();
                        let raw: Vec<u8> = Uint8Array::new(&res).to_vec();
                        *bf.borrow_mut() = Some(res);
                    }) as Box<dyn FnMut(_)>)
                };
                xhr.set_onload(Some(onload.as_ref().unchecked_ref()));
                xhr.send();
                onload.forget();
            }
            None => {
                crate::log!("パスが設定されていません");
            }
        }
    }
}
