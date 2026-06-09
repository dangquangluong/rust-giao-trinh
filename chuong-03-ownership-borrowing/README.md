# Chương 3: Ownership & Borrowing

> ⚠️ Đây là chương QUAN TRỌNG NHẤT và cũng là thứ **Python hoàn toàn KHÔNG CÓ**. Đọc chậm, đọc nhiều lần!

## 🐍 Trong Python: Bạn Không Cần Lo Về Bộ Nhớ

```python
# Python - mọi thứ "just works"
a = "hello"      # Tạo biến a trỏ đến "hello"
b = a            # b trỏ đến cùng object, Python tự quản lý reference count
print(a)         # OK - a vẫn dùng được
print(b)         # OK - b cũng dùng được
# Python có Garbage Collector tự dọn rác khi không ai trỏ đến object nữa
```

## 🦀 Trong Rust: Bạn Phải Hiểu Ai "Sở Hữu" Dữ Liệu

Rust KHÔNG có Garbage Collector. Thay vào đó dùng hệ thống **Ownership** (quyền sở hữu).

---

## 3.1 Ownership Là Gì? (Giải Thích Bằng Đời Thường)

Hãy tưởng tượng **dữ liệu = quyển sách**:

- **Python**: Mọi người có thể photocopy quyển sách thoải mái. Có người dọn dẹp (GC) đi thu gom sách không ai đọc.
- **Rust**: Mỗi quyển sách chỉ có **MỘT chủ nhân**. Muốn cho người khác thì phải **CHO** (move) hoặc **CHO MƯỢN** (borrow).

### Ba Quy Tắc (Nhớ thuộc lòng!)

| # | Quy tắc | Ví dụ đời thường |
|---|---------|-----------------|
| 1 | Mỗi giá trị có **đúng 1 owner** | Mỗi quyển sách có 1 chủ |
| 2 | Chỉ có **1 owner tại 1 thời điểm** | Không thể 2 người cùng sở hữu 1 quyển |
| 3 | Khi owner "biến mất", giá trị bị **xóa** | Chủ đi rồi thì sách bị vứt |

---

## 3.2 Move - "Cho" Quyền Sở Hữu

### Python (không có khái niệm này):
```python
a = "hello"        # a trỏ đến "hello"
b = a              # b và a CÙNG trỏ đến "hello" (Python cho phép)
print(a)           # ✅ Vẫn OK! Python dùng reference counting
print(b)           # ✅ OK
```

### Rust (KHÁC Python hoàn toàn!):
```rust
fn main() {                              // Hàm chính
    let a = String::from("hello");       // String::from() = tạo String trên heap
                                         // Giống a = "hello" Python nhưng Rust phân biệt heap/stack
    let b = a;                           // a đã "CHO" (move) quyền sở hữu cho b
                                         // Python: b = a thì cả hai vẫn dùng được
                                         // Rust: a KHÔNG còn hợp lệ nữa!
    // println!("{}", a);                // ❌ LỖI! a không còn sở hữu gì nữa (đã move cho b)
    println!("{}", b);                   // ✅ b là chủ mới, chỉ b mới dùng được
}                                        // } kết thúc main, b bị drop (giải phóng bộ nhớ)
```

### 🤔 Tại Sao Rust Làm Vậy?

> Để **tránh 2 biến cùng xóa 1 vùng nhớ** (double free bug). Python dùng GC để đếm reference, Rust dùng ownership.

### 📌 Mẹo nhớ:
> Move = **tặng quà**. Bạn tặng sách cho bạn bè thì bạn không còn quyển sách đó nữa.

---

## 3.3 Clone - "Photocopy"

Nếu muốn giữ lại bản gốc? Dùng `.clone()` (giống copy.deepcopy trong Python):

```rust
fn main() {                              // Hàm chính
    let a = String::from("hello");       // Tạo String "hello" (a là owner)
    let b = a.clone();                   // .clone() = tạo bản sao HOÀN TOÀN mới trên heap
                                         // Giống copy.deepcopy(a) trong Python
                                         // Giờ a và b là 2 String riêng biệt
    println!("a = {}", a);              // ✅ a vẫn OK (giữ bản gốc, không bị move)
    println!("b = {}", b);              // ✅ b là bản copy độc lập
}
```

> 💡 **Mẹo:** Khi mới học, gặp lỗi ownership thì thêm `.clone()` là chạy. Sau khi hiểu rõ thì tối ưu sau.

---

## 3.4 Copy - Kiểu Đơn Giản Tự Copy

Số, bool, char **tự động copy** (không cần .clone()):

```rust
fn main() {                                      // Hàm chính
    let x = 42;                                  // i32 - kiểu đơn giản, nằm trên stack
    let y = x;                                   // COPY tự động (KHÔNG phải move!)
                                                 // Vì i32 nhỏ gọn, copy rẻ, Rust tự làm
                                                 // Giống Python: y = x với int (cả hai dùng được)
    println!("x = {}, y = {}", x, y);           // ✅ Cả hai OK! x không bị move
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
def tinh_do_dai(s):        # Python tự truyền reference
    return len(s)          # Dùng xong trả về, s vẫn tồn tại ở ngoài

chuoi = "hello"
print(tinh_do_dai(chuoi))  # OK - truyền vào hàm
print(chuoi)                # Vẫn dùng được - Python không "lấy mất"
```

**Rust** - nếu không dùng reference sẽ mất ownership:
```rust
fn tinh_do_dai(s: String) -> usize {     // Nhận String (move vào hàm, hàm trở thành owner)
    s.len()                               // Trả về độ dài
}                                         // s bị drop (giải phóng) ở đây! Vì hàm là owner

fn main() {                               // Hàm chính
    let chuoi = String::from("hello");    // Tạo String
    let len = tinh_do_dai(chuoi);         // chuoi bị MOVE vào hàm (mất quyền sở hữu!)
    // println!("{}", chuoi);             // ❌ LỖI! chuoi không còn (đã move vào hàm)
    println!("Độ dài: {}", len);          // Chỉ dùng được len
}
```

### Giải pháp: Dùng Reference `&` (Cho Mượn)

```rust
fn tinh_do_dai(s: &String) -> usize {    // &String = "mượn" String (không lấy ownership)
                                          // Giống Python truyền tham số: hàm dùng nhưng không sở hữu
    s.len()                               // Dùng bình thường, trả về độ dài
}                                         // s hết scope nhưng KHÔNG drop (vì chỉ mượn, không sở hữu)

fn main() {                               // Hàm chính
    let chuoi = String::from("hello");    // Tạo String (main là owner)
    let len = tinh_do_dai(&chuoi);        // &chuoi = "cho mượn" (truyền reference, không move)
                                          // & = "tôi cho mượn, nhưng tôi vẫn là chủ"
    println!("'{}' dài {} ký tự", chuoi, len);  // ✅ chuoi vẫn dùng được! (vì chỉ cho mượn)
}
```

> 📌 **Borrow = Cho mượn sách.** Bạn bè mượn đọc nhưng phải trả lại. Bạn vẫn là chủ.

---

## 3.6 Mutable Reference - "Cho Mượn Để Sửa"

**Python:**
```python
def them_text(s):          # Python truyền reference của object
    s.append(" World!")    # Sửa trực tiếp list (vì list là mutable object)

lst = ["Hello"]
them_text(lst)             # Truyền vào hàm
print(lst)                 # ["Hello", " World!"] - đã bị sửa
```

**Rust:**
```rust
fn them_text(s: &mut String) {           // &mut = reference cho phép SỬA (mutable borrow)
                                          // Giống Python truyền mutable object vào hàm
    s.push_str(" World!");               // push_str = nối thêm chuỗi (giống += trong Python)
}                                         // Hết hàm, trả lại quyền cho owner

fn main() {                               // Hàm chính
    let mut chuoi = String::from("Hello"); // mut = cho phép thay đổi (BẮT BUỘC khi muốn &mut)
    them_text(&mut chuoi);                // &mut = cho mượn ĐỂ SỬA
                                          // Phải ghi rõ &mut (Rust bắt bạn biết mình đang làm gì)
    println!("{}", chuoi);               // "Hello World!" - đã bị sửa bởi hàm
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
fn main() {                                  // Hàm chính
    let mut s = String::from("hello");       // mut String (cần mut vì sẽ tạo &mut sau)

    // ✅ OK: nhiều immutable reference cùng lúc
    let r1 = &s;                             // r1 mượn đọc (immutable borrow)
    let r2 = &s;                             // r2 cũng mượn đọc - OK, nhiều người đọc được
    println!("{}, {}", r1, r2);              // Dùng cả hai OK

    // ✅ OK: 1 mutable reference (sau khi r1, r2 không dùng nữa)
    let r3 = &mut s;                         // r3 mượn để sửa (mutable borrow)
                                             // Chỉ 1 &mut tại 1 thời điểm!
    r3.push_str(" world");                   // Sửa thông qua mutable reference
    println!("{}", r3);                      // In: "hello world"
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

## 3.9 Bài Tập (Dễ thành Khó)

### Bài 1: Sửa lỗi (dễ)
```rust
fn main() {                              // Hàm chính
    let s = String::from("hello");       // Tạo String
    let s2 = s;                          // s bị move cho s2
    println!("{}", s);                   // ← LỖI! s đã move - Sửa để chạy được
}
```
> Gợi ý: dùng `.clone()` hoặc `&`

### Bài 2: Viết hàm dùng reference
Viết hàm `dem_ky_tu(s: &str) -> usize` đếm số ký tự (không phải bytes).

### Bài 3: Mutable reference
Viết hàm `viet_hoa(s: &mut String)` biến chuỗi thành chữ IN HOA.

---

📖 **Trước đó**: [Chương 2](../chuong-02-cu-phap-co-ban/README.md) | **Tiếp theo**: [Chương 4](../chuong-04-struct-enum/README.md)
