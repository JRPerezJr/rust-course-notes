struct User {
    first_name: String,
    last_name: String,
    password: String,
    email: String,
}

struct LineItem {
    name: String,
    count: i32,
}

fn print_it(data: &str) {
    println!("{:?}", data);
}
fn main() {
    print_it("This is a string slice!");
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another");
    print_it(&owned_string);
    print_it(&another_owned);

    let first_name = "James".to_owned();
    let last_name = "Bond".to_owned();
    let pw = "password123".to_owned();
    let email = "jbond@007.com".to_owned();
    let user_data = User {
        first_name: first_name,
        last_name: last_name,
        password: pw,
        email: email,
    };
    println!(
        "User data:\n First Name: {:?}\n Last Name: {:?}\n",
        user_data.first_name, user_data.last_name
    );

    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 1,
        },
        LineItem {
            name: String::from("fruit"),
            count: 3,
        },
    ];
    println!("Receipt Data:");
    for item in receipt {
        println!("name: {:?}, count: {:?}", item.name, item.count);
    }
}
