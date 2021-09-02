fn main() {
    let mut i = 1;
    while i <= 3 {
        println!("{:?}", i);
        i += 1;
    }

    println!("Start fun bar:");
    while i <= 20 {
        if i % 3 == 0 {
            println!("Fun");
        } else if i % 5 == 0 {
            println!("Bar");
        } else if i % 5 & i % 3 == 0 {
            println!("FunBar");
        } else {
            println!("{:?}", i);
        }
        i += 1;
    }
}
