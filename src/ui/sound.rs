use web_sys::HtmlAudioElement;

pub struct Sound {
    src: HtmlAudioElement,
}

impl Sound {
    pub fn new(element: HtmlAudioElement) -> Self {
        Sound { src: element }
    }

    pub fn play_begining(&self) {
        self.src.set_current_time(0.);
        self.play();
    }

    pub fn play(&self) {
        self.src.play();
    }

    pub fn pause(&self) {
        self.src.pause();
    }
}

pub struct SE {
    pub cancel: Sound,
    pub change: Sound,
    pub delete: Sound,
    pub landing: Sound,
    pub ok: Sound,
}

pub struct BGM {
    pub one: Sound,
}
