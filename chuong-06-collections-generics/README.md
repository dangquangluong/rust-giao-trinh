# Chương 6: Collections & Generics (list→Vec, dict→HashMap)

## 6.1 Vec = list Python

### 🐍 Python:
```python
diem = [85, 90, 78, 92]
diem.append(100)
diem.sort()
print(sum(diem) / len(diem))
```

### 🦀 Rust:
```rust
fn main() {
    let mut diem = vec![85, 90, 78, 92];
    diem.push(100);
    diem.sort();

    let tong: i32 = diem.iter().sum();
    let tb = tong as f64 / diem.len() as f64;
    println!("TB: {:.1}", tb);
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
diem = {"An": 8.5, "Binh": 7.0, "Cuong": 9.2}
diem["Dung"] = 6.5

for ten, d in diem.items():
    print(f"{ten}: {d}")
```

### 🦀 Rust:
```rust
use std::collections::HashMap;

fn main() {
    let mut diem = HashMap::new();
    diem.insert("An", 8.5);
    diem.insert("Binh", 7.0);
    diem.insert("Cuong", 9.2);
    diem.insert("Dung", 6.5);

    for (ten, d) in &diem {
        println!("{}: {}", ten, d);
    }

    // Truy cập (trả Option vì key có thể không tồn tại)
    if let Some(d) = diem.get("An") {
        println!("Điểm An: {}", d);
    }
}
```

### So sánh:

| Python | Rust |
|--------|------|
| `d["key"]` | `d.get("key")` → `Option` |
| `d["key"] = val` | `d.insert("key", val)` |
| `"key" in d` | `d.contains_key("key")` |
| `del d["key"]` | `d.remove("key")` |
| `for k, v in d.items():` | `for (k, v) in &d {` |

---

## 6.3 Iterator = Giống list comprehension Python

### 🐍 Python:
```python
so = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
chan_bp = [x**2 for x in so if x % 2 == 0]
tong = sum(so)
```

### 🦀 Rust:
```rust
fn main() {
    let so = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Giống list comprehension!
    let chan_bp: Vec<i32> = so.iter()
        .filter(|&&x| x % 2 == 0)   // if x % 2 == 0
        .map(|&x| x * x)            // x**2
        .collect();                   // → Vec

    let tong: i32 = so.iter().sum();  // sum(so)

    println!("Chẵn bình phương: {:?}", chan_bp);
    println!("Tổng: {}", tong);
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
from typing import List, TypeVar
T = TypeVar('T')

def first(lst: List[T]) -> T:
    return lst[0]
```

### 🦀 Rust:
```rust
// Hàm generic - hoạt động với mọi kiểu có thể so sánh
fn tim_max<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in &list[1..] {
        if item > max {
            max = item;
        }
    }
    max
}

fn main() {
    let so = vec![3, 1, 4, 1, 5, 9];
    println!("Max: {}", tim_max(&so));

    let chu = vec!['z', 'a', 'm'];
    println!("Max: {}", tim_max(&chu));
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
