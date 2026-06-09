# Lời Giải Chương 1

## Bài 1: Hello World

```rust
fn main() {
    let ten = "Nguyễn Văn A";       // Thay bằng tên bạn
    let tuoi = 22;                    // Thay bằng tuổi bạn

    println!("Xin chào! Tôi là {}, năm nay {} tuổi.", ten, tuoi);
    println!("Tôi đang học Rust!");
}
```

---

## Bài 2: Máy tính đơn giản

```rust
fn main() {
    let a = 10;                       // Khai báo a
    let b = 3;                        // Khai báo b

    println!("{} + {} = {}", a, b, a + b);   // Cộng
    println!("{} - {} = {}", a, b, a - b);   // Trừ
    println!("{} * {} = {}", a, b, a * b);   // Nhân
    println!("{} / {} = {}", a, b, a / b);   // Chia nguyên: 10/3 = 3
    println!("{} % {} = {}", a, b, a % b);   // Chia dư: 10%3 = 1
}
```

---

## Bài 3: Swap biến

```rust
fn main() {
    let x = 5;
    let y = 10;
    println!("Trước: x = {}, y = {}", x, y);

    // Cách 1: Dùng tuple destructuring (shadowing)
    let (x, y) = (y, x);             // Tạo biến mới x=y cũ, y=x cũ
    println!("Sau: x = {}, y = {}", x, y);
}
```

---

## Bài 4: Nhiệt độ

```rust
fn celsius_sang_fahrenheit(c: f64) -> f64 {   // Hàm chuyển C → F
    c * 9.0 / 5.0 + 32.0                      // Công thức, không ; = return
}

fn main() {
    let nhiet_do = [0.0, 25.0, 100.0];        // Array các giá trị test

    for c in nhiet_do {                        // Duyệt qua array
        let f = celsius_sang_fahrenheit(c);    // Gọi hàm
        println!("{}°C = {:.1}°F", c, f);     // {:.1} = 1 số thập phân
    }
    // Output:
    // 0°C = 32.0°F
    // 25°C = 77.0°F
    // 100°C = 212.0°F
}
```
