use std::io::{BufRead, self};
use std::fs::File;
use std::path::Path;

fn main() {
    let lines = read_all_lines_from_file("Max.asm");
    for line in lines.iter() {
        println!("{}", line);
    }

    println!("\n\nTrimmed:");

    let lines = read_trimmed_lines_from_file("Max.asm");
    for line in lines.iter() {
        println!("{}", line);
    }

    println!("\n\nRemove All Comments:");
    let lines = remove_all_comments(&lines);
    for line in lines.iter() {
        println!("{}", line);
    }


    println!("\n\nRemove All Empty Lines:");
    let lines = remove_all_empty_lines(&lines);
    for line in lines.iter() {
        println!("{}", line);
    }

}

fn remove_all_empty_lines(lines: &Vec<String>) -> Vec<String> {
    lines.iter().filter(|line| 
                        !line.is_empty())
                        .cloned()
                        .collect()
}

fn read_trimmed_lines_from_file(file_name: &str) -> Vec<String> {
    let file_lines = read_all_lines_from_file(file_name);
    let file_lines_trimmed: Vec<String> = file_lines.iter().map(|s| 
                                                                remove_white_spaces(s)).collect();
    return file_lines_trimmed;
}

fn remove_all_comments(lines: &Vec<String>) -> Vec<String> {
    lines.iter().map(|s| remove_comment(s))
            .collect()
}

fn read_all_lines_from_file(file_name: &str) -> Vec<String> {
    let mut ret = Vec::new();
    let file_path = Path::new(file_name);
    let file_handle = File::open(&file_path)
                        .expect(&format!("Failed to open source file {}", file_name));
    let reader = io::BufReader::new(file_handle);
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        ret.push(line);
    }
    ret
}

fn remove_white_spaces(some_string: &str) -> String {
    let string_without_whitespaces: String = some_string.chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    return string_without_whitespaces;
}

fn remove_comment(some_string: &str) -> String {
    match some_string.find("//") {
        None => some_string.to_string(),
        Some(index) => {
            let string_before_comment = some_string.get(..index)
                                            .expect("trouble getting everything before comment").to_string();
            string_before_comment
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() {
        let lines = read_all_lines_from_file("Max.asm");
        assert_eq!(lines.len(), 30);
    }

    #[test]
    fn test_remove_white_spaces() {
        let white_spaces_removed = remove_white_spaces("  A=B;JMP\t");
        assert_eq!(white_spaces_removed, "A=B;JMP");
    }

    #[test]
    fn test_read_trimmed_lines() {
        let lines = read_trimmed_lines_from_file("Max.asm");
        assert_eq!(lines.len(), 30);
        assert_eq!(lines[9], "D=M");
    }

    #[test]
    fn test_remove_all_comments() {
        let lines = vec!["// abc".to_string(), "a=b//dasdas//".to_string()];
        let expected = vec!["".to_string(), "a=b".to_string()];
        let lines_with_no_comments = remove_all_comments(&lines);
        assert_eq!(lines_with_no_comments, expected);
    }
    
    #[test]
    fn test_remove_empty_lines() {
        let lines = vec!["abc".to_string(), "".to_string(), "abc".to_string()];
        let expected = vec!["abc".to_string(), "abc".to_string()];
        let result = remove_all_empty_lines(&lines);
        assert_eq!(result, expected);
    }
}
