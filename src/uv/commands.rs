

pub fn is_valid_python_version(requirement: &str, version: &str) -> bool {
    if version.chars().any(|c| !c.is_digit(10) && c != '.') {
        return false;
    }
    let version_parts: Vec<u32> = version.split('.')
        .filter_map(|s| s.parse().ok())
        .collect();
    
    let parts: Vec<&str> = requirement.split(',').collect();
    let lower_bound = parts[0].trim();
    let upper_bound = if parts.len() > 1 { parts[1].trim() } else { "" };

    let mut lower_bound_check = 0;
    let mut upper_bound_check = 0;

    if lower_bound.starts_with('>') {
        lower_bound_check = -1;
        let required_version = &lower_bound[1..].trim();
        let required_parts: Vec<u32> = required_version.split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        if version_parts > required_parts{
            lower_bound_check = 1;
        }
    }
    if lower_bound.starts_with(">=") {
        lower_bound_check = -1;
        let required_version = &lower_bound[2..].trim();
        let required_parts: Vec<u32> = required_version.split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        if version_parts >= required_parts{
            lower_bound_check = 1;
        }
    }
    if lower_bound.starts_with('=') {
        lower_bound_check = -1;
        let required_version = &lower_bound[1..].trim();
        let required_parts: Vec<u32> = required_version.split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        if version_parts == required_parts{
            lower_bound_check = 1;
        }
    }
    
    if upper_bound.starts_with('<') {
        upper_bound_check = -1;
        let required_version = &upper_bound[1..].trim();
        let required_parts: Vec<u32> = required_version.split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        if version_parts < required_parts{
            upper_bound_check = 1;
        }
    }
    if upper_bound.starts_with("<=") {
        upper_bound_check = -1;
        let required_version = &upper_bound[2..].trim();
        let required_parts: Vec<u32> = required_version.split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        if version_parts <= required_parts{
            upper_bound_check = 1;
        }
    }
    if upper_bound.starts_with('=') {
        upper_bound_check = -1;
        let required_version = &upper_bound[1..].trim();
        let required_parts: Vec<u32> = required_version.split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        if version_parts == required_parts{
            upper_bound_check = 1;
        }
    }

    if lower_bound_check == 1 && upper_bound_check == 1{
        return true;
    }
    if lower_bound_check == -1 && upper_bound_check == -1{
        return false;
    }
    if lower_bound_check == 1 && upper_bound_check == 0{
        return true;
    }
    if lower_bound_check == 0 && upper_bound_check == 1{
        return true;
    }
    return false;
}

pub fn get_python_version(contents: &str) -> String {
    let version = contents.lines()
    .find(|line| line.contains("requires-python = "))
    .and_then(|line| line.split("= ").nth(1))
    .map(|v| v.trim().trim_matches('"').to_string())
    .unwrap_or_default();
    version
}