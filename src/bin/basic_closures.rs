fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add_fn(1, 1);

    // closures must be defined within a functions
    let add = |a: i32, b: i32| -> i32 { a + b };

    let sum2 = add(1, 1);

    // writing a closure without the type
    // Rust will automatically determine the type and return the value.
    let add2 = |a, b| a + b;
    let sum3 = add2(1, 1);
}
