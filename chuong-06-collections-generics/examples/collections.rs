use std::collections::HashMap;

fn main() {
    println!("=== VECTOR ===\n");
    let diem: Vec<i32> = vec![85, 90, 78, 92, 88, 95, 67, 73];
    let tong: i32 = diem.iter().sum();
    let tb = tong as f64 / diem.len() as f64;
    println!("Điểm: {:?}", diem);
    println!("Tổng: {}, TB: {:.1}, Max: {:?}", tong, tb, diem.iter().max());

    println!("\n=== HASHMAP ===\n");
    let cau = "rust là tuyệt rust rất nhanh";
    let mut dem: HashMap<&str, u32> = HashMap::new();
    for tu in cau.split_whitespace() {
        *dem.entry(tu).or_insert(0) += 1;
    }
    println!("Đếm từ: {:?}", dem);

    println!("\n=== ITERATOR ===\n");
    let so = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let chan_bp: Vec<i32> = so.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).collect();
    println!("Chẵn bình phương: {:?}", chan_bp);
}
