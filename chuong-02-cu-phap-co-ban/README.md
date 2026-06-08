# Chương 2: Cú Pháp Cơ Bản

## 2.1 Biến (Variables)

### Khai Báo Biến với `let`

Trong Rust, biến mặc định là **immutable** (không thể thay đổi):

```rust
fn main() {
    let x = 5;        // x là immutable
    println!("x = {}", x);
    // x = 10;        // ❌ LỖI! Không thể thay đổi biến immutable
}
```

### Biến Mutable (có thể thay đổi)

```rust
fn main() {
    let mut x = 5;    // x là mutable
    x = 10;           // ✅ OK!
    println!("x = {}", x);
}
```

### Shadowing

```rust
fn main() {
    let x = 5;
    let x = x + 1;       // x = 6
    let x = x * 2;       // x = 12
    println!("x = {}", x);
}
```

### Hằng Số

```rust
const MAX_DIEM: u32 = 100;
const PI: f64 = 3.14159265358979;
```

## 2.2 Kiểu Dữ Liệu

### Số Nguyên

| Kiểu | Kích thước | Phạm vi |
|------|-----------|---------|
| `i8` | 8 bit | -128 đến 127 |
| `i32` | 32 bit | ~ ±2 tỷ (mặc định) |
| `u8` | 8 bit | 0 đến 255 |
| `u32` | 32 bit | 0 đến ~4 tỷ |
| `usize` | tùy hệ thống | dùng cho index |

### Số Thực, Boolean, Char

```rust
fn main() {
    let pi: f64 = 3.14;
    let dung: bool = true;
    let emoji: char = '🦀';
}
```

### String

```rust
fn main() {
    let s1: &str = "Xin chào";          // string slice (cố định)
    let s2: String = String::from("Hi"); // String (linh hoạt)
}
```

## 2.3 Hàm (Functions)

```rust
fn cong(a: i32, b: i32) -> i32 {
    a + b    // Không có ; => giá trị trả về
}

fn chao(ten: &str) {
    println!("Xin chào, {}!", ten);
}

fn main() {
    chao("Rust");
    println!("3 + 5 = {}", cong(3, 5));
}
```

## 2.4 Điều Kiện

```rust
fn main() {
    let diem = 75;

    if diem >= 90 {
        println!("Xuất sắc!");
    } else if diem >= 70 {
        println!("Khá!");
    } else {
        println!("Cần cố gắng!");
    }

    // if là expression
    let xep_loai = if diem >= 50 { "Đậu" } else { "Rớt" };
    println!("{}", xep_loai);
}
```

## 2.5 Vòng Lặp

```rust
fn main() {
    // for
    for i in 1..=5 {
        println!("Lần {}", i);
    }

    // while
    let mut n = 10;
    while n > 0 {
        println!("{}...", n);
        n -= 1;
    }

    // loop
    let mut dem = 0;
    let ket_qua = loop {
        dem += 1;
        if dem == 10 { break dem * 2; }
    };
    println!("Kết quả: {}", ket_qua);
}
```

## 2.6 Bài Tập

1. Viết máy tính đơn giản (+, -, *, /)
2. Viết hàm kiểm tra số chẵn/lẻ
3. In bảng cửu chương 2-9
4. FizzBuzz (1-100)

---

📖 **Trước đó**: [Chương 1](../chuong-01-gioi-thieu/README.md) | **Tiếp theo**: [Chương 3](../chuong-03-ownership-borrowing/README.md)
