// === PYTHON EQUIVALENT ===
// # Python:
// # for i in range(1, 31):
// #     if i % 15 == 0:
// #         print("FizzBuzz", end=" ")
// #     elif i % 3 == 0:
// #         print("Fizz", end=" ")
// #     elif i % 5 == 0:
// #         print("Buzz", end=" ")
// #     else:
// #         print(i, end=" ")
// #     if i % 5 == 0:
// #         print()  # xuống dòng

// Bài FizzBuzz kinh điển trong Rust
// So sánh: Rust dùng match/if expression, Python dùng if/elif
fn main() {
    for i in 1..=30 {
        // Trong Rust, if là expression (trả về giá trị)
        // Giống ternary Python: x = "a" if cond else "b"
        // Nhưng mạnh hơn vì có nhiều nhánh
        let ket_qua = if i % 15 == 0 {
            String::from("FizzBuzz")
        } else if i % 3 == 0 {
            String::from("Fizz")
        } else if i % 5 == 0 {
            String::from("Buzz")
        } else {
            i.to_string()  // Giống str(i) trong Python
        };

        print!("{:>8} ", ket_qua);  // {:>8} = căn phải 8 ký tự
        if i % 5 == 0 {
            println!();  // Xuống dòng (giống print() Python)
        }
    }
}
