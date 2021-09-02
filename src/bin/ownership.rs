#[derive(Debug)]
enum Makes {
    BMW,
    AUDI,
    VW,
}

#[derive(Debug)]
enum Model {
    M8,
    Passat,
    A8,
}

struct Vehicle {
    stk_num: i32,
    year: i32,
    make: Makes,
    model: Model,
}

fn display_make(make: &Vehicle) {
    println!("Make: {:?}", make.make);
}
fn display_year(year: &Vehicle) {
    println!("Year: {:?}", year.year);
}
fn display_model(model: &Vehicle) {
    println!("Model: {:?}", model.model);
}
fn display_stk_num(stk_num: &Vehicle) {
    println!("Stock Number: {:?}", stk_num.stk_num);
}

fn main() {
    let stock1 = Vehicle {
        stk_num: 1000,
        year: 2020,
        make: Makes::BMW,
        model: Model::M8,
    };
    let stock2 = Vehicle {
        stk_num: 1001,
        year: 2021,
        make: Makes::AUDI,
        model: Model::A8,
    };
    let stock3 = Vehicle {
        stk_num: 1003,
        year: 2022,
        make: Makes::VW,
        model: Model::Passat,
    };
    println!("Current stock data:");
    display_stk_num(&stock1);
    display_year(&stock1);
    display_make(&stock1);
    display_model(&stock1);

    display_stk_num(&stock2);
    display_year(&stock2);
    display_make(&stock2);
    display_model(&stock2);

    display_stk_num(&stock3);
    display_year(&stock3);
    display_make(&stock3);
    display_model(&stock3);
}
