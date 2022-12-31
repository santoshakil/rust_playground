fn main() {
    // Declare a variable `s` of type `String` and give it ownership of a new String value.
    let mut s = String::from("hello");

    // Borrow `s` immutably and pass it to the `calculate_length` function.
    // The value of `s` cannot be modified inside the `calculate_length` function.
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

    // Borrow `s` mutably and pass it to the `change` function.
    // The value of `s` can be modified inside the `change` function.
    change(&mut s);
    println!("{}", s);
}

// This function immutably borrows a `String` value and returns its length.
fn calculate_length(s: &String) -> usize {
    s.len()
}

// This function mutably borrows a `String` value and appends ", world" to it.
fn change(s: &mut String) {
    s.push_str(", world");
}
