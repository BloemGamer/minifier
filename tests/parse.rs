#[cfg(test)]
mod test_parser {
    use minifier::parse::{split_file_into_parts};

    #[test]
    fn test_basic_separation() {
        let html = r#"<div>HTML content</div><script>console.log('js');</script><style>body { color: red; }</style>"#;
        
        let result = split_file_into_parts(html).unwrap();
        
        assert_eq!(result.html.len(), 3);
        assert_eq!(result.js.len(), 1);
        assert_eq!(result.css.len(), 1);
        
        assert_eq!(result.html[0], "<div>HTML content</div><script>");
        assert_eq!(result.html[1], "</script><style>");
        assert_eq!(result.js[0], "console.log('js');");
        assert_eq!(result.css[0], "body { color: red; }");
    }

    #[test]
    fn test_multiple_blocks() {
        let html = r#"
            <h1>Title</h1>
            <script>
                console.log('first script');
            </script>
            <p>Paragraph</p>
            <style>
                .container { margin: 0; }
            </style>
            <div>More HTML</div>
            <script>
                alert('second script');
            </script>
        "#;
        
        let result = split_file_into_parts(html).unwrap();
        
        assert_eq!(result.html.len(), 4);
        assert_eq!(result.js.len(), 2);
        assert_eq!(result.css.len(), 1);
        
        assert!(result.html[0].contains("<h1>Title</h1>"));
        assert!(result.html[1].contains("<p>Paragraph</p>"));
        assert!(result.html[2].contains("<div>More HTML</div>"));
        
        assert_eq!(result.js[0], "console.log('first script');");
        assert_eq!(result.js[1], "alert('second script');");
        
        assert_eq!(result.css[0], ".container { margin: 0; }");
    }

    #[test]
    fn test_empty_tags() {
        let html = r#"<div>content</div><script></script><style>   </style><p>more</p>"#;
        
        let result = split_file_into_parts(html).unwrap();
        
        // Empty script and style tags should be ignored
        assert_eq!(result.html.len(), 3);
        assert_eq!(result.js.len(), 0);
        assert_eq!(result.css.len(), 0);
        
        assert!(result.html[0].contains("<div>content</div>"));
        assert!(result.html[2].contains("<p>more</p>"));
    }

    #[test]
    fn test_html_only() {
        let html = r#"
            <!DOCTYPE html>
            <html>
            <head><title>Test</title></head>
            <body>
                <h1>Hello World</h1>
                <p>No scripts or styles here!</p>
            </body>
            </html>
        "#;
        
        let result = split_file_into_parts(html).unwrap();
        
        assert_eq!(result.js.len(), 0);
        assert_eq!(result.css.len(), 0);
        assert_eq!(result.html.len(), 1);
        
        // Should contain all the HTML
        assert!(result.html[0].contains("<!DOCTYPE html>"));
        assert!(result.html[0].contains("<h1>Hello World</h1>"));
    }

    #[test]
    fn test_js_and_css_only() {
        let html = r#"<script>console.log('hello');</script><style>body { margin: 0; }</style>"#;
        
        let result = split_file_into_parts(html).unwrap();
        
        assert_eq!(result.html.len(), 3);
        assert_eq!(result.js.len(), 1);
        assert_eq!(result.css.len(), 1);
        
        assert_eq!(result.js[0], "console.log('hello');");
        assert_eq!(result.css[0], "body { margin: 0; }");
    }

    #[test]
    fn test_script_with_attributes() {
        let html = r#"
            <div>Before</div>
            <script type="text/javascript">
                function test() { return 42; }
            </script>
            <div>After</div>
        "#;
        
        let result = split_file_into_parts(html).unwrap();
        
        assert_eq!(result.html.len(), 2);
        assert_eq!(result.js.len(), 1);
        
        assert!(result.html[0].contains("<div>Before</div>\n"));
        assert!(result.html[1].contains("<div>After</div>"));
        assert_eq!(result.js[0], "function test() { return 42; }");
    }

    #[test]
    fn test_style_with_attributes() {
        let html = r#"
            <h1>Title</h1>
            <style type="text/css">
                .header { font-size: 24px; }
            </style>
            <p>Content</p>
        "#;
        
        let result = split_file_into_parts(html).unwrap();
        
        assert_eq!(result.html.len(), 2);
        assert_eq!(result.css.len(), 1);
        
        assert!(result.html[0].contains("<h1>Title</h1>"));
        assert!(result.html[1].contains("<p>Content</p>"));
        assert_eq!(result.css[0], ".header { font-size: 24px; }");
    }

    #[test]
    fn test_complex_realistic_html() {
        let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Test Page</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            margin: 0;
            padding: 20px;
        }
        .container {
            max-width: 800px;
            margin: 0 auto;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>Welcome to My Site</h1>
        <p>This is a test paragraph with some content.</p>
        
        <script>
            console.log('Page loaded');
            
            function greetUser(name) {
                alert('Hello, ' + name + '!');
            }
        </script>
        
        <div class="content">
            <h2>Section Title</h2>
            <p>More content here.</p>
        </div>
    </div>
    
    <style>
        .footer {
            background: #f0f0f0;
            padding: 10px;
            text-align: center;
        }
    </style>
    
    <footer class="footer">
        <p>&copy; 2024 Test Site</p>
    </footer>
    
    <script>
        document.addEventListener('DOMContentLoaded', function() {
            console.log('DOM ready');
        });
    </script>
</body>
</html>"#;
        
        let result = split_file_into_parts(html).unwrap();
        
        // Should have multiple HTML blocks
        assert!(result.html.len() >= 3);
        // Should have 2 JS blocks
        assert_eq!(result.js.len(), 2);
        // Should have 2 CSS blocks
        assert_eq!(result.css.len(), 2);
        
        // Check that HTML blocks contain expected content
        let all_html = result.html.join("");
        assert!(all_html.contains("<!DOCTYPE html>"));
        assert!(all_html.contains("<h1>Welcome to My Site</h1>"));
        assert!(all_html.contains("<footer class=\"footer\">"));
        assert_eq!(result.html[3], r#"</style>
    
    <footer class="footer">
        <p>&copy; 2024 Test Site</p>
    </footer>
    
    <script>"#);
        
        // Check JS content
        assert!(result.js[0].contains("console.log('Page loaded')"));
        assert!(result.js[0].contains("function greetUser"));
        assert!(result.js[1].contains("DOMContentLoaded"));
        
        // Check CSS content
        assert!(result.css[0].contains("font-family: Arial"));
        assert!(result.css[0].contains("max-width: 800px"));
        assert!(result.css[1].contains("background: #f0f0f0"));
    }

    #[test]
    fn test_malformed_html() {
        // Test with unclosed script tag
        let html = r#"<div>content</div><script>console.log('test');"#;
        
        let result = split_file_into_parts(html);
        
        // Should still extract the HTML part
        assert!(result.is_ok());
    }

    #[test]
    fn test_consecutive_tags() {
        let html = r#"<script>js1</script><script>js2</script><style>css1</style><style>css2</style>"#;
        
        let result = split_file_into_parts(html).unwrap();
        
        assert_eq!(result.html.len(), 5);
        assert_eq!(result.js.len(), 2);
        assert_eq!(result.css.len(), 2);
        
        assert_eq!(result.js[0], "js1");
        assert_eq!(result.js[1], "js2");
        assert_eq!(result.css[0], "css1");
        assert_eq!(result.css[1], "css2");
    }

    #[test]
    fn test_whitespace_handling() {
        let html = r#"
        
        <div>HTML</div>
        
        <script>
            console.log('test');
        </script>
        
        <p>More HTML</p>
        
        "#;
        
        let result = split_file_into_parts(html).unwrap();
        
        assert_eq!(result.html.len(), 2);
        assert_eq!(result.js.len(), 1);
        
        // Should trim whitespace from HTML blocks
        assert!(result.html[0].contains("<div>HTML</div>"));
        assert!(result.html[1].contains("<p>More HTML</p>"));
        
        // Should trim whitespace from JS
        assert_eq!(result.js[0], "console.log('test');");
    }

    #[test]
    fn test_edge_cases() {
        // Empty string
        let result = split_file_into_parts("").unwrap();
        assert_eq!(result.html.len(), 0);
        assert_eq!(result.js.len(), 0);
        assert_eq!(result.css.len(), 0);
        
        // Only whitespace
        let result = split_file_into_parts("   \n\t  ").unwrap();
        assert_eq!(result.html.len(), 0);
        assert_eq!(result.js.len(), 0);
        assert_eq!(result.css.len(), 0);
        
        // Single character
        let result = split_file_into_parts("a").unwrap();
        assert_eq!(result.html.len(), 1);
        assert_eq!(result.html[0], "a");
    }
}
