enum User {
    FirstName,
    LastName,
    Password,
    Email,
}

fn print_user_data(data: User) {
    match data {
        User::FirstName => println!("James"),
        User::LastName => println!("Bond"),
        User::Password => println!("password123"),
        User::Email => println!("jbond@007.com"),
    }
}
fn main() {
    println!("User Data:");
    print_user_data(User::FirstName);
    print_user_data(User::LastName);
    print_user_data(User::Password);
    print_user_data(User::Email);
}
