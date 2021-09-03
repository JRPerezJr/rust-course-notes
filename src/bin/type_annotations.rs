enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
}

fn main() {
    fn print_many(msg: &str, count: i32) {
        unimplemented!();
    }

    let num: i32 = 15;
    let a: char = 'a';
    let left_click: Mouse = Mouse::LeftClick;

    // Vectors
    let numbers: Vec<i32> = vec![1, 2, 3, 4];
    let letters: Vec<char> = vec!['a', 'b', 'c', 'd'];
    let clicks: Vec<Mouse> = vec![Mouse::LeftClick, Mouse::MiddleClick, Mouse::RightClick];
}
