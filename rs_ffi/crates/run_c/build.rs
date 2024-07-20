fn main() {
    // 查询路径为当前目录的 third_c 子目录
    println!("cargo:rustc-link-search=./third_c");
    // 链接的动态库为 add
    println!("cargo:rustc-link-lib=add");
}
