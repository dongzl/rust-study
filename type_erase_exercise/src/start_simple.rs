
pub struct App {
    handlers: Vec<Box<dyn Fn() -> ()>>,
}

impl App {
    pub fn new() -> Self {
        Self { handlers: vec![] }
    }
    pub fn handler(mut self, f: impl Fn() -> () + 'static) -> Self {
        self.handlers.push(Box::new(f));
        self
    }
    pub fn dispatch(&self) {
        for handler in self.handlers.iter() {
            (handler)()
        }
    }
}