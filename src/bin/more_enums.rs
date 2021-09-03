// Example1
// enum Mouse {
//     LeftClick,
//     RightClick,
//     MiddleClick,
//     Scroll(i32),
//     Move(i32, i32),
// }

// Example2
// enum PromoDiscount {
//     NewUser,
//     Holiday(String),
// }

// enum Discount {
//     Percent(f64),
//     Flat(i32),
//     Promo(PromoDiscount),
//     Custom(String),
// }

#[derive(Debug)]
enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        3 => println!("three"),
        other => println!("number: {:?}", other),
    }
    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("flat 2"),
        Discount::Flat(amount) => println!("flat discount of {:?}", amount),
        _ => (), // ignore the rest and return nothing
    }

    let concert = Ticket {
        event: String::from("concert"),
        price: 50,
    };
    // Matching on a strut
    match concert {
        Ticket { price: 50, event } => println!("event @ 50 = {:?}", event), // match any event that is $50 and get the event printed out
        Ticket { price, .. } => println!("price {:?}", price), // 2 dots means ignore everything else so ignore the event name
    }
}
