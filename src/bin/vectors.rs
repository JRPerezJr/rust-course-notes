struct Test {
    score: i32,
}

fn main() {
    println!("My Cites:");
    let my_cites = vec!["Tokyo", "Osaka", "Fukuoka"];
    for city in my_cites {
        println!("{:?}", city);
    }
    println!("My Scores:");
    let my_scores = vec![
        Test { score: 90 },
        Test { score: 30 },
        Test { score: 85 },
        Test { score: 50 },
    ];

    for test in my_scores {
        println!("score = {:?}", test.score);
    }

    println!("My numbers:");
    let my_numbers = vec![10, 20, 30, 40];
    for num in &my_numbers {
        match num {
            30 => println!("thirty"),
            _ => println!("{:?}", num),
        }
    }
    println!("number of elements = {:?}", my_numbers.len());
}
