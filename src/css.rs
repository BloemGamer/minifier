use regex;

pub fn minify(css_parts: Vec<&str>) -> Result<Vec<String>, ()>
{
    if css_parts.is_empty() { return Ok(Vec::new()) }

    let mut new_css_parts: Vec<String> = Vec::<String>::new();
    for part_css in css_parts
    {
        let mut part: String = part_css.to_string();
        part = part.replace("\r", "");

        let regex_comments: regex::Regex = regex::Regex::new(r"(?s)/\*.*?\*/").unwrap();
        part = regex_comments.replace_all(&part, "").to_string();

        part = remove_whitespace_outside_strings(&part);


        new_css_parts.push(part.to_string());
    }
    Ok(new_css_parts)
}

fn remove_whitespace_outside_strings(css: &str) -> String
{
    let string_re = regex::Regex::new(r#""([^"\\]|\\.)*"|'([^'\\]|\\.)*'"#).unwrap();

    let mut result = String::with_capacity(css.len());
    let mut last_end = 0;

    for mat in string_re.find_iter(css) {
        // Process the non-string segment before this string literal
        let non_string = &css[last_end..mat.start()];

        // Remove line breaks and tabs
        let mut cleaned = non_string.replace(&['\n', '\r', '\t'][..], "");

        // Replace multiple spaces with a single space
        while cleaned.contains("  ") {
            cleaned = cleaned.replace("  ", " ");
        }

        // Remove spaces around CSS punctuation characters: { } : ; ,
        // But keep spaces inside values like margin: 0 10px 20px 0;
        for ch in &['{', '}', ':', ';', ','] {
            let spaced = format!(" {}", ch);
            cleaned = cleaned.replace(&spaced, &ch.to_string());
            let spaced = format!("{} ", ch);
            cleaned = cleaned.replace(&spaced, &ch.to_string());
        }

        // Trim leading/trailing spaces left behind
        cleaned = cleaned.trim().to_string();

        result.push_str(&cleaned);
        // Append string literal unchanged
        result.push_str(mat.as_str());

        last_end = mat.end();
    }

    // Process the tail after the last string
    let non_string = &css[last_end..];
    let mut cleaned = non_string.replace(&['\n', '\r', '\t'][..], "");
    while cleaned.contains("  ") {
        cleaned = cleaned.replace("  ", " ");
    }
    for ch in &['{', '}', ':', ';', ','] {
        let spaced = format!(" {}", ch);
        cleaned = cleaned.replace(&spaced, &ch.to_string());
        let spaced = format!("{} ", ch);
        cleaned = cleaned.replace(&spaced, &ch.to_string());
    }
    cleaned = cleaned.trim().to_string();

    result.push_str(&cleaned);

    result
}

