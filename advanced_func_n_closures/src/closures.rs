pub fn main() {
    let list_of_numbers = vec![1, 2, 3];
    // let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    // ToString::to_string is a Function Pointers
    // Closures & Function Pointers are both compiled down as the same output

    println!("{:?}", list_of_strings);
}

pub fn func_pointer_tuple() {
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);
}

pub fn returns_closures(a: i32) -> Box<dyn Fn(i32) -> i32> {
    // Box:new(move |b| a - b)
    if a > 0 {
        Box:new(move |b| a + b)
    } else {
        Box:new(move |b| a - b)
    }
}