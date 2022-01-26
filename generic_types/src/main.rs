mod structs;
mod structs_impl;
mod enums;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(number_list);
    println!("the largest number is {largest}");

    let number_list = vec![134, 250, 325, 4100, 565];
    let largest = get_largest(number_list);
    println!("the largest number is {largest}");

    let char_list = vec!['c', 'h', 'a', 'r'];
    let largest = get_largest(char_list);
    println!("the largest char is {largest}");
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
