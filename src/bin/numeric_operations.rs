fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    let sum = 2 + 2;
    let value = 10 - 5;
    let division = 10 / 2;
    let mult = 5 * 5;

    let five = sub(8, 3);
    let remainder = 6 % 3;
    let remainder2 = 6 % 4;
    println!("{:?}", sum);
    println!("{:?}", value);
    println!("{:?}", division);
    println!("{:?}", mult);
    println!("{:?}", five);
    println!("{:?}", remainder);
    println!("{:?}", remainder2);
}
