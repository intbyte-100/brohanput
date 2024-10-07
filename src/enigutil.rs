use enigo::{Direction, Enigo, Key, Keyboard, Settings};



static mut ENIGO: Option<Enigo> = None;

pub(crate) struct BrKeyboard<'a> {
    enigo: &'a mut Enigo,
}

impl<'a> BrKeyboard<'a> {
    pub(crate) fn get() -> Self {
        unsafe {
            ENIGO
                .is_none()
                .then(|| ENIGO = Some(Enigo::new(&Settings::default()).unwrap()));
            Self {
                enigo: (ENIGO.as_mut().unwrap()),
            }
        }
    }

    fn recreate() -> Self {
        unsafe {
            ENIGO = None;
            Self::get()
        }
    }

    pub(crate) fn text(mut self, text: &String) {
        self.use_enigo(|enigo| enigo.text(text));
    }

    pub(crate) fn backspace(mut self) {
        self.use_enigo(|enigo| enigo.key(Key::Backspace, Direction::Click));
    }

    pub(crate) fn enter(mut self) {
        self.use_enigo(|enigo| enigo.key(Key::Return, Direction::Click));
    }

    pub(crate) fn tab(mut self) {
        self.use_enigo(|enigo| enigo.key(Key::Tab, Direction::Click));
    }

    pub(crate) fn arrow(mut self, arrow: &String) {
        let arrow = match arrow.as_str() {
            "up" => Key::UpArrow,
            "down" => Key::DownArrow,
            "left" => Key::LeftArrow,
            "right" => Key::RightArrow,
            _ => panic!("Invalid arrow")
        };

        self.use_enigo(|enigo| enigo.key(arrow, Direction::Click));
    }

    pub(crate) fn shift(mut self, arrow: &String) {
        let state = match arrow.as_str() {
            "press" => Direction::Press,
            "release" => Direction::Release,
            _ => panic!()
        };  

        self.use_enigo(|enigo| enigo.key(Key::Shift, state));
    }

    pub(crate) fn control(mut self, state: &String) {
        let state = match state.as_str() {
            "press" => Direction::Press,
            "release" => Direction::Release,
            _ => panic!()
        };  

        self.use_enigo(|enigo| enigo.key(Key::LControl, state));
    }


    fn use_enigo<T, A, B>(&mut self, func: T)
    where
        T: Fn(&mut Enigo) -> Result<A, B>,
        B: std::fmt::Debug,
    {
        func(self.enigo).is_err().then(|| {
            func(Self::recreate().enigo).unwrap();
        });
    }
}
