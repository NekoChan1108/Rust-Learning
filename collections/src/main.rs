//!
//! the answer of chapter 8
//!

// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::thread::sleep;

///
/// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
/// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
/// # Examples
/// ```
/// let list = vec![1, 2, 3, 4, 5, 5, 5];
/// println!("{:?}", median_and_mode(&list));
/// ```
///
fn median_and_mode(list: &Vec<i32>) -> (Option<i32>, Option<i32>) {
    if list.is_empty() {
        return (None, None);
    }
    let mut median: Option<i32> = None;
    let mut count_map: HashMap<i32, i32> = HashMap::new();
    if list.len() % 2 == 1 {
        median = Some(list[list.len() / 2]);
    } else {
        median = Some(list[list.len() / 2 - 1] + list[list.len() / 2] / 2);
    }
    for i in list {
        let count = count_map.entry(*i).or_insert(0);
        *count += 1;
    }
    let mode = count_map
        .into_iter()
        .max_by_key(|(_, v)| *v)
        .map(|(k, _)| k);

    (median, mode)
}

/// Convert strings to pig latin. The first consonant of each word
/// is moved to the end of the word and “ay” is added, so “first”
/// becomes “irst-fay.” Words that start with a vowel have “hay”
/// added to the end instead (“apple” becomes “apple-hay”). Keep
/// in mind the details about UTF-8 encoding!
/// # Examples
/// ```
/// assert_eq!(pig_latin(&mut String::from("apple")), "apple-hay");
/// ```
fn pig_latin(s: &mut String) -> String {
    let s = s.trim();
    if s.is_empty() {
        return "".to_string();
    }
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let mut new_s = String::new();
    for word in s.split_ascii_whitespace() {
        if word.is_empty() {
            return "".to_string();
        }
        let c = word.chars().nth(0);
        if c.is_none() || (c.is_some() && c.unwrap().is_whitespace()) {
            return "".to_string();
        }
        if VOWELS.contains(&c.unwrap()) {
            // println!("{:?}", word.splitn(2, c.unwrap()).collect::<Vec<&str>>()[1]);
            new_s.push_str(format!("{}-hay", word).as_str());
        } else {
            new_s.push_str(
                format!(
                    "{}-{}ay",
                    word.splitn(2, c.unwrap()).collect::<Vec<&str>>()[1],
                    c.unwrap()
                )
                .as_str(),
            );
            new_s.push_str(" ");
        }
    }
    new_s.trim().to_string()
}

/// Using a hash map and vectors, create a text interface
/// to allow a user to add employee names to a department
/// in a company. For example, “Add Sally to Engineering”
/// or “Add Amir to Sales.” Then let the user retrieve a
/// list of all people in a department or all people in
/// the company by department, sorted alphabetically.

struct Company {
    department: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Self {
        Company {
            department: HashMap::new(),
        }
    }

    fn add_employee(&mut self, name: &str, department: &str) {
        self.department
            .entry(department.to_string())
            .or_insert(Vec::new())
            .push(name.to_string());
    }
    fn get_employees(&self, department: &str) -> Option<Vec<String>> {
        if self.department.get(department).is_none() {
            return None;
        }
        self.department.get(department).map(|employees| {
            let mut clone_employees = employees.clone();
            clone_employees.sort();
            clone_employees
        })
    }
    fn get_all_employees(&self) -> Vec<(String, Vec<String>)> {
        let mut result: Vec<(String, Vec<String>)> = self
            .department
            .iter()
            .map(|(department, employees)| {
                let mut clone_employees = employees.clone();
                clone_employees.sort();
                (department.clone(), clone_employees)
            })
            .collect();
        result.sort_by(|a, b| a.0.cmp(&b.0));
        result
    }
}

enum Command {
    Add { name: String, department: String },
    ListDepartment { department: String },
    ListAll,
}

fn parse_command(input: &str) -> Result<Command, String> {
    let parts: Vec<&str> = input.trim().split_ascii_whitespace().collect();
    match parts.as_slice() {
        ["Add", name, "to", department] => Ok(Command::Add {
            name: name.to_string(),
            department: department.to_string(),
        }),
        ["List", "employees", "in", department] => Ok(Command::ListDepartment {
            department: department.to_string(),
        }),
        ["List", "all", "employees"] => Ok(Command::ListAll),
        _ => Err("Invalid command".to_string()),
    }
}

fn main() {
    println!("{:?}", median_and_mode(&vec![1, 2, 3, 4, 5, 5, 5]));
    println!("{}", pig_latin(&mut String::from("apple")));
    println!("{}", pig_latin(&mut String::from(" first second under")));

    let mut company = Company::new();
    loop {
        println!("Enter command (enter 'quit' to exit) :");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let res = io::stdin().read_line(&mut input);
        if res.is_err() {
            println!("Error reading input");
            continue;
        }
        let input = input.trim();
        if input.eq_ignore_ascii_case("quit") {
            println!("Exit");
            break;
        }
        match parse_command(input) {
            Ok(command) => match command {
                Command::Add { name, department } => {
                    company.add_employee(&name, &department);
                    println!("Added {} to {}", name, department);
                }
                Command::ListDepartment { department } => {
                    match company.get_employees(&department) {
                        Some(employees) => {
                            println!("Employees in {} are {:?}:", department, employees);
                        }
                        None => println!("No employees found in {}", department),
                    }
                }
                Command::ListAll => {
                    let all_employees = company.get_all_employees();
                    println!("All employees:");
                    for (department, employees) in all_employees {
                        println!("{} : {:?}", department, employees);
                    }
                }
            },
            Err(err) => println!("{}", err),
        }
    }
}
