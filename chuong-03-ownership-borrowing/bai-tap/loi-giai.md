# Lời Giải Chương 3

## Bài 1: Sửa lỗi Move (3 cách)

```rust
fn main() {
    // === CÁCH 1: Clone (tạo bản sao) ===
    let s = String::from("hello");
    let s2 = s.clone();              // Clone → s vẫn sống
    println!("s = {}, s2 = {}", s, s2);

    // === CÁCH 2: Dùng reference ===
    let s = String::from("hello");
    let s2 = &s;                     // Mượn, không move
    println!("s = {}, s2 = {}", s, s2);

    // === CÁCH 3: Không tạo s2 ===
    let s = String::from("hello");
    println!("{}", s);               // Dùng trực tiếp
    println!("{}", s);               // Dùng lại OK (không move)
}
```

---

## Bài 2: Đếm ký tự

```rust
fn dem_ky_tu(s: &str) -> usize {    // &str = mượn chuỗi
    s.chars().count()                // .chars() = iterator ký tự Unicode
                                     // .count() = đếm số phần tử
                                     // Khác .len() vì len đếm bytes (UTF-8)
}

fn main() {
    let chuoi = "Xin chào 🦀";
    println!("'{}' có {} ký tự", chuoi, dem_ky_tu(chuoi));
    // Output: 'Xin chào 🦀' có 9 ký tự
    // (.len() sẽ ra 15 vì UTF-8 dùng nhiều bytes cho tiếng Việt và emoji)
}
```

---

## Bài 3: Viết hoa

```rust
fn viet_hoa(s: &mut String) {       // &mut String = mượn để SỬA
    *s = s.to_uppercase();           // to_uppercase() trả String mới
                                     // * = gán giá trị mới cho s (dereference)
}

fn main() {
    let mut chuoi = String::from("xin chào rust");  // mut vì sẽ bị sửa
    println!("Trước: {}", chuoi);
    viet_hoa(&mut chuoi);           // Cho mượn + quyền sửa
    println!("Sau: {}", chuoi);     // "XIN CHÀO RUST"
}
```

---

## Bài 4: Nối chuỗi với reference

```rust
fn noi_chuoi(s1: &str, s2: &str) -> String {   // Mượn 2 chuỗi, trả String MỚI
    format!("{} {}", s1, s2)                     // format! tạo String mới (không sửa input)
}

fn main() {
    let s1 = String::from("Xin");
    let s2 = String::from("chào");

    let ket_qua = noi_chuoi(&s1, &s2);          // Cho mượn cả hai
    println!("Kết quả: {}", ket_qua);           // "Xin chào"
    println!("s1 vẫn OK: {}", s1);              // ✅ s1 vẫn dùng được
    println!("s2 vẫn OK: {}", s2);              // ✅ s2 vẫn dùng được
}
```
