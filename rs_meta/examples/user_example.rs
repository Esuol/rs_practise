#![allow(dead_code)]

extern crate rs_meta;
use rs_meta::User;

fn main() {
    // 创建一个User实例
    let user = User {
        name: String::from("Example User"),
    };

    // 打印User实例
    println!("{:?}", user);
}
