# Bài Tập Chương 7: Traits & Lifetimes

## Bài 1: Trait Printable (⭐⭐ Trung bình)

**Yêu cầu:** Tạo trait `Printable` với method `to_string(&self) -> String`. Implement cho `SinhVien` và `SanPham`.

---

## Bài 2: Trait Object (⭐⭐⭐ Khó)

**Yêu cầu:** Tạo trait `HinhHoc` (dien_tich, ten). Tạo Vec<Box<dyn HinhHoc>> chứa nhiều loại hình, tính tổng diện tích.

---

## Bài 3: Generic hàm (⭐⭐ Trung bình)

**Yêu cầu:** Viết hàm generic `in_list<T: std::fmt::Display>(items: &[T])` in từng phần tử có đánh số.
