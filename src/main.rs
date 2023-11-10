use std::io::{BufRead, self};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap; 

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
    let instructions = get_instructions_from_lines(&lines);
    for instruction in instructions.iter() {
        println!("{:?}", instruction);
    }
}

// Converts line(text) of assembly to an Instruction, which will
// later be used to create binary code.
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

        return Instruction::Cinstruction { destination, computation, jump_type }
    }
}

fn address_to_binary(address_string: &str) -> String {
    let parsed_integer = address_string.parse::<u16>()
                                .expect(&format!("Could not parse address {} into a 16 bit integer", address_string));
    let binary_string = format!("{:16b}", parsed_integer);
    let binary_string: String = binary_string.chars().into_iter()
                    .map(|char| {
                        if char == ' ' {
                            '0'
                        } else {
                            char
                        }
                    }).collect();
    assert_eq!(binary_string.len(), 16);
    let most_significant_bit = binary_string.get(0..1).unwrap();
    assert_eq!(most_significant_bit, "0");
    binary_string
}

fn instruction_to_binary(instruction: Instruction) -> String {
    match instruction {
        Instruction::Ainstruction{address} => {
            address_to_binary(&address)
        },
        Instruction::Cinstruction { destination, computation, jump_type } =>  {

        }
    }
}

fn remove_all_empty_lines(lines: &Vec<String>) -> Vec<String> {
    lines.iter().filter(|line| 
                        !line.is_empty())
                        .cloned()
                        .collect()
}

fn get_instructions_from_lines(lines: &Vec<String>) -> Vec<Instruction> {
    lines.iter().map(|s| get_instruction(s)).collect()
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

fn get_destination_to_binary_map() -> HashMap<String, String> {
    let mut destination_to_binary = HashMap::new();
    destination_to_binary.insert(String::from(""), String::from("000"));
    destination_to_binary.insert(String::from("M"), String::from("001"));
    destination_to_binary.insert(String::from("D"), String::from("010"));
    destination_to_binary.insert(String::from("MD"), String::from("011"));
    destination_to_binary.insert(String::from("A"), String::from("100"));
    destination_to_binary.insert(String::from("AM"), String::from("101"));
    destination_to_binary.insert(String::from("AD"), String::from("110"));
    destination_to_binary.insert(String::from("AMD"), String::from("111"));
    return destination_to_binary;
}

fn get_jump_type_to_binary_map() -> HashMap<String, String> {
    let mut jump_type_to_binary = HashMap::new();
    jump_type_to_binary.insert(String::from(""), String::from("000"));
    jump_type_to_binary.insert(String::from("JGT"), String::from("001"));
    jump_type_to_binary.insert(String::from("JEQ"), String::from("010"));
    jump_type_to_binary.insert(String::from("JGE"), String::from("011"));
    jump_type_to_binary.insert(String::from("JLT"), String::from("100"));
    jump_type_to_binary.insert(String::from("JNE"), String::from("101"));
    jump_type_to_binary.insert(String::from("JLE"), String::from("110"));
    jump_type_to_binary.insert(String::from("JMP"), String::from("111"));
    return jump_type_to_binary;
}

fn get_compuation_to_binary_map() -> HashMap<String, String> {
    let mut computation_to_binary = HashMap::new();
    computation_to_binary.insert(String::from("0"), String::from("0101010"));
    computation_to_binary.insert(String::from("1"), String::from("0111111"));
    computation_to_binary.insert(String::from("-1"), String::from("0111010"));
    computation_to_binary.insert(String::from("D"), String::from("0001100"));
    computation_to_binary.insert(String::from("A"), String::from("0110000"));
    computation_to_binary.insert(String::from("!D"), String::from("0001101"));
    computation_to_binary.insert(String::from("!A"), String::from("0110001"));
    computation_to_binary.insert(String::from("-D"), String::from("0001111"));
    computation_to_binary.insert(String::from("-A"), String::from("0110011"));
    computation_to_binary.insert(String::from("D+1"), String::from("0011111"));
    computation_to_binary.insert(String::from("A+1"), String::from("0110111"));
    computation_to_binary.insert(String::from("D-1"), String::from("0001110"));
    computation_to_binary.insert(String::from("A-1"), String::from("0110010"));
    computation_to_binary.insert(String::from("D+A"), String::from("0000010"));
    computation_to_binary.insert(String::from("D-A"), String::from("0010011"));
    computation_to_binary.insert(String::from("A-D"), String::from("0000111"));
    computation_to_binary.insert(String::from("D&A"), String::from("0000000"));
    computation_to_binary.insert(String::from("D|A"), String::from("0010101"));


    computation_to_binary.insert(String::from("M"), String::from("1110000"));
    computation_to_binary.insert(String::from("!M"), String::from("1110001"));
    computation_to_binary.insert(String::from("-M"), String::from("1110011"));
    computation_to_binary.insert(String::from("M+1"), String::from("1110111"));
    computation_to_binary.insert(String::from("M-1"), String::from("1110010"));
    computation_to_binary.insert(String::from("D+M"), String::from("1000010"));
    computation_to_binary.insert(String::from("D-M"), String::from("1010011"));
    computation_to_binary.insert(String::from("M-D"), String::from("1000111"));
    computation_to_binary.insert(String::from("D&M"), String::from("1000000"));
    computation_to_binary.insert(String::from("D|M"), String::from("1010101"));

    return computation_to_binary;
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

    #[test]
    fn test_address_to_binary() {
        assert_eq!("0000000000001000", address_to_binary("8"));
    }

    #[test]
    fn test_instruction_to_binary() {
        let result = instruction_to_binary(Instruction::Ainstruction { address: "8".into() });
        assert_eq!("0000000000001000", result);
    }

}
