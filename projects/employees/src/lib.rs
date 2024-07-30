use std::process;
use std::io;

// TODO: create hash map and vector, here? or has to be in main()? need to exist above these functions so they can all access

pub fn employees() {
    // top of the tree, let's users select to add a new employee, view employees, or quit the program.
    // TODO: loop for error handling
    let mut input = String::new();
    
    println!("Would you like to 'Add employee', 'View employees', or 'Quit'?");
    io::stdin().read_line(&mut input).expect("Unable to read input.");
    let s = input.as_str();

    // matches the user's input and calls the corresponding function
    match s {
        "Add employee\r\n" => add_employee(),
        "View employees\r\n" => view_employees(),
        "Quit\r\n" => process::exit(0),
        _ => panic!("Error"),
    }
}

pub fn add_employee() {
    // gets two inputs from the user for the name and department of the employee
    // adds the employee to the hash map if they don't exist already
    // also adds the employee's name to the vector
            println!("Add employee test")
            // input name, if "Quit" then process::exit()
            // input department, if "Quit" then process::exit()
            // add to hashmap and vector
            // employees();
}


pub fn view_employees() {
    // in company or specific department? get input
    // TODO: loop for error handling
    let mut input = String::new();
    
    println!("Would you like to view employees 'In Company' or 'In Department'? You can also go 'Back' or 'Quit' the program.");
    io::stdin().read_line(&mut input).expect("Unable to read input.");
    let s = input.as_str();
        
    match s {
        "In Company\r\n" => view_company(),
        "In Department\r\n" => view_department(),
        "Back\r\n" => employees(),
        "Quit\r\n" => process::exit(0),
        _ => panic!("Error"),
    }
}

pub fn view_company() {
    // sorts the employee name vector and prints the names, then calls employees() to return to the top of the tree

    println!("View company test")
    // sort vector and print

    // employees();
}


pub fn view_department() {
    // adds all employees from that department from the hashmap to a vector, sorts it, and prints the names
    // then calls employees() to return to the top of the tree

    
    println!("View department test")
    // get input for department name, if "Quit" then process::exit()
    // add all department names from hashmap to a vector and sort and print

    // employees();
}
