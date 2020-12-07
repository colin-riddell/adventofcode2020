use std::fs;

struct Line {
    pub range_min: u32,
    pub range_max: u32,
    pub letter: char,
    pub password: String
}

impl Line {

    fn new(line: &str) -> Line {
        let split_line : Vec<&str> =  line.split(": ").collect();
        let left_split : Vec<&str> = split_line[0].split(" ").collect();
        let letter = left_split[1];
        let password = split_line[1];
        let range : Vec<&str> = left_split[0].split("-").collect();
        return Self{
            range_min: range[0].parse().unwrap(),
            range_max: range[1].parse().unwrap(),
            letter: letter.chars().nth(0).unwrap(),
            password: password.to_string()
        }
     
    }

    pub fn is_valid_part_two(&self) -> bool {

        let password_len :usize = self.password.len();
        if self.range_min as usize > password_len || self.range_max as usize > password_len{
            return false
        }

        let mut in_left : bool = true;
        let char_at_min : char = self.password.chars().nth(self.range_min as usize -1).unwrap();
        println!("char at min (position {} ) is: {} comparing to {} on password: {} ", self.range_min, char_at_min, self.letter, self.password);
        in_left = char_at_min == self.letter;

        let mut in_right: bool = true;
        let char_at_max : char = self.password.chars().nth(self.range_max as usize -1).unwrap();
        println!("char at max (position {} ) is: {} comparing to {} on password: {}  ", self.range_max, char_at_max, self.letter, self.password);
        in_right = char_at_max == self.letter;
 
        return in_left ^ in_right;
    }

    // is valid for part 1
    pub fn is_valid(&self) -> bool {
        // count the number of `letter` in the password
       let chars : Vec<char> =  self.password.chars().filter(|c| self.letter == *c).collect();
       if chars.len() >= self.range_min as usize && chars.len() <= self.range_max  as usize{
           return true;
       } else {
           return false;
       }
        // if the number of letters is > min and  < max
        // it's valid - return true
        // else return false
    }
}

pub fn run() {

    let input = fs::read_to_string("day2.txt").expect("Something went wrong reading the file");

    let lines : Vec<&str> = input.split("\n").collect();
    // let mut strings : Vec<String> = Vec::new();
    let mut all_parsed_lines : Vec<Line> = Vec::new();
    let mut num_valid = 0;
    for line in lines {
        println!("{}", line);
        let parsed_line = Line::new(line);
        let &valid = &parsed_line.is_valid_part_two();
        println!("  is valid: {}", valid);
        if valid == true {
            num_valid = num_valid + 1;
        }
        all_parsed_lines.push(parsed_line);
        
    }

    println!("There are {} valid passwords", num_valid);
}



#[test]
fn can_split_line() {
    // Given a line: 10-16 n: lvknnwnnvsmnnnnhn
    let line: String = String::from("10-16 n: lvknnwnnvsmnnnnhn");
    // When we split it
    let line_data: Line = Line::new(&line);

    // Then we should have
    // range min: 10
    assert_eq!(line_data.range_min, 10);
    // range max: 16
    assert_eq!(line_data.range_max, 16);
    // letter: n
    assert_eq!(line_data.letter, 'n');
    // password: lvknnwnnvsmnnnnhn
    assert_eq!(line_data.password, "lvknnwnnvsmnnnnhn");
}

#[test]
fn is_valid_invalid1() {

    // Given a line: 10-16 n: lvknnwnnvsmnnnnhn
    let line: String = String::from("10-16 n: lvknnwnnvsmnnnnhn");
    // When we split it
    let line_data: Line = Line::new(&line);

    assert_eq!(line_data.is_valid_part_two(), false);
    
}

#[test]
fn is_valid_invalid2() {

    // Given a line: 5-6 r: rrrrcqr
    let line: String = String::from("5-6 r: rrrrcqr");
    // When we split it
    let line_data: Line = Line::new(&line);

    assert_eq!(line_data.is_valid_part_two(), false);
    
}

#[test]
fn is_valid_invalid3() {

    // Given a line: 15-16 q: qqqqqnqtqqqbqxqlqqcq
    let line: String = String::from("15-16 q: qqqqqnqtqqqbqxqlqqcq");
    // When we split it
    let line_data: Line = Line::new(&line);

    assert_eq!(line_data.is_valid_part_two(), true);
    
}


#[test]
fn is_valid_valid() {

    // Given a line:     3-15 m: mvmttdmmsmmmkwmx
    let line: String = String::from("3-15 m: mvmttdmmsmmmkwmx");
    // When we split it
    let line_data: Line = Line::new(&line);

    assert_eq!(line_data.is_valid_part_two(), false);
    
}