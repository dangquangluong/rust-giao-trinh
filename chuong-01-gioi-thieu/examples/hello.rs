// === PYTHON EQUIVALENT ===
// # Python:
// # ten = "Ban"
// # print(f"Xin chao, {ten}! Chao mung den voi Rust.")
// # print("Day la chuong trinh dau tien cua toi.")
// # print("Rust rat thu vi!")

// Chương trình Hello World đầu tiên trong Rust
fn main() {
    // println! là macro dùng để in ra màn hình (giống print() trong Python)
    println!("Xin chào, Rust! 🦀");

    // In nhiều dòng
    println!("Đây là chương trình đầu tiên của tôi.");
    println!("Rust rất thú vị!");

    // Sử dụng format string (giống f-string trong Python)
    let ten = "Bạn";  // giống: ten = "Ban" trong Python
    println!("Xin chào, {}! Chào mừng đến với Rust.", ten);

    // Từ Rust 1.58+, có thể viết giống f-string Python hơn:
    println!("Xin chào, {ten}! Đây là cách viết mới.");
}
