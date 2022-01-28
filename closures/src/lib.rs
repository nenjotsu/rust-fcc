pub fn capturing_environment_with_closure() -> bool {
    let x = 4;
    let equal_to_x = |z| z == x;

    // fn equal_to_x(z: i32) -> bool {
    //     z == x // can't capture dynamic environment in a fn item
    // }
    let y = 4;
    assert!(equal_to_x(y));
    equal_to_x(y)
}

// closures capture values from their environment in three ways
// 1. by taking ownership = FnOnce
// 2. by borrowing immutably = Fn
// 2. by borrowing mutably = FnMut

pub fn func() {
    let x = vec![1, 2, 3];
    // force to move ownership
    let equal_to_x = move |z| z == x;
    println!("can't use x here: {:?}", x); // borrow of moved value: `x` value borrowed here after move
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
