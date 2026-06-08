# Chương 3: Ownership & Borrowing

> ⚠️ Đây là chương QUAN TRỌNG NHẤT và cũng là thứ **Python hoàn toàn KHÔNG CÓ**. Đọc chậm, đọc nhiều lần!

## 🐍 Trong Python: Bạn Không Cần Lo Về Bộ Nhớ

```python
# Python - mọi thứ "just works"
a = "hello"
b = a        # b trỏ đến cùng object, Python tự quản lý
print(a)     # OK
print(b)     # OK
# Python có Garbage Collector tự dọn rác
```

## 🦀 Trong Rust: Bạn Phải Hiểu Ai "Sở Hữu" Dữ Liệu

Rust KHÔNG có Garbage Collector. Thay vào đó dùng hệ thống **Ownership** (quyền sở hữu).

---

## 3.1 Ownership Là Gì? (Giải Thích Bằng Đời Thường)

Hãy tưởng tượng **dữ liệu = quyển sách**:

- **Python**: Mọi người có thể photocopy quyển sách thoải mái. Có người dọn dẹp (GC) đi thu gom sách không ai đọc.
- **Rust**: Mỗi quyển sách chỉ có **MỘT chủ nhân**. Muốn cho người khác → phải **CHO** (move) hoặc **CHO MƯỢN** (borrow).

### Ba Quy Tắc (Nhớ thuộc lòng!)

| # | Quy tắc | Ví dụ đời thường |
|---|---------|-----------------|
| 1 | Mỗi giá trị có **đúng 1 owner** | Mỗi quyển sách có 1 chủ |
| 2 | Chỉ có **1 owner tại 1 thời điểm** | Không thể 2 người cùng sở hữu 1 quyển |
| 3 | Khi owner "biến mất", giá trị bị **xóa** | Chủ đi rồi → sách bị vứt |

---

## 3.2 Move - "Cho" Quyền Sở Hữu

### Python (không có khái niệm này):
```python
a = "hello"
b = a          # b và a CÙNG trỏ đến "hello"
print(a)       # ✅ Vẫn OK! Python cho phép
print(b)       # ✅ OK
```

### Rust (KHÁC Python hoàn toàn!):
```rust
fn main() {
    let a = String::from("hello");
    let b = a;       // a đã "CHO" (move) cho b
    // println!("{}", a);  // ❌ LỖI! a không còn sở hữu gì nữa
    println!("{}", b);     // ✅ b là chủ mới
}
```

### 🤔 Tại Sao Rust Làm Vậy?

> Để **tránh 2 biến cùng xóa 1 vùng nhớ** (double free bug). Python dùng GC để đếm reference, Rust dùng ownership.

### 📌 Mẹo nhớ:
> Move = **tặng quà**. Bạn tặng sách cho bạn bè → bạn không còn quyển sách đó nữa.

---

## 3.3 Clone - "Photocopy"

Nếu muốn giữ lại bản gốc? Dùng `.clone()` (giống copy.deepcopy trong Python):

```rust
fn main() {
    let a = String::from("hello");
    let b = a.clone();     // Tạo bản sao hoàn toàn
    println!("a = {}", a); // ✅ a vẫn OK (giữ bản gốc)
    println!("b = {}", b); // ✅ b là bản copy
}
```

> 💡 **Mẹo:** Khi mới học, gặp lỗi ownership → thêm `.clone()` là chạy. Sau khi hiểu rõ thì tối ưu sau.

---

## 3.4 Copy - Kiểu Đơn Giản Tự Copy

Số, bool, char **tự động copy** (không cần .clone()):

```rust
fn main() {
    let x = 42;      // i32 - kiểu đơn giản
    let y = x;       // COPY (không phải move!)
    println!("x = {}, y = {}", x, y); // ✅ Cả hai OK!
}
```

| Kiểu | Move hay Copy? | Tương đương Python |
|------|---------------|-------------------|
| `i32`, `f64`, `bool`, `char` | **Copy** (tự động) | int, float, bool |
| `String`, `Vec`, `HashMap` | **Move** | str, list, dict |

> 📌 **Quy tắc đơn giản:** Kiểu nhỏ (nằm trên stack) = Copy. Kiểu lớn (nằm trên heap) = Move.

---

## 3.5 References & Borrowing - "Cho Mượn"

### Vấn đề: Muốn truyền vào hàm mà không mất ownership?

**Python** - không có vấn đề gì:
```python
def tinh_do_dai(s):
    return len(s)

chuoi = "hello"
print(tinh_do_dai(chuoi))  # OK
print(chuoi)                # Vẫn dùng được
```

**Rust** - nếu không dùng reference sẽ mất ownership:
```rust
fn tinh_do_dai(s: String) -> usize {
    s.len()
}  // s bị drop ở đây!

fn main() {
    let chuoi = String::from("hello");
    let len = tinh_do_dai(chuoi);  // chuoi bị move vào hàm
    // println!("{}", chuoi);       // ❌ LỖI! chuoi không còn
}
```

### Giải pháp: Dùng Reference `&` (Cho Mượn)

```rust
fn tinh_do_dai(s: &String) -> usize {  // &String = "mượn" string
    s.len()
}

fn main() {
    let chuoi = String::from("hello");
    let len = tinh_do_dai(&chuoi);     // &chuoi = "cho mượn"
    println!("'{}' dài {} ký tự", chuoi, len); // ✅ Vẫn dùng được!
}
```

> 📌 **Borrow = Cho mượn sách.** Bạn bè mượn đọc nhưng phải trả lại. Bạn vẫn là chủ.

---

## 3.6 Mutable Reference - "Cho Mượn Để Sửa"

**Python:**
```python
def them_text(s):
    s.append(" World!")  # Sửa trực tiếp

lst = ["Hello"]
them_text(lst)
print(lst)  # ["Hello", " World!"]
```

**Rust:**
```rust
fn them_text(s: &mut String) {    // &mut = cho mượn ĐỂ SỬA
    s.push_str(" World!");
}

fn main() {
    let mut chuoi = String::from("Hello");  // phải khai báo mut
    them_text(&mut chuoi);                   // cho mượn để sửa
    println!("{}", chuoi);                   // "Hello World!"
}
```

---

## 3.7 Quy Tắc Borrowing (3 luật quan trọng!)

| Quy tắc | Giải thích | Ví dụ đời thường |
|---------|------------|-----------------|
| Nhiều `&` cùng lúc: ✅ | Nhiều người cùng đọc sách: OK | Thư viện cho nhiều người đọc |
| Chỉ 1 `&mut` tại 1 thời điểm: ✅ | Chỉ 1 người được viết/sửa | Chỉ 1 người cầm bút sửa |
| Không vừa `&` vừa `&mut`: ❌ | Không vừa đọc vừa sửa | Đang đọc thì đừng xé trang! |

```rust
fn main() {
    let mut s = String::from("hello");

    // ✅ OK: nhiều immutable reference
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    // ✅ OK: 1 mutable reference
    let r3 = &mut s;
    r3.push_str(" world");
    println!("{}", r3);
}
```

---

## 3.8 Tóm Tắt Bằng Bảng So Sánh

| Hành động | Python | Rust |
|-----------|--------|------|
| Gán biến (kiểu phức) | Cả hai dùng được | Move (biến cũ mất) |
| Muốn giữ cả hai | Tự động | `.clone()` |
| Truyền vào hàm | Tự do | `&` (borrow) hoặc move |
| Sửa trong hàm | Truyền object là sửa được | `&mut` |
| Dọn bộ nhớ | GC tự dọn | Tự dọn khi owner hết scope |

---

## 3.9 Bài Tập (Dễ → Khó)

### Bài 1: Sửa lỗi (dễ)
```rust
fn main() {
    let s = String::from("hello");
    let s2 = s;
    println!("{}", s); // ← Sửa để chạy được
}
```
> Gợi ý: dùng `.clone()` hoặc `&`

### Bài 2: Viết hàm dùng reference
Viết hàm `dem_ky_tu(s: &str) -> usize` đếm số ký tự (không phải bytes).

### Bài 3: Mutable reference
Viết hàm `viet_hoa(s: &mut String)` biến chuỗi thành chữ IN HOA.

---

📖 **Trước đó**: [Chương 2](../chuong-02-cu-phap-co-ban/README.md) | **Tiếp theo**: [Chương 4](../chuong-04-struct-enum/README.md)
