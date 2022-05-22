use std::{fs, io::Write};

fn main() {
    println!("-Enter the account ID you wish to calculate for-");
    let mut id = String::new();
    std::io::stdin().read_line(&mut id).unwrap();
    let id = id.trim();

    println!("-Enter dummy data-");
    let mut dd = String::new();
    std::io::stdin().read_line(&mut dd).unwrap();
    let dd = dd.trim();

    let mut file = std::fs::File::create(id).expect("create failed");
    file.write_all(dd.as_bytes()).expect("write failed");
    
    let dir = "Logs";
    fs::create_dir_all(dir).unwrap();
    std::fs::rename(id, format!("{dir}/{id}.txt")).unwrap();
}
