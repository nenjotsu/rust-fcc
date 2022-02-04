use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
    //
}

// three types of procedural macros
// 1. custom derived
// 2. attribute-like
// 3. function-like

fn main() {
    println!("Hello, world!");
}
