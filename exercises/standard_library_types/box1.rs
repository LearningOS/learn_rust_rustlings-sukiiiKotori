// box1.rs
//
//在编译时，Rust 需要知道一个类型占用了多少空间。这变得有问题
//对于递归类型，一个值可以有另一个相同类型的值作为其自身的一部分。
//为了解决这个问题，我们可以使用 `Box` -一个用于在堆上存储数据的智能指针，
//这也允许我们包装一个递归类型。
//
//我们在本练习中实现的递归类型是 `cons list` -一种数据结构
//经常出现在函数式编程语言中。 cons 列表中的每个项目都包含两个
//元素：当前项和下一项的值。最后一项是一个名为“Nil”的值。
//
//第 1 步：在枚举定义中使用 `Box` 使代码编译
//第 2 步：通过替换 `unimplemented!()` 创建空的和非空的 cons 列表
//
//注意：不应更改测试
//
//为提示执行 `rustlings hint box1` :)



#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(0, Box::new(List::Nil))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
