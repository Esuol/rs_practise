use clap::Parser;
use std::fs;

/// 定义一个 clap 的结构体，用于声明参数
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 源文件地址
    #[arg(short = 's', long)]
    source_file: String,
    /// 目标文件地址
    #[arg(short = 'd', long)]
    dest_file: String,
}

fn main() {
    // 调用 clap 的方法解析命令行参数
    let args = Args::parse();
    // 将解析到参数分别获取
    let source_file = args.source_file;
    let dest_file = args.dest_file;
    // // 使用获取到的参数 执行最后的逻辑
    fs::copy(source_file, dest_file).unwrap();
}
