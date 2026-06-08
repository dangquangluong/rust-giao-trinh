# Chương 6: Collections & Generics (list thành Vec, dict thành HashMap)

## 6.1 Vec = list Python

### 🐍 Python:
```python
diem = [85, 90, 78, 92]          # Tạo list
diem.append(100)                  # Thêm phần tử cuối
diem.sort()                       # Sắp xếp
print(sum(diem) / len(diem))      # Tính trung bình
```

### 🦀 Rust:
```rust
fn main() {                                      // Hàm chính
    let mut diem = vec![85, 90, 78, 92];         // vec![] = tạo Vec (giống [85, 90, 78, 92] Python)
                                                  // mut vì sẽ thêm phần tử và sắp xếp
    diem.push(100);                              // push() = append() Python - thêm vào cuối
    diem.sort();                                 // sort() = sắp xếp tăng dần (giống Python)

    let tong: i32 = diem.iter().sum();           // iter() = tạo iterator, sum() = sum() Python
                                                  // Phải ghi kiểu i32 vì Rust cần biết kiểu tổng
    let tb = tong as f64 / diem.len() as f64;   // as f64 = ép kiểu (giống float(tong) Python)
                                                  // len() = len() Python
    println!("TB: {:.1}", tb);                   // {:.1} = format 1 chữ số thập phân
}
```

### So sánh methods:

| Python | Rust | Ghi chú |
|--------|------|---------|
| `lst.append(x)` | `vec.push(x)` | |
| `lst.pop()` | `vec.pop()` | Trả `Option` |
| `lst.remove(x)` | `vec.remove(index)` | Rust xóa theo index |
| `lst.sort()` | `vec.sort()` | |
| `len(lst)` | `vec.len()` | |
| `lst[0]` | `vec[0]` | Giống nhau! |
| `x in lst` | `vec.contains(&x)` | |
| `lst[1:3]` | `&vec[1..3]` | |

---

## 6.2 HashMap = dict Python

### 🐍 Python:
```python
diem = {"An": 8.5, "Binh": 7.0, "Cuong": 9.2}  # Tạo dict
diem["Dung"] = 6.5                                # Thêm key-value mới

for ten, d in diem.items():                       # Duyệt qua dict
    print(f"{ten}: {d}")
```

### 🦀 Rust:
```rust
use std::collections::HashMap;                   // Import HashMap (giống from collections import ...)
                                                  // Rust không có dict sẵn, phải import

fn main() {                                       // Hàm chính
    let mut diem = HashMap::new();               // HashMap::new() = {} Python (tạo dict rỗng)
                                                  // mut vì sẽ thêm phần tử
    diem.insert("An", 8.5);                      // insert() = diem["An"] = 8.5 Python
    diem.insert("Binh", 7.0);                    // Thêm key "Binh"
    diem.insert("Cuong", 9.2);                   // Thêm key "Cuong"
    diem.insert("Dung", 6.5);                    // Thêm key "Dung"

    for (ten, d) in &diem {                      // &diem = mượn dict để duyệt (giống for k,v in d.items())
        println!("{}: {}", ten, d);              // In từng cặp key-value
    }

    // Truy cập (trả Option vì key có thể không tồn tại - an toàn hơn Python!)
    if let Some(d) = diem.get("An") {            // get() trả Option (giống dict.get() Python)
                                                  // if let Some(d) = "nếu có giá trị, lấy ra gán vào d"
        println!("Điểm An: {}", d);             // In điểm
    }
}
```

### So sánh:

| Python | Rust |
|--------|------|
| `d["key"]` | `d.get("key")` (trả `Option`) |
| `d["key"] = val` | `d.insert("key", val)` |
| `"key" in d` | `d.contains_key("key")` |
| `del d["key"]` | `d.remove("key")` |
| `for k, v in d.items():` | `for (k, v) in &d {` |

---

## 6.3 Iterator = Giống list comprehension Python

### 🐍 Python:
```python
so = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
chan_bp = [x**2 for x in so if x % 2 == 0]      # List comprehension: lọc chẵn, bình phương
tong = sum(so)                                    # Tính tổng
```

### 🦀 Rust:
```rust
fn main() {                                       // Hàm chính
    let so = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];  // Tạo Vec (giống list Python)

    // Giống list comprehension [x**2 for x in so if x % 2 == 0]
    let chan_bp: Vec<i32> = so.iter()            // iter() = bắt đầu chuỗi xử lý (giống generator)
        .filter(|&&x| x % 2 == 0)               // filter = if x % 2 == 0 (lọc số chẵn)
                                                  // |&&x| = closure (giống lambda x:), && vì iter cho &
        .map(|&x| x * x)                        // map = x**2 (biến đổi từng phần tử)
                                                  // |&x| x * x giống lambda x: x**2
        .collect();                               // collect() = gom kết quả thành Vec
                                                  // Phải có collect() vì iterator lazy (giống generator Python)

    let tong: i32 = so.iter().sum();             // sum() = sum() Python, iter() vì cần iterator
    
    println!("Chẵn bình phương: {:?}", chan_bp); // {:?} = debug print cho Vec
    println!("Tổng: {}", tong);                  // In tổng
}
```

### Bảng chuyển đổi Iterator:

| Python | Rust | Ý nghĩa |
|--------|------|---------|
| `[f(x) for x in lst]` | `.iter().map(\|x\| f(x)).collect()` | Biến đổi |
| `[x for x in lst if cond]` | `.iter().filter(\|x\| cond).collect()` | Lọc |
| `sum(lst)` | `.iter().sum()` | Tổng |
| `max(lst)` | `.iter().max()` | Lớn nhất |
| `any(...)` | `.iter().any(\|x\| ...)` | Có phần tử thỏa? |
| `all(...)` | `.iter().all(\|x\| ...)` | Tất cả thỏa? |
| `enumerate(lst)` | `.iter().enumerate()` | Có index |

---

## 6.4 Generics = typing Python

### 🐍 Python (type hints):
```python
from typing import List, TypeVar         # Import typing
T = TypeVar('T')                         # T = kiểu generic (bất kỳ kiểu nào)

def first(lst: List[T]) -> T:           # Hàm nhận list kiểu T, trả về T
    return lst[0]                        # Python KHÔNG kiểm tra thật lúc chạy
```

### 🦀 Rust:
```rust
// Hàm generic - hoạt động với mọi kiểu có thể so sánh
fn tim_max<T: PartialOrd>(list: &[T]) -> &T {   // <T: PartialOrd> = T phải so sánh được
                                                  // &[T] = slice (mượn array/Vec)
                                                  // -> &T = trả reference đến phần tử lớn nhất
    let mut max = &list[0];                      // Giả sử phần tử đầu là max
    for item in &list[1..] {                     // Duyệt từ phần tử thứ 2 (giống list[1:] Python)
        if item > max {                          // So sánh (được phép vì T: PartialOrd)
            max = item;                          // Cập nhật max mới
        }
    }
    max                                           // Return max (không có ; = return)
}

fn main() {                                       // Hàm chính
    let so = vec![3, 1, 4, 1, 5, 9];            // Vec<i32>
    println!("Max: {}", tim_max(&so));           // Gọi với i32, Rust tự suy luận T = i32

    let chu = vec!['z', 'a', 'm'];              // Vec<char>
    println!("Max: {}", tim_max(&chu));          // Cùng hàm, T = char - đa hình (polymorphism)!
}
```

> 💡 **Generics Rust** = type hints Python nhưng **compiler kiểm tra thật**, không chỉ là gợi ý!

---

## 6.5 Bài Tập

1. Vec: nhập list điểm, tính TB, tìm max/min, đếm điểm >= 5
2. HashMap: đếm tần suất từ trong câu (giống Counter Python)
3. Iterator: từ Vec sản phẩm, lọc giá > 100k, tính tổng giá trị

---

📖 **Trước đó**: [Chương 5](../chuong-05-error-handling/README.md) | **Tiếp theo**: [Chương 7](../chuong-07-traits-lifetimes/README.md)
