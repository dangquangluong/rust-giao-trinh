# Bài Tập Chương 4: Struct & Enum

## Bài 1: Hình chữ nhật (⭐ Dễ)

**Yêu cầu:** Tạo struct `HinhChuNhat` với fields `rong` và `dai` (f64). Thêm methods:
- `dien_tich(&self) -> f64`
- `chu_vi(&self) -> f64`
- `la_hinh_vuong(&self) -> bool`

---

## Bài 2: Enum phương tiện (⭐⭐ Trung bình)

**Yêu cầu:** Tạo enum:
```
PhuongTien {
    XeDap,
    XeMay(String),              // tên hãng
    OTo { hang: String, so_cho: u8 }
}
```
Viết hàm `mo_ta(pt: &PhuongTien)` in thông tin bằng `match`.

---

## Bài 3: Option - Chia an toàn (⭐⭐ Trung bình)

**Yêu cầu:** Viết hàm `chia(a: f64, b: f64) -> Option<f64>` trả `None` khi b=0. Dùng `match` để in kết quả.

---

## Bài 4: Danh bạ (⭐⭐⭐ Khó)

**Yêu cầu:** Tạo struct `LienHe { ten: String, sdt: String, email: Option<String> }`. Tạo Vec<LienHe>, viết hàm tìm kiếm theo tên (trả Option<&LienHe>).
