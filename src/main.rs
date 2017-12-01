// mod grammar;

mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

fn main() {
    println!("Hello, world!");
    match grammar::program("hello world asd") {
        Ok(r) => println!("Parsed as: {:?}", r),
        Err(e) => println!("Parse error: {}", e),
    }
}
