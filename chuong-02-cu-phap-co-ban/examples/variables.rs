// === PYTHON EQUIVALENT ===
// # Python:
// # x = 5                    # Biến (mutable mặc định)
// # y = 10; y = 20           # Gán lại OK
// # tuoi = 25                # int
// # nhiet_do = -10           # int
// # pi = 3.14159             # float
// # dung = True              # bool
// # thong_tin = ("Nam", 25, 1.75)  # tuple
// # diem = [85, 90, 78, 92, 88]    # list

// Ví dụ về biến và kiểu dữ liệu trong Rust
// So sánh với Python để dễ hiểu hơn

fn main() {
    // === BIẾN IMMUTABLE (Python không có concept này) ===
    let x = 5;
    println!("x = {}", x);
    // x = 10;  // LỖI! Trong Python thì OK, Rust thì không

    // === BIẾN MUTABLE (giống biến bình thường Python) ===
    let mut y = 10;
    println!("y truoc: {}", y);
    y = 20;  // OK vì có "mut"
    println!("y sau: {}", y);

    // === SHADOWING (tạo biến mới cùng tên) ===
    // Python: x = 5; x = x + 1; x = x * 2 (giống nhau về kết quả)
    let z = 5;
    let z = z + 1;   // Tạo biến z mới = 6
    let z = z * 2;   // Tạo biến z mới = 12
    println!("z = {}", z);

    // === KIỂU DỮ LIỆU ===
    // Python: tuoi = 25 (tự suy kiểu, không giới hạn)
    // Rust: phải chọn kiểu cụ thể
    let tuoi: u8 = 25;        // u8: 0-255, đủ cho tuổi
    let nhiet_do: i32 = -10;  // i32: số nguyên có âm
    let pi: f64 = 3.14159;    // f64: giống float Python
    let dung: bool = true;    // bool: giống True/False Python (viết thường!)
    let chu: char = '🦀';     // char: 1 ký tự (dùng nháy đơn)

    println!("Tuoi: {}, Nhiet do: {}°C", tuoi, nhiet_do);
    println!("Pi: {}, Bool: {}, Char: {}", pi, dung, chu);

    // === TUPLE (giống tuple Python) ===
    // Python: thong_tin = ("Nam", 25, 1.75)
    let thong_tin = ("Nam", 25, 1.75);
    println!("Ten: {}, Tuoi: {}, Cao: {}m", thong_tin.0, thong_tin.1, thong_tin.2);

    // Destructuring (giống unpacking Python: ten, tuoi, cao = thong_tin)
    let (ten, tuoi_2, cao) = thong_tin;
    println!("Unpacked: {} {} {}", ten, tuoi_2, cao);

    // === ARRAY (khác list Python! kích thước cố định) ===
    // Python list: diem = [85, 90, 78, 92, 88]; diem.append(100) OK
    // Rust array: không thêm/bớt được
    let diem = [85, 90, 78, 92, 88];
    println!("Diem dau: {}, So mon: {}", diem[0], diem.len());

    // Muốn giống list Python? Dùng Vec!
    let mut diem_vec = vec![85, 90, 78];
    diem_vec.push(92);  // Thêm phần tử (giống .append() Python)
    println!("Vec: {:?}", diem_vec);
}
