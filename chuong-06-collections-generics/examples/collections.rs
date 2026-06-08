// === PYTHON EQUIVALENT ===
// # Python:
// # diem = [85, 90, 78, 92, 88, 95, 67, 73]
// # print(f"Tổng: {sum(diem)}, TB: {sum(diem)/len(diem):.1f}, Max: {max(diem)}")
// #
// # # Đếm từ (Counter)
// # from collections import Counter
// # cau = "rust là tuyệt rust rất nhanh"
// # dem = Counter(cau.split())
// # print(dem)
// #
// # # List comprehension
// # so = list(range(1, 11))
// # chan_bp = [x**2 for x in so if x % 2 == 0]

// ============================================
// Collections trong Rust
// Vec = list Python
// HashMap = dict Python
// Iterator = list comprehension Python
// ============================================

// use = giống from collections import HashMap
use std::collections::HashMap;

fn main() {
    // =========================================
    // VEC = LIST PYTHON
    // =========================================
    println!("=== VECTOR (giống list Python) ===\n");

    // vec![] = giống [85, 90, 78, ...] Python
    // Vec<i32> = list chỉ chứa số nguyên (Python list chứa bất kỳ)
    let diem: Vec<i32> = vec![85, 90, 78, 92, 88, 95, 67, 73];

    // .iter() = tạo iterator (giống iter() Python)
    // .sum() = giống sum() Python
    let tong: i32 = diem.iter().sum();

    // as f64 = giống float() Python (ép kiểu)
    let tb = tong as f64 / diem.len() as f64;

    // {:?} = debug print (in cả vector ra)
    println!("Điểm: {:?}", diem);
    // .iter().max() trả về Option (vì list có thể rỗng)
    println!("Tổng: {}, TB: {:.1}, Max: {:?}", tong, tb, diem.iter().max());

    // =========================================
    // HASHMAP = DICT PYTHON
    // =========================================
    println!("\n=== HASHMAP (giống dict Python) ===\n");

    let cau = "rust là tuyệt rust rất nhanh";

    // HashMap::new() = giống {} Python
    let mut dem: HashMap<&str, u32> = HashMap::new();

    // .split_whitespace() = giống .split() Python
    for tu in cau.split_whitespace() {
        // .entry(tu) = kiểm tra key có chưa
        // .or_insert(0) = nếu chưa có thì thêm với value = 0
        // * = dereference (lấy giá trị để sửa)
        // Giống Python: dem[tu] = dem.get(tu, 0) + 1
        *dem.entry(tu).or_insert(0) += 1;
    }
    println!("Đếm từ: {:?}", dem);

    // =========================================
    // ITERATOR = LIST COMPREHENSION PYTHON
    // =========================================
    println!("\n=== ITERATOR (giống list comprehension) ===\n");

    let so = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Python: chan_bp = [x**2 for x in so if x % 2 == 0]
    // Rust:   .filter() = if, .map() = biến đổi, .collect() = tạo Vec
    let chan_bp: Vec<i32> = so.iter()      // Tạo iterator
        .filter(|&&x| x % 2 == 0)         // Lọc số chẵn (giống if x%2==0)
        .map(|&x| x * x)                  // Bình phương (giống x**2)
        .collect();                         // Thu gom thành Vec (giống [])

    println!("Chẵn bình phương: {:?}", chan_bp);
    // Kết quả: [4, 16, 36, 64, 100]

    // Thêm ví dụ:
    // Python: total = sum(x for x in so if x > 5)
    let tong_lon_hon_5: i32 = so.iter()
        .filter(|&&x| x > 5)
        .sum();
    println!("Tổng các số > 5: {}", tong_lon_hon_5);  // 40
}
