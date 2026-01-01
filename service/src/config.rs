use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryRules {
    #[serde(default)]
    pub distracting: Vec<String>,
    #[serde(default)]
    pub productive: Vec<String>,
    #[serde(default)]
    pub recovery: Vec<String>,
}

impl Default for CategoryRules {
    fn default() -> Self {
        CategoryRules {
            distracting: vec!["game".into(), "netflix".into()],
            productive: vec!["code".into(), "terminal".into()],
            recovery: vec!["music".into()],
        }
    }
}

pub fn load_rules() -> CategoryRules {
    // Look for categories.json next to the executable
    let exe = std::env::current_exe().unwrap_or_default();
    let dir = exe.parent().unwrap_or(Path::new("."));
    // Also check ../../../categories.json for dev mode if needed, but strict is next to exe.
    // For dev mode convenience we can check CWD too.
    
    let paths = [
        dir.join("categories.json"),
        PathBuf::from("categories.json"),
        dir.join("../../../categories.json"), // cargo run location check
    ];

    for p in &paths {
        if p.exists() {
            if let Ok(f) = File::open(p) {
                let reader = BufReader::new(f);
                if let Ok(rules) = serde_json::from_reader(reader) {
                    return rules;
                }
            }
        }
    }

    // Default if missing (Constraint: System must work)
    CategoryRules::default()
}
