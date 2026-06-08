// Ví dụ minh họa Ownership, Move, Clone, và Borrowing
fn main() {
    println!("=== 1. MOVE ===");
    let s1 = String::from("Xin chào");
    let s2 = s1; // s1 bị move
    println!("s2 = {}", s2);

    println!("\n=== 2. CLONE ===");
    let s3 = String::from("Rust");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    println!("\n=== 3. COPY ===");
    let x = 42;
    let y = x;
    println!("x = {}, y = {}", x, y);

    println!("\n=== 4. BORROWING ===");
    let ten = String::from("Việt Nam");
    in_ra(&ten);
    println!("Vẫn dùng được: {}", ten);

    println!("\n=== 5. MUTABLE REF ===");
    let mut chuoi = String::from("Hello");
    them_text(&mut chuoi);
    println!("Sau khi thêm: {}", chuoi);
}

fn in_ra(s: &String) {
    println!("Trong hàm: {}", s);
}

fn them_text(s: &mut String) {
    s.push_str(", Rust! 🦀");
}
