fn main() {
    let mut i = 1;

    loop {
        if i % 3 == 0 {
            println!("Fun");
        } else if i % 5 == 0 {
            println!("Bar");
        } else if i % 5 & i % 3 == 0 {
            println!("FunBar");
        } else {
            println!("{:?}", i);
        }
        i = i + 1;

        if i == 101 {
            break;
        }
    }
    println!("Done!");
}
