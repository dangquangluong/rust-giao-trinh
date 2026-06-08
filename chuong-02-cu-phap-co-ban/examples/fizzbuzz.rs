// Bài FizzBuzz kinh điển trong Rust
fn main() {
    for i in 1..=30 {
        let ket_qua = if i % 15 == 0 {
            String::from("FizzBuzz")
        } else if i % 3 == 0 {
            String::from("Fizz")
        } else if i % 5 == 0 {
            String::from("Buzz")
        } else {
            i.to_string()
        };
        print!("{:>8} ", ket_qua);
        if i % 5 == 0 { println!(); }
    }
}
