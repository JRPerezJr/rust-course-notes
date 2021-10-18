enum Color {
    Red,
    Blue,
    Green,
}

fn main() {
    let maybe_user = Some("James");

    match maybe_user {
        Some(user) => println!("user={:?}", user),
        None => println!("No user found"),
    }

    // if let example
    if let Some(user) = maybe_user {
        println!("user2={:?}", user);
    } else {
        println!("No user found");
    }

    let red = Color::Red;
    if let Color::Red = red {
        println!("it's red!");
    } else {
        println!("it's not red");
    }
}
