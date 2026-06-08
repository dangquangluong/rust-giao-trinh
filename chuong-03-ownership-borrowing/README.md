# Chương 3: Ownership & Borrowing

> ⚠️ Đây là chương QUAN TRỌNG NHẤT khi học Rust!

## 3.1 Ba Quy Tắc Ownership

1. Mỗi giá trị có đúng một owner
2. Tại một thời điểm, chỉ có một owner duy nhất
3. Khi owner ra khỏi scope, giá trị bị drop

## 3.2 Move

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 bị MOVE sang s2
    // println!("{}", s1); // ❌ LỖI!
    println!("{}", s2);    // ✅ OK
}
```

## 3.3 Clone & Copy

```rust
fn main() {
    // Clone (heap)
    let s1 = String::from("Rust");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2); // ✅ Cả hai OK

    // Copy (stack - i32, f64, bool, char...)
    let x = 42;
    let y = x; // Copy, không phải move
    println!("{}, {}", x, y); // ✅ OK
}
```

## 3.4 References & Borrowing

```rust
// Immutable reference (mượn đọc)
fn tinh_do_dai(s: &String) -> usize {
    s.len()
}

// Mutable reference (mượn sửa)
fn them_text(s: &mut String) {
    s.push_str(" World!");
}

fn main() {
    let mut chuoi = String::from("Hello");
    println!("Độ dài: {}", tinh_do_dai(&chuoi));
    them_text(&mut chuoi);
    println!("{}", chuoi);
}
```

## 3.5 Quy Tắc Borrowing

- ✅ Nhiều `&T` (immutable reference) cùng lúc
- ✅ Chỉ MỘT `&mut T` tại một thời điểm
- ❌ KHÔNG thể vừa `&T` vừa `&mut T`

## 3.6 Bài Tập

1. Sửa lỗi ownership trong code cho sẵn
2. Viết hàm `dem_ky_tu(s: &str) -> usize`
3. Viết hàm `viet_hoa(s: &mut String)`

---

📖 **Trước đó**: [Chương 2](../chuong-02-cu-phap-co-ban/README.md) | **Tiếp theo**: [Chương 4](../chuong-04-struct-enum/README.md)
