pub fn main() {
    let string = format!("my friend");
    greet(&string);
}

fn greet(name: &String) {
    println!("Hello, {}!", &name[3..]);
}

// Goal #1: Convert `greet` to use borrowing, not ownership, so that
// this program executes without doing any cloning.
//
// Goal #2: Use a subslice so that it prints "Hello, friend" instead of
// "Hello, my friend".
