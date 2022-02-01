pub trait Draw {
    fn draw(&self);
}

// trait objects
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32;
    pub height: u32;
    pub label: String;
}

impl Draw for Button {
    fn draw(&self) {
        unimplemented!() // TODO draw button
    }
}

// generics & trait bound
// downside is limited in one type
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }
