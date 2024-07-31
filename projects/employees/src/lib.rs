use std::{process, io};
use std::collections::HashMap;

pub fn employees() {
    // let's users select to add a new employee to a department, view existing employees in the company or by department, or quit the program.

    let mut map: HashMap<String, String> = HashMap::new();
    
    loop {
        let mut input = String::new();
    
        println!("\nWould you like to 'Add employee', 'View employees', or 'Quit'?");
        io::stdin().read_line(&mut input).expect("Unable to read input.");
        input = input.trim().to_string();

        // matches the user's input and calls the corresponding function
        match input.as_str() {
            "Add employee" => add_employee(&mut map),
            "View employees" => view_employees(&map),
            "Quit" => process::exit(0),
            _ => continue,
        }
    }
    
}

pub fn add_employee(map: &mut HashMap<String, String>) {
    // gets the name of the new employee
    let mut name: String;
    'name: loop {
        name = String::new();
        
        println!("\nPlease enter the name of the new employee, go 'Back', or 'Quit'.");
        io::stdin().read_line(&mut name).expect("Unable to read input.");
        name = name.trim().to_string();

        match name.as_str() {
            "Back" => return,
            "Quit" => process::exit(0),
            _ => (),
        }

        // don't accept empty inputs
        if name.is_empty() {
            continue 'name
        }

        // only accept alphabetical inputs
        for letter in name.chars() {
            if !letter.is_alphabetic() {
                continue 'name
            }
        }
        break
    }

    // gets the department of the new employee
    let mut department: String;
    'department: loop {
        department = String::new();
        
        println!("\nPlease enter the department of the new employee, go 'Back', or 'Quit'.");
        io::stdin().read_line(&mut department).expect("Unable to read input.");
        department = department.trim().to_string();

        match department.as_str() {
            "Back" => return,
            "Quit" => process::exit(0),
            _ => (),
        }

        // don't accept empty inputs
        if department.is_empty() {
            continue 'department
        }

        // only accept alphabetical inputs
        for letter in department.chars() {
            if !letter.is_alphabetic() {
                continue 'department
            }
        }
        break
    }

    // adds employee to the hashmap if they don't exist yet, or updates the employee's department
    map.insert(name, department);
}


pub fn view_employees(map: &HashMap<String, String>) {
    // in company or specific department? get input
    'view: loop {
        let mut input = String::new();
    
        println!("\nWould you like to view employees in the 'Company' or in a specific 'Department'? You can also go 'Back' or 'Quit' the program.");
        io::stdin().read_line(&mut input).expect("Unable to read input.");
        input = input.trim().to_string();

        // don't accept empty inputs
        if input.is_empty() {
            continue 'view
        }

        // only accept alphabetical inputs
        for letter in input.chars() {
            if !letter.is_alphabetic() {
                continue 'view
            }
        }
            
        match input.as_str() {
            "Company" => view_company(map),
            "Department" => view_department(map),
            "Back" => return,
            "Quit" => process::exit(0),
            _ => continue,
        }
    }
}

pub fn view_company(map: &HashMap<String, String>) {
    // sorts the employee name vector to print the names in alphabetical order
    let mut names: Vec<String> = vec![];

    for name in map.keys() {
        names.push(name.to_string());
    }

    names.sort();
    print!("\nCompany employees: {} from {}", &names[0], map.get(&names[0]).unwrap_or(&"NO DEPARTMENT".to_string()));
    for name in &names[1..] {
        print!(", {} from {}", name, map.get(name).unwrap_or(&"NO DEPARTMENT".to_string()));
    }
    print!("\n");
}


pub fn view_department(map: &HashMap<String, String>) {
    // adds all employees from that department from the hashmap to a vector, sorts it, and prints the names
    'view: loop {
        let mut input = String::new();
    
        println!("\nFor which department would you like to see the employees? You can also go 'Back' or 'Quit'?");
        io::stdin().read_line(&mut input).expect("Unable to read input.");
        input = input.trim().to_string();

        // don't accept empty inputs
        if input.is_empty() {
            continue 'view
        }

        // only accept alphabetical inputs
        for letter in input.chars() {
            if !letter.is_alphabetic() {
                continue 'view
            }
        }

        match input.as_str() {
            "Back" => return,
            "Quit" => process::exit(0),
            _ => (),
        }

        // creates a vector with the names of people in that department and sorts it
        let mut names: Vec<String> = vec![];

        for (name, department) in map.iter() {
            if *department == input {
                names.push(name.to_string());
            }
        }
        names.sort();

        print!("\n{} employees: {}", input, &names[0]);
        for name in &names[1..] {
            print!(", {}", name);
        }
        print!("\n");
        return
    }
}