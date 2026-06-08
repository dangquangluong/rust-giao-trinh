// === PYTHON EQUIVALENT ===
// # Python:
// # s1 = "Xin chao"
// # s2 = s1           # Cả s1 và s2 cùng trỏ tới chuỗi (OK)
// # print(s1, s2)     # Cả hai đều dùng được
// #
// # s3 = "Rust"
// # s4 = s3           # Tương tự, cả hai OK
// #
// # def in_ra(s):
// #     print(f"Trong ham: {s}")
// #
// # ten = "Viet Nam"
// # in_ra(ten)
// # print(f"Van dung duoc: {ten}")  # OK! Python không "lấy" ten đi

// Ví dụ minh họa Ownership, Move, Clone, và Borrowing
// KHÁC BIỆT LỚN với Python: Rust "move" (chuyển quyền) thay vì "share"

fn main() {
    // === 1. MOVE (Khái niệm KHÔNG CÓ trong Python) ===
    // Trong Python: s2 = s1 -> cả hai cùng trỏ tới data
    // Trong Rust: s2 = s1 -> s1 MẤT quyền, chỉ s2 dùng được
    println!("=== 1. MOVE ===");
    let s1 = String::from("Xin chao");
    let s2 = s1; // s1 bị move! Giống "sang tên" quyền sở hữu
    // println!("{}", s1); // LỖI nếu bỏ comment! s1 đã "chết"
    println!("s2 = {}", s2);

    // === 2. CLONE (Giống copy.deepcopy() Python) ===
    // Muốn cả hai đều dùng? Dùng .clone() để tạo bản sao
    println!("\n=== 2. CLONE ===");
    let s3 = String::from("Rust");
    let s4 = s3.clone(); // Tạo bản sao hoàn toàn
    println!("s3 = {}, s4 = {}", s3, s4); // Cả hai OK!

    // === 3. COPY (Kiểu nhỏ tự động copy, giống Python int/float) ===
    // Số nguyên, float, bool, char: tự động copy, KHÔNG bị move
    println!("\n=== 3. COPY ===");
    let x = 42;
    let y = x; // Copy tự động (vì i32 nhỏ)
    println!("x = {}, y = {}", x, y); // Cả hai OK!

    // === 4. BORROWING - cho mượn đọc (giống truyền tham số Python) ===
    // Dùng & để "cho mượn" thay vì "cho luôn"
    println!("\n=== 4. BORROWING (cho muon doc) ===");
    let ten = String::from("Viet Nam");
    in_ra(&ten); // &ten = cho mượn đọc, không move
    println!("Van dung duoc: {}", ten); // OK! ten vẫn là chủ

    // === 5. MUTABLE REF - cho mượn + sửa ===
    // Giống truyền list/dict vào hàm Python rồi sửa bên trong
    println!("\n=== 5. MUTABLE REF (cho muon + sua) ===");
    let mut chuoi = String::from("Hello");
    println!("Truoc: {}", chuoi);
    them_text(&mut chuoi); // Cho mượn + quyền sửa
    println!("Sau khi them: {}", chuoi); // "Hello, Rust! 🦀"
}

// Hàm nhận reference (mượn đọc) - không lấy ownership
// Giống def in_ra(s): trong Python
fn in_ra(s: &String) {
    println!("Trong ham: {}", s);
}

// Hàm nhận mutable reference (mượn + sửa)
// Giống def them(lst): lst.append(...) trong Python
fn them_text(s: &mut String) {
    s.push_str(", Rust! 🦀");
}
