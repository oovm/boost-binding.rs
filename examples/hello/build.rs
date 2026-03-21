fn main() {
    // 使用 itools 库的 build_with_resource 函数来处理资源编译
    itools::build_with_resource("assets/icon.ico");
    
    // 确保图标文件变化时重新构建
    println!("cargo:rerun-if-changed=assets/icon.ico");
    println!("cargo:rerun-if-changed=build.rs");
}