mod tests_minifier
{
    #[test]
    fn test_css()
    {
        use minifier::css;
        let css = r#"
            p::before {
                content: "Hello, world!  ";
                content: ' Another string ';
                padding: 10px 5px 10px 5px;
            }
        "#;

        let minified = css::minify(vec![css]).unwrap()[0].clone();
        
        assert!(minified.contains(r#""Hello, world!  ""#));
        assert!(minified.contains(r#"' Another string '"#));
        assert!(minified.contains("padding:10px 5px 10px 5px;"));
        let expected = r#"p::before{content:"Hello, world!  ";content:' Another string ';padding:10px 5px 10px 5px;}"#;
        assert_eq!(minified, expected);
    }

    #[test]
    fn test_html()
    {
        use minifier::html;
        let html = r#"
<!-- This is the basic structure of an HTML document -->
<!DOCTYPE html>
<html lang="en">
<head>
  <!-- The <meta> tag defines metadata like character encoding -->
  <meta charset="UTF-8">
  <!-- The <title> tag sets the title of the page shown in the browser tab -->
  <title>My Simple Page</title>
</head>
<body>
  <!-- This is a main heading -->
  <h1>Welcome to My Website</h1>

  <!-- This is a paragraph of text -->
  <p>This is a simple HTML example with comments.</p>

  <!-- This is a link to another website -->
  <a href="https://example.com">Visit Example.com</a>
</body>
</html>
        "#;

        let minified = html::minify(vec![html]).unwrap()[0].clone();
        
        let expected = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><title>My Simple Page</title></head><body><h1>Welcome to My Website</h1><p>This is a simple HTML example with comments.</p><a href=\"https://example.com\">Visit Example.com</a></body></html>";
        assert_eq!(minified, expected);
    }
}
