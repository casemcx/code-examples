use colored::*;

pub fn print_gradient(word: &str, start_color: (u8, u8, u8), end_color: (u8, u8, u8)) {
    for (i, char) in word.chars().enumerate() {
        // 计算当前字符的颜色
        let ratio = i as f32 / word.len() as f32;
        let r = interpolation(start_color.0, end_color.0, ratio) as u8;
        let g = interpolation(start_color.1, end_color.1, ratio) as u8;
        let b = interpolation(start_color.2, end_color.2, ratio) as u8;

        // 应用颜色并打印字符
        print!("{}", format!("{}", char).truecolor(r, g, b));
    }
    println!(); // 新行
}

pub fn print_title() {
    let title = "Hello, thank can use Front Scaffold!";
    let start_color = (255, 0, 0);
    let end_color: (u8, u8, u8) = (148, 0, 211);
    print_gradient(title, start_color, end_color);
}

// 线性插值函数
fn interpolation(start: u8, end: u8, ratio: f32) -> f32 {
    (1.0 - ratio) * start as f32 + ratio * end as f32
}