# Chương 2: Cú Pháp Cơ Bản (So Sánh Với Python)

> Chương này dễ! Hầu hết giống Python, chỉ khác cú pháp một chút.

## 2.1 Biến - Khác Biệt Lớn Nhất!

### 🐍 Python: biến mặc định thay đổi được
```python
x = 5
x = 10  # OK, thoải mái
```

### 🦀 Rust: biến mặc định KHÔNG thay đổi được!
```rust
fn main() {
    let x = 5;
    // x = 10;  // ❌ LỖI! biến mặc định là immutable

    let mut y = 5;  // mut = mutable (thay đổi được)
    y = 10;          // ✅ OK vì có mut
    println!("y = {}", y);
}
```

> 🤔 **Tại sao?** Để tránh bug. Nếu biến không cần đổi → khai báo bất biến. Compiler sẽ cảnh báo nếu bạn vô tình sửa.

---

## 2.2 Kiểu Dữ Liệu - Phải Khai Báo Rõ Ràng

### 🐍 Python: tự đoán kiểu
```python
x = 42         # int
y = 3.14       # float
name = "An"    # str
ok = True      # bool
```

### 🦀 Rust: thường tự suy luận, nhưng có thể ghi rõ
```rust
fn main() {
    let x = 42;           // Rust tự hiểu: i32
    let y: f64 = 3.14;   // Ghi rõ kiểu: f64
    let name = "An";     // &str (string slice)
    let ok: bool = true;  // boolean

    // Rust có NHIỀU kiểu số (Python chỉ có int, float)
    let tuoi: u8 = 25;        // unsigned 8-bit (0-255)
    let nhiet_do: i32 = -10;  // signed 32-bit
    let pi: f64 = 3.14159;    // float 64-bit
}
```

### Bảng so sánh kiểu:

| Python | Rust | Ghi chú |
|--------|------|---------|
| `int` | `i32`, `i64`, `u8`, `u32`... | Rust có nhiều kích thước |
| `float` | `f32`, `f64` | |
| `bool` | `bool` | `True/False` → `true/false` |
| `str` | `String` hoặc `&str` | 2 loại string! |

---

## 2.3 String - Phức Tạp Hơn Python!

Rust có **2 loại** string (Python chỉ có `str`):

```rust
fn main() {
    // &str - chuỗi cố định (giống string literal)
    let s1: &str = "Xin chào";  // Nhẹ, nhanh, không thay đổi được

    // String - chuỗi linh hoạt (giống str Python thông thường)
    let mut s2 = String::from("Hello");  // Có thể thay đổi
    s2.push_str(", World!");
    println!("{}", s2);  // "Hello, World!"
}
```

> 💡 **Mẹo:** Khi mới học, cứ dùng `String::from("...")` cho an toàn. Sau hiểu rồi mới dùng `&str`.

---

## 2.4 Hàm

### 🐍 Python:
```python
def cong(a, b):
    return a + b

ket_qua = cong(3, 5)
```

### 🦀 Rust:
```rust
fn cong(a: i32, b: i32) -> i32 {  // Phải ghi kiểu tham số + kiểu trả về
    a + b  // Không có ; ở dòng cuối = return
}

fn main() {
    let ket_qua = cong(3, 5);
    println!("3 + 5 = {}", ket_qua);
}
```

### Khác biệt quan trọng:

| | Python | Rust |
|-|--------|------|
| Khai báo | `def ten(a, b):` | `fn ten(a: i32, b: i32) -> i32` |
| Return | `return value` | Dòng cuối không có `;` = return |
| Kiểu | Không cần | BẮT BUỘC ghi kiểu |

---

## 2.5 If/Else

### 🐍 Python:
```python
diem = 75
if diem >= 90:
    print("Xuất sắc!")
elif diem >= 70:
    print("Khá!")
else:
    print("Cần cố gắng!")
```

### 🦀 Rust (gần giống, khác mấy chi tiết nhỏ):
```rust
fn main() {
    let diem = 75;

    if diem >= 90 {         // Không có : và ()
        println!("Xuất sắc!");
    } else if diem >= 70 {  // else if (không phải elif)
        println!("Khá!");
    } else {
        println!("Cần cố gắng!");
    }

    // if là expression (trả về giá trị!) - Python cũng có tương tự
    let xep_loai = if diem >= 50 { "Đậu" } else { "Rớt" };
    println!("{}", xep_loai);
}
```

---

## 2.6 Vòng Lặp

### 🐍 Python → 🦀 Rust:

```python
# Python: for
for i in range(1, 6):
    print(i)

# Python: while
n = 10
while n > 0:
    print(n)
    n -= 1
```

```rust
fn main() {
    // for (giống Python!)
    for i in 1..=5 {      // 1..=5 = range(1, 6) trong Python
        println!("{}", i);
    }

    // 1..5 = range(1, 5) = 1,2,3,4 (không có 5)
    // 1..=5 = 1,2,3,4,5 (có 5)

    // while (giống Python)
    let mut n = 10;
    while n > 0 {
        println!("{}...", n);
        n -= 1;
    }

    // loop = while True: trong Python
    let mut dem = 0;
    let ket_qua = loop {
        dem += 1;
        if dem == 10 {
            break dem * 2;  // break + trả về giá trị!
        }
    };
    println!("Kết quả: {}", ket_qua);  // 20
}
```

---

## 2.7 In Ra Màn Hình

| Python | Rust | Ghi chú |
|--------|------|---------|
| `print("hello")` | `println!("hello");` | Có dấu `!` (macro) và `;` |
| `print(f"x = {x}")` | `println!("x = {}", x);` | Dùng `{}` placeholder |
| `print(f"{x:.2f}")` | `println!("{:.2}", x);` | Format giống nhau |

---

## 2.8 Tuple & Array

### 🐍 Python → 🦀 Rust:

```python
# Python
tup = ("An", 25, 1.75)    # tuple
lst = [85, 90, 78, 92]    # list
```

```rust
fn main() {
    // Tuple (giống Python)
    let thong_tin = ("An", 25, 1.75_f64);
    println!("Tên: {}, Tuổi: {}", thong_tin.0, thong_tin.1);

    // Array - kích thước CỐ ĐỊNH (khác list Python!)
    let diem = [85, 90, 78, 92];
    println!("Điểm đầu: {}", diem[0]);
    println!("Số môn: {}", diem.len());

    // Vec = giống list Python (kích thước linh hoạt)
    let mut ds = vec![1, 2, 3];
    ds.push(4);
    println!("{:?}", ds);  // [1, 2, 3, 4]
}
```

---

## 2.9 Tóm Tắt: Python → Rust Cheat Sheet

| Bạn muốn | Python | Rust |
|-----------|--------|------|
| Biến thường | `x = 5` | `let x = 5;` |
| Biến thay đổi | `x = 5` (mặc định) | `let mut x = 5;` |
| Hàm | `def f(a):` | `fn f(a: i32) -> i32 {` |
| In ra | `print(f"x={x}")` | `println!("x={}", x);` |
| List | `[1,2,3]` | `vec![1,2,3]` |
| For loop | `for i in range(5):` | `for i in 0..5 {` |
| While | `while x > 0:` | `while x > 0 {` |
| If | `if x > 0:` | `if x > 0 {` |
| Comment | `# comment` | `// comment` |

---

## 2.10 Bài Tập

1. Viết hàm `tinh_bmi(can_nang: f64, chieu_cao: f64) -> f64`
2. FizzBuzz 1-30 (giống Python nhưng bằng Rust)
3. Viết hàm kiểm tra số chẵn/lẻ: `la_chan(n: i32) -> bool`
4. Tính tổng từ 1 đến n bằng vòng lặp for

---

📖 **Trước đó**: [Chương 1](../chuong-01-gioi-thieu/README.md) | **Tiếp theo**: [Chương 3](../chuong-03-ownership-borrowing/README.md)
