use std::{self, io::Write};
use minify_html;
use minify_js;

struct FileParts<'a>
{
    html: Vec<&'a str>,
    js: Vec<String>,
    css: Vec<&'a str>,
    block_order: Vec<BlockType>,
}
impl FileParts<'_>
{
    pub fn new() -> Self
    {
        Self { html: Vec::new(), js: Vec::new(), css: Vec::new(), block_order: Vec::new() }
    }
}

#[derive(Debug)]
struct FileStructure<'a>
{
    pub html: Vec<&'a str>,
    pub js: Vec<&'a str>,
    pub css: Vec<&'a str>,
    pub block_order: Vec<BlockType>,
}

#[derive(Debug, Clone, Copy)]
enum BlockType
{
    Script,
    Style,
}

impl FileStructure<'_>
{
    pub fn new() -> Self
    {
        Self 
        {
            html: Vec::new(), 
            js: Vec::new(), 
            css: Vec::new(),
            block_order: Vec::new(),
        }
    }
}

fn main()
{
    let file: String = match get_file("test-files/test.html")
    {
        Ok(file) => file,
        Err(_) => { eprintln!("There was not given exactly one argument"); panic!() },
    };
    let file2: String = match get_file("test-files (copy)/test.html")
    {
        Ok(file) => file,
        Err(_) => { eprintln!("There was not given exactly one argument"); panic!() },
    };
    
    let new_file: String = minify_file(file);
    let new_test_file: String = minify_file(file2);

    write_to_file(new_file, "test-files/generated.html");
    write_to_file(new_test_file, "test-files (copy)/generated.html");
    println!("Size of file: {} B", std::fs::metadata("test-files/generated.html").unwrap().len());
    println!("Size of file: {} B", std::fs::metadata("test-files (copy)/generated.html").unwrap().len());
}

pub fn minify_file(file_given: String) -> String
{
    let file: String = file_given.replace("\r", "");

    let parsed_file: FileStructure<'_> = split_file_into_parts(&file).unwrap();

    let cfg = minify_html::Cfg {
        minify_doctype: false,
        keep_closing_tags: false,
        keep_comments: false,
        minify_css: true,
        minify_js: false,
        allow_noncompliant_unquoted_attribute_values: false,
        keep_html_and_head_opening_tags: false,
        allow_removing_spaces_between_attributes: true,
        keep_input_type_text_attr: false,
        keep_ssi_comments: false,
        preserve_brace_template_syntax: true,
        preserve_chevron_percent_template_syntax: false,
        remove_bangs: true,
        remove_processing_instructions: true,
        allow_optimal_entities: false,


        //..Default::default()
    };
    let mut file_parts: FileParts = FileParts::new();

    for html in parsed_file.html
    {
        file_parts.html.push(html);
    }
    for js in parsed_file.js
    {
        let session = minify_js::Session::new();
        let source = js.as_bytes();
        let mut output: Vec<u8> = Vec::new();
        minify_js::minify(&session, minify_js::TopLevelMode::Global, &source, &mut output).unwrap();
        file_parts.js.push(String::from_utf8(output).unwrap());
    }
    for css in parsed_file.css
    {
        file_parts.css.push(css);
    }

    file_parts.block_order = parsed_file.block_order;
    let new_file = generate_file_from_structure(&file_parts);

    let minified = minify_html::minify(new_file.as_bytes(), &cfg);
    String::from_utf8(minified).unwrap()
}

fn get_file(file_name: &str) -> Result<String, ()>
{
    std::fs::read_to_string(file_name).map_err(|_| ())
}

fn write_to_file(file_content: String, file_name: &str)
{
    let mut file = std::fs::File::create(file_name).unwrap();
    let _ = file.write_all(file_content.as_bytes());
}

fn generate_file_from_structure(file_parts: &FileParts) -> String
{
    let mut result = String::new();
    
    let mut js_index = 0;
    let mut css_index = 0;
    let mut html_index = 0;
    
    // Add first HTML chunk if available
    if html_index < file_parts.html.len()
    {
        result.push_str(&file_parts.html[html_index]);
        html_index += 1;
    }
    
    // Process blocks in their original order
    for block_type in &file_parts.block_order
    {
        match block_type
        {
            BlockType::Script =>
            {
                if js_index < file_parts.js.len()
                {
                    result.push_str(&file_parts.js[js_index]);
                    js_index += 1;
                }
            }
            BlockType::Style =>
            {
                if css_index < file_parts.css.len()
                {
                    result.push_str(&file_parts.css[css_index]);
                    css_index += 1;
                }
            }
        }
        
        // Add next HTML chunk if available
        if html_index < file_parts.html.len()
        {
            result.push_str(&file_parts.html[html_index]);
            html_index += 1;
        }
    }
    
    result
}



fn split_file_into_parts<'a>(file: &'a str) -> Result<FileStructure<'a>, ()>
{
    let mut file_structure: FileStructure<'a> = FileStructure::new();
    let mut pos = 0;
    let mut last_html_end = 0;

    while pos < file.len()
    {
        let remaining = &file[pos..];

        let next_script = remaining.find("<script");
        let next_style = remaining.find("<style");

        let (next_tag_start, tag_name): (usize, &str) = match (next_script, next_style)
        {
            (Some(s), Some(t)) =>
            {
                if s < t
                {
                    (pos + s, "script")
                }
                else
                {
                    (pos + t, "style")
                }
            }
            (Some(s), None) =>
            {
                (pos + s, "script")
            }
            (None, Some(t)) =>
            {
                (pos + t, "style")
            }
            (None, None) =>
            {
                break;
            }
        };

        let open_tag_end = match file[next_tag_start..].find('>')
        {
            Some(idx) =>
            {
                next_tag_start + idx + 1
            }
            None =>
            {
                pos = next_tag_start + 1;
                last_html_end = pos;
                continue;
            }
        };

        let closing_tag_str = format!("</{}>", tag_name);
        let closing_tag_start = match file[open_tag_end..].find(&closing_tag_str)
        {
            Some(idx) =>
            {
                open_tag_end + idx
            }
            None =>
            {
                pos = open_tag_end;
                last_html_end = pos;
                continue;
            }
        };

        let closing_tag_end = closing_tag_start + closing_tag_str.len();

        if last_html_end < open_tag_end
        {
            let html_chunk = &file[last_html_end..open_tag_end];
            let trimmed = html_chunk.trim();
            if !trimmed.is_empty()
            {
                file_structure.html.push(trimmed);
            }
        }

        let content = &file[open_tag_end..closing_tag_start];
        let trimmed_content = content.trim();

        if !trimmed_content.is_empty()
        {
            match tag_name
            {
                "script" =>
                {
                    file_structure.js.push(trimmed_content);
                    file_structure.block_order.push(BlockType::Script);
                }
                "style" =>
                {
                    file_structure.css.push(trimmed_content);
                    file_structure.block_order.push(BlockType::Style);
                }
                _ => {}
            }
        }

        last_html_end = closing_tag_start;
        pos = closing_tag_end;
    }

    if last_html_end < file.len()
    {
        let remaining_html = &file[last_html_end..];
        let trimmed = remaining_html.trim();
        if !trimmed.is_empty()
        {
            file_structure.html.push(trimmed);
        }
    }

    Ok(file_structure)
}
