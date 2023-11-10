use std::io::{BufRead, self};
use std::fs::File;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Instruction {
    Ainstruction {address: String},
    Cinstruction {destination: String, computation: String, jump_type: String}
}

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


    println!("\n\nAll Instructions");
    let lines = remove_all_empty_lines(&lines);
    for line in lines.iter() {
        println!("{}", line);
    }

}

fn get_instruction(assembly_string: &str) -> Instruction {
    assert!(assembly_string.len() > 0, "Empty strings cannot be converted into instructions");
    let first_character = assembly_string.get(0..1).unwrap();
    if first_character == "@" { // A instruction 
        let address = assembly_string.get(1..)
                        .expect("trouble getting the address part of A instruction");
        let address = address.to_string();
        return Instruction::Ainstruction { address };
    } else { // C instruction
        let destination = match assembly_string.find("=") {
            None => "".to_string(),
            Some(index) => {
                assembly_string.get(..index).expect("trouble getting everything before =").to_string()
            }
        };

        let mut computation_begin_idx = destination.len();
        if !destination.is_empty() {
            computation_begin_idx += 1;
        }

        let computation = match assembly_string.find(";") {
            None => assembly_string.get(computation_begin_idx..)
                        .expect("trouble getting computation without jump")
                        .to_string(),
            Some(index) => assembly_string.get(computation_begin_idx..index)
                            .expect("trouble getting computation without jump")
                            .to_string() 
        };
        assert!(!computation.is_empty(), "computation can never be empty: assembly = {}", assembly_string);

        let jump_type = match assembly_string.find(";") {
            None => "".to_string(),
            Some(index) => assembly_string.get((index+1)..)
                            .expect(&format!("trouble getting jump type, assembly {}", assembly_string))
                            .to_string()  
        };

        return Instruction::Cinstruction { destination, computation, jump_type}
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

    #[test]
    fn test_get_instruction() {
        let instruction = get_instruction("@1234");
        assert_eq!(instruction, Instruction::Ainstruction { address: "1234".into() });

        
        let instruction = get_instruction("D=A+D;JMP");
        assert_eq!(instruction, Instruction::Cinstruction { destination: "D".into(), computation: "A+D".into(), jump_type: "JMP".into()});


        let instruction = get_instruction("D=A+D");
        assert_eq!(instruction, Instruction::Cinstruction { destination: "D".into(), computation: "A+D".into(), jump_type: "".into()});


        let instruction = get_instruction("0;JMP");
        assert_eq!(instruction, Instruction::Cinstruction { destination: "".into(), computation: "0".into(), jump_type: "JMP".into()});
    }
}
