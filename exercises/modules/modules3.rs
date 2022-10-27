//模块3.rs
//您可以使用 'use' 关键字从任何地方的模块中获取模块路径
//特别是从 Rust 标准库到你的范围。
//带上 SystemTime 和 UNIX_EPOCH
//来自 std::time 模块。如果你能用一条线做到这一点，就可以获得额外的风格点数！
//让我编译！执行 `rustlings hint modules3` 获取提示 :)



// TODO: Complete this use statement
use std::time::{SystemTime,UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
