use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 获取目标目录
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable is not defined");
    let debug_dir = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    // 定义源目录和目标目录
    let src_dir = Path::new("resources");
    let target_dir = debug_dir.join("resources");

    // 递归复制文件夹
    if let Err(e) = copy_dir_all(src_dir, &target_dir) {
        panic!("Failed to copy directory: {}", e);
    }

    println!("cargo:rerun-if-changed=resources");
}

// 递归复制文件夹的函数
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let target_path = dst.join(entry.file_name());

        if entry_path.is_dir() {
            copy_dir_all(&entry_path, &target_path)?;
        } else {
            fs::copy(&entry_path, &target_path)?;
        }
    }
    Ok(())
}
