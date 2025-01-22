use std::env;

fn main() {
    for argument in env::args() {
        let reversed = reverse(argument.clone());
        println!("{argument} reversed: {reversed}");
    }
}

fn reverse(s: String) -> String {
    let input = s.chars();
    let mut chars: Vec<u8> = vec![];
    let len = s.len();
    chars.resize(len, 0); // warning, allocations!
    let mut current_index = 0;
    for x in input {
        chars.insert(len - current_index, x as u8);
        current_index += 1;
    }

    String::from_utf8(chars).expect("should be a correct string")
}
