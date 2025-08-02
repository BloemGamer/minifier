use regex;

pub fn minify(js_parts: Vec<&str>) -> Result<Vec<String>, ()>
{
    if js_parts.is_empty() { return Ok(Vec::new()) }

    let mut new_js_parts: Vec<String> = Vec::<String>::new();
    for part_js in js_parts
    {
        let mut part: String = part_js.to_string();
        part = part.replace("\r", "");

        let regex_comments_multiple_lines: regex::Regex = regex::Regex::new(r"(?s)/\*.*?\*/").unwrap();
        part = regex_comments_multiple_lines.replace_all(&part, "").to_string();
        let regex_comments_single_line: regex::Regex = regex::Regex::new(r"//.*$").unwrap();
        part = regex_comments_single_line.replace_all(&part, "").to_string();

        let regex_whitespace_start_line: regex::Regex = regex::Regex::new(r"^ *").unwrap();
        part = regex_whitespace_start_line.replace_all(&part, "").to_string();
        let regex_whitespace_end_line: regex::Regex = regex::Regex::new(r" *$").unwrap();
        part = regex_whitespace_end_line.replace_all(&part, "").to_string();


        new_js_parts.push(part.to_string());
    }
    Ok(new_js_parts)
}
