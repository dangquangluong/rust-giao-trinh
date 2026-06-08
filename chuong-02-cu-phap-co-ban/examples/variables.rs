// Ví dụ về biến và kiểu dữ liệu trong Rust

fn main() {
    // === BIẾN IMMUTABLE ===
    let x = 5;
    println!("x = {}", x);

    // === BIẾN MUTABLE ===
    let mut y = 10;
    println!("y trước: {}", y);
    y = 20;
    println!("y sau: {}", y);

    // === SHADOWING ===
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("z = {}", z); // 12

    // === KIỂU DỮ LIỆU ===
    let tuoi: u8 = 25;
    let nhiet_do: i32 = -10;
    let pi: f64 = 3.14159;
    let dung: bool = true;
    let chu: char = '🦀';

    println!("Tuổi: {}, Nhiệt độ: {}°C", tuoi, nhiet_do);
    println!("Pi: {}, Bool: {}, Char: {}", pi, dung, chu);

    // === TUPLE ===
    let thong_tin = ("Nam", 25, 1.75);
    println!("Tên: {}, Tuổi: {}, Cao: {}m", thong_tin.0, thong_tin.1, thong_tin.2);

    // === ARRAY ===
    let diem = [85, 90, 78, 92, 88];
    println!("Điểm đầu: {}, Số môn: {}", diem[0], diem.len());
}
