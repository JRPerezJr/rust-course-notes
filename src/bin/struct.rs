struct Container {
    depth: i32,
    width: i32,
    height: i32,
    price: f64,
}

fn main() {
    let my_container = Container {
        depth: 3,
        width: 2,
        height: 5,
        price: 3500.89,
    };
    let deep = my_container.depth;
    let wide = my_container.width;
    let high = my_container.height;
    let cost = my_container.price;

    println!("Container Dimensions and Cost:");
    println!("The container is {:?} meters deep", deep);
    println!("The container is {:?} meters wide", wide);
    println!("The container is {:?} meters high", high);
    println!("The price for the container is ${:?}", cost);
}
