pub fn main() {
    let (mut str1, str2) = two_words();
    str1 = join_words(&mut str1, &str2);
    println!("concatenated string is {:?}", str1);
}

fn two_words() -> (String, String) {
    (format!("fellow"), format!("Rustaceans"))
}

/// Concatenate `suffix` onto the end of `prefix`.
fn join_words(prefix: &mut String, suffix: &String) -> String {
    prefix.push(' '); // separate the words with a space
    for ch in suffix.chars() {
        prefix.push(ch);
    }
    prefix.to_string()
}

// Challenge: Convert `join_words` to use borrowing, not ownership.
// The new function should mutate `prefix` in place, and should not
// take ownership of `suffix`.
//
// Hint: If you'd like a hint as to how to proceed, open
// <http://rust-tutorials.com/exercises/hint/mutable_borrow_1/>.

// Question: Now that you've converted `join_words`, what happens if you
// call `join_words` using the same string for `prefix` and `suffix`?
// Why?
