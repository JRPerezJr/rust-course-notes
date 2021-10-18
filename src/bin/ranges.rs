fn main() {
    let _range = 1..=3;

    // alternative way
    let _range2 = 1..4;

    for num in 1..4 {
        println!("{:?}", num);
    }

    for chars in 'a'..='f' {
        println!("{:?}", chars);
    }
}
