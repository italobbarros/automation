use std::process::Command;
use std::fs;
use serde_json::Value;

pub fn open_safari_with_cmd(cmd: &str) {
    let map: Value = read_map_json().expect("Unable to parse map.json");
    if let Some(url) = map[cmd]["url"].as_str() {
        let status = Command::new("open")
            .arg("-a")
            .arg("Safari")
            .arg(url)
            .status()
            .expect("Failed to open Safari");

        if status.success() {
            println!("Successfully opened Safari with URL: {}", url);
        } else {
            eprintln!("Failed to open Safari with URL: {}", url);
        }
    } else {
        eprintln!("Command not found in map.json: {}", cmd);
    }
}

fn read_map_json() -> Result<Value, serde_json::Error> {
    let manifest_dir = "/Users/italobarros/Tractian/dev/rust/automation";
    let map_path = std::path::Path::new(manifest_dir)
        .join("src")
        .join("safari")
        .join("map.json");
    let map_data = fs::read_to_string(&map_path).unwrap_or_else(|_| {
        panic!("Unable to read map.json at path: {}", map_path.display());
    });
    serde_json::from_str(&map_data)
}

pub fn open_safari_with_tags(tags: &[String]) {
    let map: Value = read_map_json().expect("Unable to parse map.json");
    if let Some(map_object) = map.as_object() {
        for (_, value) in map_object {
            if let Some(url_tags) = value["tags"].as_array() {
                let url_tags: Vec<String> = url_tags.iter().filter_map(|tag| tag.as_str().map(|s| s.to_string())).collect();

                if tags.iter().all(|tag| url_tags.contains(tag)) {
                    if let Some(url) = value["url"].as_str() {
                        let status = Command::new("open")
                            .arg("-a")
                            .arg("Safari")
                            .arg(url)
                            .status();

                        match status {
                            Ok(status) if status.success() => {
                                println!("Successfully opened Safari with URL: {}", url);
                            }
                            _ => {
                                eprintln!("Failed to open Safari with URL: {}", url);
                            }
                        }
                    }
                }
            }
        }
    }
}