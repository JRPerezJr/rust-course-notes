fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let mut plus_one = vec![]; // create an empty array
    for num in numbers {
        plus_one.push(num + 1);
    }

    // using interators
    let mut plus_two = vec![];

    let plus_two: Vec<_> = numbers
        .iter() // iterate through the items
        .map(|num| num + 2) // take the item and add to 2
        .collect(); // collect the items into the vector/array

    // filter any number 4 or less
    let less_than_four: Vec<_> = numbers.iter().filter(|num| num <= &&4).collect();

    // find number 2
    let find_two = numbers.iter().find(|num| num == &&2);

    let count = numbers.iter().count();

    let last = numbers.iter().last();

    let min = numbers.iter().min();

    let max = numbers.iter().max();

    let take_first_3: Vec<_> = numbers.iter().take(3).collect();
}
