use crate::models::{B50Project, CsvRow, Difficulty, ScoreRecord, WikiCache};
use crate::scraper::Scraper;
use std::path::Path;

pub struct Calculator;

const PLAYER_NAME: &str = "Player";

impl Calculator {
    /// Rating 计算公式
    /// 1.000m ~ 1.009m: Slope 1500
    /// 1.009m ~ 1.010m: Slope 250
    fn calculate_rating(level: f32, score: u32) -> f64 {
        let lv = level as f64;
        let s = score as f64;

        if score >= 1_009_000 {
            let base_level = lv * 10.0;
            let base_bonus = 6.0; // (1009000 - 1000000) / 1500
            let extra_bonus = (s - 1_009_000.0) / 250.0;
            ((base_level + base_bonus + extra_bonus) * 100.0).floor() / 100.0
        } else if score >= 1_000_000 {
            let base_level = lv * 10.0;
            let bonus = (s - 1_000_000.0) / 1500.0;
            ((base_level + bonus) * 100.0).floor() / 100.0
        } else {
            let ratio = s / 1_000_000.0;
            ((lv * 10.0 * ratio) * 100.0).floor() / 100.0
        }
    }

    /// 从 CSV 文件加载数据并计算 B50 结果
    pub fn load_and_calculate(path: &str) -> Result<B50Project, Box<dyn std::error::Error>> {
        if !Path::new(path).exists() {
            return Err(format!("Error: CSV file not found at '{}'", path).into());
        }

        let mut rdr = csv::Reader::from_path(path)?;
        let mut records = Vec::new();

        // 确保元数据缓存
        let wiki_cache: WikiCache = Scraper::ensure_metadata_cache();

        for result in rdr.deserialize() {
            let row: CsvRow = match result {
                Ok(r) => r,
                Err(_) => continue,
            };

            let score_val = match row.score {
                Some(s) => s,
                None => continue,
            };

            let acc = score_val as f64 / 10000.0;
            let rating = Self::calculate_rating(row.level, score_val);
            let diff_enum = Difficulty::from_str(&row.difficulty);

            // 从缓存中查找元数据（包含 is_new 标记）
            let (artist, cover_url, is_new) = if let Some(meta) = wiki_cache.get(&row.title) {
                (
                    meta.artist.clone(),
                    meta.local_cover_path.clone(),
                    meta.is_new,
                )
            } else {
                (
                    "Unknown Artist".to_string(),
                    "covers/default.jpg".to_string(),
                    false,
                )
            };

            let record = ScoreRecord {
                song_metadata: crate::models::SongMetadata {
                    song_id: row.song_level_id,
                    title: row.title,
                    artist,
                    cover_url,
                    bilibili_av: None,
                    is_new,
                },
                difficulty: diff_enum,
                level: row.level,
                score: score_val,
                acc: (acc * 100.0).round() / 100.0,
                rating: (rating * 100.0).round() / 100.0,
                comment: None,
                display_duration_sec: 5,
            };
            records.push(record);
        }

        // 排序并截断至 B50
        records.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap());
        records.truncate(51);

        Ok(B50Project {
            player_name: PLAYER_NAME.to_string(),
            generated_at: chrono::Local::now().format("%Y-%m-%d").to_string(),
            records,
        })
    }
}
