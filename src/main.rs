use std::fs;

fn main() {
    match fs::read_to_string("input.txt") {
        Ok(contents) => {
            if let Err(e) = fs::write(
                "output.txt",
                contents.split_whitespace().count().to_string(),
            ) {
                eprintln!("Failed to write file: {}", e.to_string());
            };
        }
        Err(e) => {
            eprintln!("Error encountered: {}", e.to_string());
        }
    };
}
