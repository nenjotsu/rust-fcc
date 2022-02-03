mod gen;
mod new_type;
mod same_name;

pub trait Iterator {
    type Item; // associated typed, one concrete type per implementation

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

// impl Iterator for Counter {
//     type Item = u16;

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(0)
//     }
// }

fn main() {
    new_type::main();
}
