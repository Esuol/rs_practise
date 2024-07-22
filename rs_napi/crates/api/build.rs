fn main() {
    // 告诉 cargo 在编译阶段需要链接到动态链接库 libace_napi.z.so
    println!("cargo:rustc-link-lib=dylib=ace_napi.z");
}
