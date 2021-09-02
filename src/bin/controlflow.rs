let a = 99;

if a > 99 {
    println!("Big number");
} else {
    println!("Small number");
}

// Nested example
if a > 99 {
    if a > 200 {
        println!("Huge number");
    }else{
        println!("Big number");
    }
} else {
    println!("Small number");
}

// if else example
if a > 200 {
    println!("Huge number");
} else if a > 99 {
    println!("Big number");
} else {
    println!("Small number");
}