fn maybe_num() -> Option<i32> {
    // code
}

fn maybe_word() -> Option<String> {
    // code
}

fn main() {
    // let plus_one = match maybe_num() {
    //     Some(num) => Some(num + 1),
    //     None => None,
    // };

    // map combinator examples
    let plus_one_mapped = maybe_num().map(|num| num + 1);

    let word_length = maybe_word().map(|word| word.len()).map(|len| len * 2);
}
