//一所虚构的魔法学校有一个用 Rust 编写的新成绩单生成系统！
//目前系统只支持创建学生成绩的成绩单
//以数字表示（例如 1.0 -> 5.5）。
//但是，学校也会发布字母等级（A+ -> F-）和需要
//能够打印两种类型的成绩单！

//在 struct ReportCard 和 impl 块中进行必要的代码更改
//支持按字母顺序排列的成绩单。将第二次测试中的成绩更改为“A+”
//表明您的更改允许按字母顺序评分。

//执行 'rustlings hint generics3' 获取提示！



pub struct ReportCard <T>{
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: std::fmt::Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
