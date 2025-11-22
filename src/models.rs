use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 歌曲难度枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Difficulty {
    Detected, // Cyan
    Invaded,  // Red/Pink
    Massive,  // Purple
    Fate,     // White/Deep Purple
    Unknown(String),
}

impl Difficulty {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Detected" => Difficulty::Detected,
            "Invaded" => Difficulty::Invaded,
            "Massive" => Difficulty::Massive,
            "Fate" => Difficulty::Fate,
            _ => Difficulty::Unknown(s.to_string()),
        }
    }

    pub fn to_color(&self) -> &str {
        match self {
            Difficulty::Detected => "#3fcbff",
            Difficulty::Invaded => "#ff6b6b",
            Difficulty::Massive => "#8f629d",
            Difficulty::Fate => "#eeeeee",
            Difficulty::Unknown(_) => "#888888",
        }
    }
}

/// 歌曲元数据（来自维基）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SongMetadata {
    pub song_id: String,
    pub title: String,
    pub artist: String,
    pub cover_url: String, // Will point to local path
    pub bilibili_av: Option<String>,
    #[serde(default)]
    pub is_new: bool,
}

/// B50 单曲记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScoreRecord {
    pub song_metadata: SongMetadata,
    pub difficulty: Difficulty,
    pub level: f32,
    pub score: u32,
    pub acc: f64,
    pub rating: f64,
    pub comment: Option<String>,
    pub display_duration_sec: u64,
}

/// 完整的 B50 项目数据
#[derive(Debug, Serialize, Deserialize)]
pub struct B50Project {
    pub player_name: String,
    pub generated_at: String,
    pub records: Vec<ScoreRecord>,
}

/// CSV 输入行结构
#[derive(Debug, Deserialize)]
pub struct CsvRow {
    pub song_level_id: String,
    pub title: String,
    #[allow(dead_code)]
    pub version: String,
    pub difficulty: String,
    pub level: f32,
    pub score: Option<u32>,
}

/// 缓存的维基元数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CachedMeta {
    pub artist: String,
    pub local_cover_path: String,
    #[serde(default)]
    pub is_new: bool,
}

/// 维基缓存类型别名：Title -> Metadata
pub type WikiCache = HashMap<String, CachedMeta>;
