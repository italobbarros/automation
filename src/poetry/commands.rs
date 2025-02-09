

pub fn is_valid_python_version(requirement: &str, version: &str) -> bool {
    if version.chars().any(|c| !c.is_digit(10) && c != '.') {
        return false;
    }
    let version_parts: Vec<u32> = version.split('.')
        .filter_map(|s| s.parse().ok())
        .collect();

    let parts: Vec<&str> = requirement.split(',').collect();
    let lower_bound = parts[0].trim();
    let upper_bound = parts[1].trim();

    if lower_bound.starts_with('^') {
        let lower_version = &lower_bound[1..].trim();
        let lower_parts: Vec<u32> = lower_version.split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        if version_parts <= lower_parts {
            return false;
        }
    }

    if upper_bound.starts_with('<') {
        let upper_version = &upper_bound[1..].trim();
        let upper_parts: Vec<u32> = upper_version.split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        if version_parts >= upper_parts {
            return false;
        }
    }

    true
}

pub fn get_python_version(contents: &str) -> String {
    let version = contents.lines()
    .find(|line| line.contains("python = "))
    .and_then(|line| line.split('=').nth(1))
    .map(|v| v.trim().trim_matches('"').to_string())
    .unwrap_or_default();
    version
}