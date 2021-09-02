fn main() {
    let coord = (2, 3);
    println!("Coordinates are {:?}, {:?}", coord.0, coord.1);

    let (x, y, z) = (2, 3, 10);
    println!("Axis data: x:{:?}, x:{:?} z:{:?}", x, y, z);
    let (first_name, last_name, age) = ("James", "Bond", 30);
    println!(
        "Welcome {:?} {:?} your age is {:?}",
        first_name, last_name, age
    );
}
