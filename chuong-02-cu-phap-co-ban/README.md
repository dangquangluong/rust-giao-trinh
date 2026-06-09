# Chương 2: Cú Pháp Cơ Bản (So Sánh Với Python)

> Chương này dễ! Hầu hết giống Python, chỉ khác cú pháp một chút.

## 2.1 Biến - Khác Biệt Lớn Nhất!

### 🐍 Python: biến mặc định thay đổi được
```python
x = 5       # Khai báo biến, Python mặc định cho thay đổi
x = 10      # OK, thoải mái gán lại
```

### 🦀 Rust: biến mặc định KHÔNG thay đổi được!
```rust
fn main() {                      // fn = def, main = hàm chính chạy đầu tiên
    let x = 5;                   // let = khai báo biến (giống x = 5 Python), mặc định KHÔNG đổi được
    // x = 10;                   // ❌ LỖI! biến mặc định là immutable (bất biến)

    let mut y = 5;               // mut = mutable = cho phép thay đổi (Python mặc định đã thay đổi được)
    y = 10;                      // ✅ OK vì có mut ở trên
    println!("y = {}", y);       // println! = print(), {} = chỗ điền giá trị (giống f-string {y})
}                                // } = kết thúc hàm (Python dùng thụt dòng thay vì ngoặc)
```

> 🤔 **Tại sao?** Để tránh bug. Nếu biến không cần đổi thì khai báo bất biến. Compiler sẽ cảnh báo nếu bạn vô tình sửa.

---

## 2.2 Kiểu Dữ Liệu - Phải Khai Báo Rõ Ràng

### 🐍 Python: tự đoán kiểu
```python
x = 42         # int - Python tự hiểu
y = 3.14       # float
name = "An"    # str
ok = True      # bool
```

### 🦀 Rust: thường tự suy luận, nhưng có thể ghi rõ
```rust
fn main() {                          // fn main() = điểm bắt đầu chương trình
    let x = 42;                      // Rust tự hiểu: i32 (integer 32-bit, giống int Python)
    let y: f64 = 3.14;              // Ghi rõ kiểu f64 (float 64-bit, giống float Python)
    let name = "An";                 // &str = string slice (tương tự str Python nhưng cố định)
    let ok: bool = true;             // boolean - true/false (Python dùng True/False viết hoa)

    // Rust có NHIỀU kiểu số (Python chỉ có int, float)
    let tuoi: u8 = 25;              // u8 = unsigned 8-bit (0-255), tiết kiệm bộ nhớ
    let nhiet_do: i32 = -10;        // i32 = signed 32-bit (có số âm), kiểu mặc định
    let pi: f64 = 3.14159;          // f64 = float 64-bit, kiểu float mặc định
}                                    // } kết thúc main
```

### Bảng so sánh kiểu:

| Python | Rust | Ghi chú |
|--------|------|---------|
| `int` | `i32`, `i64`, `u8`, `u32`... | Rust có nhiều kích thước |
| `float` | `f32`, `f64` | |
| `bool` | `bool` | `True/False` thành `true/false` |
| `str` | `String` hoặc `&str` | 2 loại string! |

---

## 2.3 String - Phức Tạp Hơn Python!

Rust có **2 loại** string (Python chỉ có `str`):

```rust
fn main() {                                      // Hàm chính
    // &str - chuỗi cố định (giống string literal trong Python)
    let s1: &str = "Xin chào";                  // &str = chuỗi "mượn", cố định, nhẹ, nhanh
                                                 // Giống Python: s1 = "Xin chào" (không thay đổi nội dung)

    // String - chuỗi linh hoạt (giống str Python khi bạn nối chuỗi)
    let mut s2 = String::from("Hello");          // String::from() = tạo String từ text, giống str("Hello")
                                                 // mut = cho phép thay đổi nội dung sau này
    s2.push_str(", World!");                     // push_str() = nối thêm chuỗi (giống s2 += ", World!" Python)
    println!("{}", s2);                          // In ra: "Hello, World!" - {} là placeholder cho s2
}
```

> 💡 **Mẹo:** Khi mới học, cứ dùng `String::from("...")` cho an toàn. Sau hiểu rồi mới dùng `&str`.

---

## 2.4 Hàm

### 🐍 Python:
```python
def cong(a, b):          # def = khai báo hàm, không cần ghi kiểu
    return a + b         # return trả về giá trị

ket_qua = cong(3, 5)    # Gọi hàm
```

### 🦀 Rust:
```rust
fn cong(a: i32, b: i32) -> i32 {     // fn = def, (a: i32) = tham số + kiểu BẮT BUỘC
                                      // -> i32 = kiểu trả về (giống -> int trong type hint Python)
    a + b                             // Dòng cuối KHÔNG có ; = tự động return
                                      // (giống "return a + b" nhưng gọn hơn)
}                                     // } kết thúc hàm

fn main() {                           // Hàm chính, chạy đầu tiên
    let ket_qua = cong(3, 5);        // Gọi hàm, let = khai báo biến nhận kết quả
    println!("3 + 5 = {}", ket_qua); // In ra: "3 + 5 = 8", {} được thay bằng ket_qua
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
diem = 75                    # Khai báo biến
if diem >= 90:               # if + điều kiện + dấu :
    print("Xuất sắc!")       # Thụt dòng = trong block if
elif diem >= 70:             # elif = else if
    print("Khá!")
else:                        # else = trường hợp còn lại
    print("Cần cố gắng!")
```

### 🦀 Rust (gần giống, khác mấy chi tiết nhỏ):
```rust
fn main() {                              // Hàm chính
    let diem = 75;                       // let = khai báo biến (giống diem = 75)

    if diem >= 90 {                      // if + điều kiện + { (KHÔNG có dấu : như Python)
                                         // Cũng KHÔNG cần () bao điều kiện (khác C/Java)
        println!("Xuất sắc!");           // println! = print()
    } else if diem >= 70 {              // else if (Python dùng elif)
        println!("Khá!");
    } else {                             // else = trường hợp còn lại
        println!("Cần cố gắng!");
    }

    // if là expression (trả về giá trị!) - giống ternary Python: "Đậu" if diem >= 50 else "Rớt"
    let xep_loai = if diem >= 50 { "Đậu" } else { "Rớt" };  // Gán kết quả if vào biến!
    println!("{}", xep_loai);            // In kết quả
}
```

---

## 2.6 Vòng Lặp

### 🐍 Python:
```python
# for loop
for i in range(1, 6):    # range(1, 6) = 1, 2, 3, 4, 5
    print(i)

# while loop
n = 10
while n > 0:             # Lặp khi n > 0
    print(n)
    n -= 1               # Giảm n
```

### 🦀 Rust:
```rust
fn main() {                              // Hàm chính
    // for (giống Python!)
    for i in 1..=5 {                     // 1..=5 = range(1, 6) Python = 1,2,3,4,5 (có 5)
                                         // 1..5 = range(1, 5) = 1,2,3,4 (KHÔNG có 5)
        println!("{}", i);               // In giá trị i
    }

    // while (giống Python)
    let mut n = 10;                      // mut vì n sẽ thay đổi (Python mặc định cho đổi)
    while n > 0 {                        // while + điều kiện + { (giống Python nhưng dùng { thay :)
        println!("{}...", n);            // In n
        n -= 1;                          // Giảm n (giống Python)
    }                                    // } kết thúc while

    // loop = while True: trong Python (lặp vô hạn)
    let mut dem = 0;                     // Biến đếm, mut vì sẽ tăng
    let ket_qua = loop {                 // loop = lặp vô hạn, có thể trả về giá trị!
        dem += 1;                        // Tăng đếm (giống dem += 1 Python)
        if dem == 10 {                   // Kiểm tra điều kiện dừng
            break dem * 2;               // break + giá trị = dừng VÀ trả về (Python break không trả về!)
        }
    };                                   // ; vì đây là câu lệnh gán (let ket_qua = ...)
    println!("Kết quả: {}", ket_qua);   // In: 20
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

### 🐍 Python:
```python
tup = ("An", 25, 1.75)    # tuple - không thay đổi được
lst = [85, 90, 78, 92]    # list - thay đổi được, kích thước linh hoạt
```

### 🦀 Rust:
```rust
fn main() {                                      // Hàm chính
    // Tuple (giống tuple Python)
    let thong_tin = ("An", 25, 1.75_f64);        // Tạo tuple, _f64 = ghi rõ kiểu cho 1.75
                                                  // Giống Python: thong_tin = ("An", 25, 1.75)
    println!("Tên: {}, Tuổi: {}", thong_tin.0, thong_tin.1);
                                                  // .0, .1 = truy cập phần tử (Python dùng [0], [1])

    // Array - kích thước CỐ ĐỊNH (khác list Python!)
    let diem = [85, 90, 78, 92];                 // Array cố định 4 phần tử, KHÔNG thêm/xóa được
                                                  // Giống tuple Python về mặt cố định kích thước
    println!("Điểm đầu: {}", diem[0]);           // Truy cập bằng index (giống Python)
    println!("Số môn: {}", diem.len());          // .len() = len() Python

    // Vec = giống list Python (kích thước linh hoạt)
    let mut ds = vec![1, 2, 3];                  // vec![] = tạo Vec (giống [1, 2, 3] Python)
                                                  // mut vì sẽ thêm phần tử
    ds.push(4);                                   // push() = append() Python - thêm phần tử cuối
    println!("{:?}", ds);                         // {:?} = debug print, in ra [1, 2, 3, 4]
                                                  // {:?} giống repr() Python
}
```

---

## 2.9 Tóm Tắt: Python thành Rust Cheat Sheet

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
