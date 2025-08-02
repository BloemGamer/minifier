use std::{self, io::Write};
use minify_html::{minify, Cfg};

mod parse;
mod html;
mod js;
mod css;

pub struct FileParts
{
    pub html: Vec<String>,
    pub js: Vec<String>,
    pub css: Vec<String>,
    pub block_order: Vec<parse::BlockType>,
}
impl FileParts
{
    pub fn new() -> Self
    {
        Self { html: Vec::new(), js: Vec::new(), css: Vec::new(), block_order: Vec::new() }
    }
}

fn main()
{
    let file: String = match get_file()
    {
        Ok(file) => file,
        Err(_) => { eprintln!("There was not given exactly one argument"); panic!() },
    };
    
    //let new_file: String = minify_file(file.clone());
    let new_test_file: String = minify_test_file(file);

    //write_to_file(new_file, "test-files/generated.html");
    write_to_file(new_test_file, "test-files/test_generated.html");


    println!("Done with parsing the file");
}

fn get_file() -> Result<String, ()>
{
    std::fs::read_to_string("test-files/test.html").map_err(|_| ())
}

fn write_to_file(file_content: String, file_name: &str)
{
    let mut file = std::fs::File::create(file_name).unwrap();
    let _ = file.write_all(file_content.as_bytes());
}

fn minify_file(file: String) -> String
{
    let parsed_file: parse::FileStructure<'_> = parse::split_file_into_parts(&file).unwrap();

    let mut file_parts: FileParts = FileParts::new();

    file_parts.block_order = parsed_file.block_order;
    file_parts.html = html::minify(parsed_file.html.clone()).unwrap();
    file_parts.css = css::minify(parsed_file.css).unwrap();
    file_parts.js = js::minify(parsed_file.js).unwrap();

    generate_file_from_structure(&file_parts)
}


pub fn generate_file_from_structure(file_parts: &FileParts) -> String
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
            parse::BlockType::Script =>
            {
                if js_index < file_parts.js.len()
                {
                    //result.push_str("<script>");
                    result.push_str(&file_parts.js[js_index]);
                    //result.push_str("</script>");
                    js_index += 1;
                }
            }
            parse::BlockType::Style =>
            {
                if css_index < file_parts.css.len()
                {
                    //result.push_str("<style>");
                    result.push_str(&file_parts.css[css_index]);
                    //result.push_str("</style>");
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



fn minify_test_file(file_given: String) -> String
{
    let file: String = file_given.replace("\r", "");

    let cfg = Cfg {
        do_not_minify_doctype: false,
        keep_closing_tags: false,
        keep_comments: false,
        minify_css: true,
        minify_js: true,
        ensure_spec_compliant_unquoted_attribute_values: true,
        keep_html_and_head_opening_tags: true,
        keep_spaces_between_attributes: false,
        keep_input_type_text_attr: false,
        keep_ssi_comments: false,
        preserve_brace_template_syntax: false,
        preserve_chevron_percent_template_syntax: false,
        remove_bangs: true,
        remove_processing_instructions: true,
        

        //..Default::default()
    };

    let minified = minify(file.as_bytes(), &cfg);
    String::from_utf8(minified).unwrap()
}
