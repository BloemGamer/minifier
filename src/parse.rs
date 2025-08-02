#[derive(Debug)]
pub struct FileStructure<'a>
{
    pub html: Vec<&'a str>,
    pub js: Vec<&'a str>,
    pub css: Vec<&'a str>,
    pub block_order: Vec<BlockType>,
}

#[derive(Debug, Clone, Copy)]
pub enum BlockType
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

pub fn split_file_into_parts<'a>(file: &'a str) -> Result<FileStructure<'a>, ()>
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


fn extract_single_tag_content<'a>(input: &'a str, start: usize, tag: &str) -> Result<Option<(&'a str, usize)>, ()>
{
    let open_tag_end: usize = match input[start..].find('>') 
    {
        Some(i) => start + i + 1,
        None => return Ok(None),
    };
    let close_tag: String = format!("</{}>", tag);

    let close_tag_start_opt: Option<usize> = input[open_tag_end..].find(&close_tag).map(|i| open_tag_end + i);

    let (content, end_pos): (&str, usize) = match close_tag_start_opt
    {
        Some(close_tag_start) =>
        {
            let content: &str = &input[open_tag_end..close_tag_start];
            let end_pos: usize = close_tag_start + close_tag.len();
            (content, end_pos)
        }
        None =>
        {
            // No closing tag: consume to end of file
            let content: &str = &input[open_tag_end..];
            let end_pos: usize = input.len();
            (content, end_pos)
        }
    };

    Ok(Some((content, end_pos)))
}
