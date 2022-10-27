// errors2.rs
//假设我们正在编写一个游戏，您可以在其中使用代币购买物品。所有项目成本
//5 个代币，每当您购买商品时，都会收取 1 的手续费
//令牌。游戏玩家将输入他们想要购买的物品数量，
//`total_cost` 函数将计算令牌的总数。
//但是，由于玩家输入了数量，我们将其作为字符串获取——并且
//他们可能输入了任何东西，而不仅仅是数字！
//现在，这个函数根本没有处理错误情况（而且不是
//正确处理成功案例）。我们要做的是：
//如果我们在一个不是数字的字符串上调用 `parse` 函数，那
//函数将返回一个 `ParseIntError`，在这种情况下，我们想要
//立即从我们的函数中返回该错误，而不是尝试相乘
//并添加。

//至少有两种正确的实现方式——但是
//一个要短得多！执行 `rustlings hint errors2` 以获取两种方式的提示。



use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
