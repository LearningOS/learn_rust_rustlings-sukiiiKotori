// errors3.rs
//这是一个试图使用完整版本的程序
//上一个练习中的 `total_cost` 函数。但它不工作！
//为什么不？我们应该怎么做才能修复它？
//执行 `rustlings hint errors3` 获取提示！



use std::num::ParseIntError;

fn main()->Result<(), ParseIntError>{
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
