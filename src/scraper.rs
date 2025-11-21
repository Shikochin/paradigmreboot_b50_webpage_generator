use crate::models::{CachedMeta, WikiCache};
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Write, copy};
use std::path::Path;
use std::time::Duration;

pub struct Scraper;

impl Scraper {
    /// 创建配置好的 HTTP 客户端
    pub fn create_client() -> Client {
        Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .timeout(Duration::from_secs(30))
            // 强制 HTTP/1.1 修复协议错误
            .http1_only()
            .build()
            .unwrap_or_else(|_| Client::new())
    }

    /// 获取HTML内容：优先网络 -> 其次本地文件
    fn get_html_content() -> String {
        let url = "https://paradigmrebootzh.miraheze.org/wiki/%E6%9B%B2%E7%9B%AE%E5%88%97%E8%A1%A8";

        println!("尝试从网络获取维基页面...");
        let client = Self::create_client();
        match client.get(url).send() {
            Ok(resp) => {
                println!("网络请求成功！");
                return resp.text().unwrap_or_default();
            }
            Err(e) => {
                eprintln!("网络请求失败: {}", e);
                eprintln!("正在尝试读取本地 'wiki.html'...");
            }
        }

        if Path::new("wiki.html").exists() {
            if let Ok(content) = fs::read_to_string("wiki.html") {
                println!("成功加载本地 'wiki.html'。");
                return content;
            }
        }

        String::new()
    }

    /// 下载图片并返回本地路径
    fn download_image(client: &Client, title: &str, url: &str) -> String {
        // 净化文件名，防止非法字符
        let safe_title: String = title
            .chars()
            .map(|x| {
                if x.is_alphanumeric() || x == '-' || x == '_' {
                    x
                } else {
                    '_'
                }
            })
            .collect();

        let ext = if url.to_lowercase().ends_with(".png") {
            "png"
        } else {
            "jpg"
        };
        let filename = format!("covers/{}.{}", safe_title, ext);

        if Path::new(&filename).exists() {
            return filename;
        }

        print!("下载: {} ... ", title);
        let _ = std::io::stdout().flush();

        match client.get(url).send() {
            Ok(mut resp) => {
                if let Ok(mut file) = File::create(&filename) {
                    if copy(&mut resp, &mut file).is_ok() {
                        println!("OK");
                        return filename;
                    }
                }
            }
            Err(_) => {}
        }
        println!("Skipped (Error)");

        // 如果下载失败，返回默认占位符
        "covers/default.jpg".to_string()
    }

    /// 入口：确保获取元数据（优先读取缓存，其次尝试网络，最后尝试本地HTML）
    pub fn ensure_metadata_cache() -> WikiCache {
        let cache_file = "wiki_cache.json";

        // 1. 尝试读取已存在的缓存文件
        if Path::new(cache_file).exists() {
            println!("发现本地缓存 'wiki_cache.json'，正在加载...");
            if let Ok(content) = fs::read_to_string(cache_file) {
                if let Ok(cache) = serde_json::from_str::<WikiCache>(&content) {
                    println!("成功加载 {} 首歌曲的缓存数据。", cache.len());
                    return cache;
                }
            }
            println!("缓存文件损坏，准备重新获取...");
        }

        // 2. 缓存不存在，准备爬取
        println!("正在初始化元数据库（仅需执行一次）...");

        let html_content = Self::get_html_content();
        if html_content.is_empty() {
            eprintln!(
                "错误：无法获取维基数据。请检查网络，或将网页手动保存为 'wiki.html' 放在目录下。"
            );
            return HashMap::new();
        }

        let document = Html::parse_document(&html_content);
        let row_selector = Selector::parse("table.wikitable tr").unwrap();
        let img_selector = Selector::parse("img").unwrap();

        let mut cache = HashMap::new();
        let client = Self::create_client();

        if !Path::new("covers").exists() {
            fs::create_dir("covers").unwrap();
        }

        println!("正在分析表格并下载所有封面（请耐心等待）...");

        for row in document.select(&row_selector) {
            let cells: Vec<_> = row.select(&Selector::parse("td").unwrap()).collect();
            if cells.len() < 3 {
                continue;
            }

            let title = cells[1].text().collect::<String>().trim().to_string();
            let artist = cells[2].text().collect::<String>().trim().to_string();

            if title.is_empty() {
                continue;
            }

            // 解析图片 URL
            let img_url = cells[0]
                .select(&img_selector)
                .next()
                .and_then(|img| img.value().attr("src"))
                .map(|src| {
                    let src = if src.starts_with("//") {
                        format!("https:{}", src)
                    } else {
                        src.to_string()
                    };
                    // 去除 /thumb/ 和 /80px- 等缩略图标记，获取原图
                    if src.contains("/thumb/") {
                        let parts: Vec<&str> = src.split("/thumb/").collect();
                        if parts.len() > 1 {
                            let tail = parts[1];
                            let tail_parts: Vec<&str> = tail.split('/').collect();
                            if tail_parts.len() > 1 {
                                let original_tail = tail_parts[..tail_parts.len() - 1].join("/");
                                format!("{}/{}", parts[0], original_tail)
                            } else {
                                src
                            }
                        } else {
                            src
                        }
                    } else {
                        src
                    }
                })
                .unwrap_or_default();

            let local_path = if !img_url.is_empty() {
                Self::download_image(&client, &title, &img_url)
            } else {
                "covers/default.jpg".to_string()
            };

            cache.insert(
                title,
                CachedMeta {
                    artist,
                    local_cover_path: local_path,
                },
            );
        }

        // 3. 保存缓存到文件
        if let Ok(json_str) = serde_json::to_string_pretty(&cache) {
            let _ = fs::write(cache_file, json_str);
            println!("元数据已缓存至 '{}'，下次运行将跳过下载。", cache_file);
        }

        cache
    }
}
