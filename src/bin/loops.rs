fn main() {
    let mut a = 0;
    loop {
        if a == 5 {
            break;
        }
        println!("{:?}", a);
        a = a + 1;
    }

    // While loop example
    let mut a = 0;
    while a != 5 {
        println!("{:?}", a);
        a = a + 1;
    }
}
