use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // Strings are stored as a colection of UTF-8 encoded bytes
    let s1 = String::new();
    assert_eq!(s1, "");
    let s2 = "initial contents"; // string slice
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    // appending using push & push_str
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    // foobar!

    // appending using +
    // this will save memory, instead of copying values of string
    let s5 = String::from("hello, ");
    let s6 = String::from("world!");
    let s7: String = s5 + &s6; // using +
    print!("{s7}");
    let s8 = format!("{s7}{s6}"); // using format, format macro doesnt take ownership, so you can use the variables even after this
    assert_eq!(s8, "hello, world!world!");

    // indexing
    let hello = String::from("hello");
    // let c:char = hello[0]; // this is error

    let namaste = String::from("नमस्ते");

    // Bytes
    println!("Bytes: {:?}", namaste.as_bytes());
    
    // Scalar values
    println!("Scalar values: {:?}", namaste.chars());
    
    // Grapheme clusters
    // println!("Grapheme clusters: {:?}", namaste.graphemes(true));
    for g in String::from("नमस्ते").graphemes(true) {
      println!("{g}");
    }
}

