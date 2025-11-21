mod calculator;
mod generator;
mod models;
mod scraper;

use calculator::Calculator;
use generator::html_generator::HtmlGenerator;
use generator::script_generator::ScriptGenerator;
use std::path::Path;

fn main() {
    println!("Paradigm: Reboot B50 Generator (Modularized Game UI Edition)");
    let csv_path = "records.csv";

    // 1. 检查输入文件
    if !Path::new(csv_path).exists() {
        eprintln!("错误: 未找到 '{}'。请确保CSV文件在当前目录下。", csv_path);
        return;
    }

    // 2. 加载数据并计算 B50
    let project = match Calculator::load_and_calculate(csv_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("数据处理错误: {}", e);
            return;
        }
    };

    println!("成功加载 {} 条记录。", project.records.len());

    // 3. 生成 HTML 幻灯片
    if let Err(e) = HtmlGenerator::generate(&project, "b50_slides.html") {
        eprintln!("HTML生成错误: {}", e);
    } else {
        println!("已生成幻灯片: 'b50_slides.html'");
    }

    // 4. 生成渲染脚本
    if let Err(e) = ScriptGenerator::generate(&project) {
        eprintln!("脚本生成错误: {}", e);
    } else {
        println!("已生成渲染脚本: 'render.sh'");
    }

    println!("\n完成！请在浏览器打开 'b50_slides.html' 查看效果。");
    println!(
        "提示: 如果爬取元数据失败，请在浏览器保存维基页面为 'wiki.html' 并放在本程序同目录下。"
    );
}
