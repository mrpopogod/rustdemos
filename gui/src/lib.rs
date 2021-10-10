pub trait Draw {  // traits effectively operate like interfaces
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // by using 'dyn Draw' instead of where 'T: Draw' means the components vector
                                        // can be heterogeneous; with 'where T: Draw' we will concretely define T for
                                        // a Screen instance at instantiation time and it could only take in one type
                                        // of component.  
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button named {} that is {}x{}", self.label, self.width, self.height);
    }
}
