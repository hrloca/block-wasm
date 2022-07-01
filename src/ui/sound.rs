use super::*;
use web_sys::HtmlAudioElement;

pub struct Sound<'a> {
    src: &'a str,
}

impl<'a> Sound<'a> {
    pub fn new(src: &'a str) -> Self {
        Sound { src }
    }

    pub fn play(&self) {
        // TODO: Web Audio APIを使う
        // let sound: HtmlAudioElement = Tag::cast(Tag::name("audio").unwrap());
        // sound.set_src(&self.src);
        // sound.play();
    }
}

pub struct SE<'a> {
    pub cancel: Sound<'a>,
    pub change: Sound<'a>,
    pub delete: Sound<'a>,
    pub landing: Sound<'a>,
    pub ok: Sound<'a>,
}

pub struct BGM<'a> {
    pub one: Sound<'a>,
}
