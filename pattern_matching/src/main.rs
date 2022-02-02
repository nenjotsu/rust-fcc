#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    // matching patterns
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
    }

    let language = Language::English;

    match language {
        Language::English => println!("hello world"),
        Language::Spanish => println!("hola mundo"),
        Language::Russian => println!("Всем привет"),
        _ => println!("unsupported language!"), // will catch all patterns
    }

    // conditional if let expressions
    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("authorization status: {status}");
    } else if is_admin {
        println!("authorization status: admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("authorization status: priviledged");
        } else {
            println!("authorization status: basic");
        }
    } else {
        println!("authorization status: guest");
    }

    // while let conditional loops
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loops
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index)
    }

    // let statements
    let x = 5;
    // let PATTERN = EXPRESSION;
    let (x, y, z) = (1, 2, 3);

    // function parameters
    let point = (3, 5);
    print_coordinates(&point);

    patterns();
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("current location: ({}, {})", x, y);
}

#[allow(unused_variables)]
fn patterns() {
    // irrefutable
    let x = 5;

    // refutable
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    };

    // can only accept irrefutable patters:
    // function parameters
    // let statements
    // for loops
    refutable_sample();
}

#[allow(unused_variables)]
fn refutable_sample() {
    let x: Option<&str> = None;

    // this will error
    // let Some(x) = x;

    // this will error
    // if let x = 5 {
    //     println!("{}", x);
    // }
}
