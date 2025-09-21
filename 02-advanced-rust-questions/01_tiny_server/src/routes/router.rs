use std::collections::HashMap;

pub fn match_route<'a>(pattern: &str, path: &'a str) -> Option<HashMap<String, String>> {
    let mut params = HashMap::new();
    let pattern_parts: Vec<&str> = pattern.split('/').collect();
    let path_parts: Vec<&str> = path.split('/').collect();

    if pattern_parts.len() != path_parts.len() {
        return None;
    }

    for (p_pat, p_path) in pattern_parts.iter().zip(path_parts.iter()) {
        if p_pat.starts_with(':') {
            let key = p_pat.trim_start_matches(':').to_string();
            params.insert(key, (*p_path).to_string());
        } else if p_pat != p_path {
            return None;
        }
    }

    Some(params)
}
