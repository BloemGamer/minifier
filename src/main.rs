use std::{self, io::Write};

mod minify;

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
    
    let new_file: String = minify::minify_file(file);
    let new_test_file: String = minify::minify_file(file2);

    write_to_file(new_file, "test-files/generated.html");
    write_to_file(new_test_file, "test-files (copy)/generated.html");
    println!("Size of file: {} B", std::fs::metadata("test-files/generated.html").unwrap().len());
    println!("Size of file: {} B", std::fs::metadata("test-files (copy)/generated.html").unwrap().len());
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

