use std::{collections::HashMap, io};

fn main() {
    // Endless loop through user input
    // Starts with "Add" => Add employee to department => company[department].push(employee)
    // Starts with "List" => List departments subsequently named, or list all staff by dept
    //      sorted alphabetically
    // Starts with "Quit" => Quit
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Command:");
        let mut cmd_str = String::new();
        io::stdin()
            .read_line(&mut cmd_str)
            .expect("Failed to read line");

        // Find the first word
        let first_space = cmd_str.find(' ');
        let first_word = match first_space {
            Some(n) => cmd_str[0..n].to_lowercase(),
            None => cmd_str[0..].trim().to_lowercase(),
        };

        // Match the first word to figure out what we're doing
        match &first_word[0..] {
            "quit" => {
                println!("Goodbye");
                break;
            },
            "list" => list_departments(&company, &cmd_str[first_space.unwrap_or(cmd_str.len() - 1) + 1..]),
            "add" => add_employee(&mut company, &cmd_str[first_space.unwrap_or(cmd_str.len() - 1) + 1..]),
            other => println!("Unexpected command `{other}`"),
        }
    }
}

fn list_departments(company: &HashMap<String, Vec<String>>, s: &str) {
    // s should either have a length of 0 or begin with the second word entered
    if s.len() == 0 {
        list_all_departments(company);
    } else {
        for word in s.split_whitespace() {
            list_department(company, &word.to_string());
        }
    }
}

fn list_all_departments(company: &HashMap<String, Vec<String>>) {
    // Get all departments (keys on company) and sort them
    let mut depts: Vec<String> = company.clone().into_keys().collect();
    depts.sort_unstable();
    for dept in depts {
        list_department(&company, &dept);
    }
}

fn list_department(company: &HashMap<String, Vec<String>>, dept: &String) {
    if company.contains_key(dept) {
        let mut staff = company.get(dept).unwrap_or(&Vec::new()).clone();
        staff.sort_unstable();
        println!("Department: {dept}");
        for s in staff {
            println!("  - {s}");
        }
    } else {
        println!("ERROR: {dept} is not a valid department");
    }
}

fn add_employee(company: &mut HashMap<String, Vec<String>>, s: &str) {
    // Syntax Add <staff> to <dept>. Add has already been eaten
    let mut name = String::new();
    let mut dept = String::new();
    let mut i = 0;
    // TODO: Support multiple names by appending words to name until "to" is found
    for word in s.split_whitespace() {
        match i {
            0 => name = word.to_string(),
            2 => dept = word.to_string(),
            _ => {},
        }
        i += 1;
    }
    if name.len() == 0 || dept.len() == 0 {
        println!("Error: Expected syntax `Add <name> to <department>");
    } else {
        // YAGNI: Case sensitive or not?
        // Are we adding or modifying?
        if company.contains_key(&dept) {
            let mut old_staff = company.get_mut(&dept).unwrap();
            old_staff.push(name);
        } else {
            let mut new_staff = Vec::new();
            new_staff.push(name);
            company.insert(dept, new_staff);
        }
    }
}
