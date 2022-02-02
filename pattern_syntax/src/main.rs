#![allow(dead_code)]
#![allow(unused_variables)]

fn matching_literals() {
    // matching literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_named_variables() {
    // matching named variables
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("got 50"),
        Some(y) => println!("matched, y = {:?}", y),
        _ => println!("default case, x = {:?}", x),
    }
}

fn multiple_patterns() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_ranges_of_values() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destructuring_to_break_apart_values_p1() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

fn destructuring_to_break_apart_values_p2() {
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => {
            println!("on the x axis at {}", x);
        }
        Point { x: 0, y } => {
            println!("on the y axis at {}", y);
        }
        Point { x, y } => {
            println!("on neither axis: ({}, {})", x, y);
        }
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn destructuring_to_break_apart_values_p3() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color: red {}, green {}, and blue {}", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color: hue {}, saturation {}, and value {}", h, s, v);
        }
        Message::Quit => {
            println!("Quit");
        }
        Message::Move { x, y } => {
            println!("move to x: {} y: {}", x, y);
        }
        Message::Write(text) => {
            println!("text message: {}", text);
        }
    }
}

fn destructuring_to_break_apart_values_p4() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

fn foo(_: i32, y: i32) {
    println!("this code only uses the y parameter: {}", y);
}

fn ignoring_values_in_a_pattern_p1() {
    foo(3, 4);
}

fn ignoring_values_in_a_pattern_p2() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

fn ignoring_values_in_a_pattern_p3() {
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("some numbers: {}, {}, {}", first, third, fifth);
        }
    }
}

fn prefix_variables() {
    let _x = 5;
    let y = 10;

    let s = Some(String::from("hello"));

    if let Some(_s) = s {
        // underscore still binds the value
        println!("found a string");
    }

    // println!("{:?}", s); // error: partial move
}

fn range_syntax() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    // using range, ignore other fields without bindings,
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}

fn match_guards() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

fn match_guards_with_shadowing() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(n) if n == y => println!("matched, n = {}", n),
        _ => println!("default case, x = {:?}", x),
    }
}

fn match_guards_multiple() {
    let x = 4;
    let y = false;

    // 4 | 5 | 6 if y
    // (4 | 5 | 6) & y
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn bindings_at() {
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("found an id in another range")
        }
        Message::Hello { id } => {
            println!("found some other id: {}", id)
        }
    }
}

fn main() {
    println!("pattern syntax");
}
