use regex;

pub fn minify(html_parts: Vec<&str>) -> Result<Vec<String>, ()>
{
    if html_parts.is_empty() { return Ok(Vec::new()) }

    let mut new_html_parts: Vec<String> = Vec::<String>::new();
    for part_html in html_parts
    {
        let mut part: String = part_html.to_string();
        part = part.replace("\r", "");

        let regex_comments: regex::Regex = regex::Regex::new("(?s)<!--.*?-->").unwrap();
        part = regex_comments.replace_all(&part, "").to_string();

        let regex_whitespace: regex::Regex = regex::Regex::new(r">[\s]+<").unwrap();
        part = regex_whitespace.replace_all(&part, "><").to_string();
        let regex_whitespace_start_file: regex::Regex = regex::Regex::new(r"^[\s]+<").unwrap();
        part = regex_whitespace_start_file.replace_all(&part, "<").to_string();
        let regex_whitespace_end_file: regex::Regex = regex::Regex::new(r">[\s]+$").unwrap();
        part = regex_whitespace_end_file.replace_all(&part, ">").to_string();


        new_html_parts.push(part.to_string());
    }
    Ok(new_html_parts)
}
