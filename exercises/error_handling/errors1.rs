//errors1.rs
//此函数拒绝生成要打印在名称标签上的文本，如果
//你传给它一个空字符串。如果能说明问题就更好了
//是，而不是有时返回 `None`。值得庆幸的是，Rust 也有类似的
//构造 `Option` 可用于表达错误条件。让我们使用它！
//执行 `rustlings hint errors1` 获取提示！



pub fn generate_nametag_text(name: String) -> Option<String> {
    if name.len() > 0 {
        Some(format!("Hi! My name is {}", name))
    } else {
        // Empty names aren't allowed.
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()).ok_or("`name` was empty; it must be nonempty."),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()).ok_or("`name` was empty; it must be nonempty."),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
