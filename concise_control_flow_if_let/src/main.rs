fn main() {
    refactor_using_iflet();

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("max is config to be {max}"),
        _ => (),
    }
}

fn refactor_using_iflet() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("max is config to be {max}");
    }
}

// fn using_match() {
//     let mut count = 0;
//     match coin {
//         Coin::Quarter(state) => println!("state quarter from {:?}!", state),
//         _ => count += 1,
//     }
// }

// fn using_if_let_else() {
//     let mut count = 0;
//     if let Coin::Quarter(state) = coin {
//         println!("state quarter from {:?}!", state);
//     } else {
//         count += 1;
//     }
// }
