mod calculator;
mod html_generator;
mod models;
mod scraper;

use calculator::Calculator;
use html_generator::HtmlGenerator;
use std::fs::File;
use std::path::Path;
use tiny_http::{Header, Response, Server};

const SERVER_PORT: u16 = 8080;

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
    let output_dir = "dist";
    if !Path::new(output_dir).exists() {
        std::fs::create_dir_all(output_dir).unwrap();
    }
    let output_path = format!("{}/b50_slides.html", output_dir);

    if let Err(e) = HtmlGenerator::generate(&project, &output_path) {
        eprintln!("HTML生成错误: {}", e);
    } else {
        println!("已生成幻灯片: '{}'", output_path);
    }

    println!("\n完成！请在浏览器打开 'dist/b50_slides.html' 查看效果。");
    println!(
        "提示: 如果爬取元数据失败，请在浏览器保存维基页面为 'wiki.html' 并放在本程序同目录下。"
    );

    // 4. 启动本地服务器
    start_server(SERVER_PORT, "dist");
}

fn start_server(port: u16, root_dir: &str) {
    let addr = format!("0.0.0.0:{}", port);
    let server = match Server::http(&addr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("无法启动服务器: {}", e);
            return;
        }
    };

    let url = format!("http://localhost:{}", port);
    println!("\n正在启动本地服务器...");
    println!("服务地址: {}", url);
    println!("按 Ctrl+C 停止服务");

    // 尝试自动打开浏览器
    let _ = open::that(&url);

    for request in server.incoming_requests() {
        let url = request.url();
        // 简单的路径处理
        let path = if url == "/" { "/b50_slides.html" } else { url };

        // 防止目录遍历
        if path.contains("..") {
            let _ = request.respond(Response::from_string("Invalid path").with_status_code(403));
            continue;
        }

        // 移除 URL 参数（如果有）
        let clean_path = path.split('?').next().unwrap_or(path);

        let file_path = format!("{}{}", root_dir, clean_path);

        if let Ok(file) = File::open(&file_path) {
            let content_type = if file_path.ends_with(".html") {
                "text/html; charset=UTF-8"
            } else if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") {
                "image/jpeg"
            } else if file_path.ends_with(".png") {
                "image/png"
            } else if file_path.ends_with(".json") {
                "application/json"
            } else if file_path.ends_with(".css") {
                "text/css"
            } else if file_path.ends_with(".js") {
                "application/javascript"
            } else {
                "application/octet-stream"
            };

            let header = Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap();
            let response = Response::from_file(file).with_header(header);
            let _ = request.respond(response);
        } else {
            let _ = request.respond(Response::from_string("Not Found").with_status_code(404));
        }
    }
}
