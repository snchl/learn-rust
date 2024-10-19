use std::{collections::HashMap, io};

enum MenuChoice {
    AddEmployee,
    ListDepartmentEmployee,
    ListCompanyEmployee,
    Exit,
    None,
}

fn menu_ui() {
    println!("########################################################");
    println!("# Welcome, how can I help you?                         #");
    println!("# 1- Add employee in a department                      #");
    println!("# 2- List department's employees                       #");
    println!("# 3- List all company's employees (sort by department) #");
    println!("# 0- Exit                                              #");
    println!("########################################################");
}

fn take_menu_choice() -> MenuChoice {
    println!("You want:");

    let mut choice_input = String::new();

    io::stdin()
        .read_line(&mut choice_input)
        .expect("Failed to read your choice!");

    // Remove enter newline input
    choice_input.pop();

    match choice_input.as_str() {
        "1" => MenuChoice::AddEmployee,
        "2" => MenuChoice::ListDepartmentEmployee,
        "3" => MenuChoice::ListCompanyEmployee,
        "0" => MenuChoice::Exit,
        _ => MenuChoice::None,
    }
}

fn menu_action(menu_choice: MenuChoice, department_employees: &mut HashMap<String, Vec<String>>, mut wrong_try_count: i32) {
    match menu_choice {
        MenuChoice::AddEmployee => {
            add_employee(department_employees);

            if continue_question() {
                app(department_employees, 0);
            }
        }
        MenuChoice::ListDepartmentEmployee => {
            list_department_employee(department_employees);

            if continue_question() {
                app(department_employees, 0);
            }
        }
        MenuChoice::ListCompanyEmployee => {
            list_all_employees_per_department(&department_employees);

            if continue_question() {
                app(department_employees, 0);
            }
        },
        MenuChoice::Exit => println!("Goodbye!"),
        MenuChoice::None => {
            wrong_try_count += 1;
            if wrong_try_count == 3 {
                println!("You choose three time an invalid option, for security we close your session.");
                println!("Goodbye!");
            } else {
                println!("You choose an invalid option, try again");
                app(department_employees, wrong_try_count);
            }
        }
    }
}

fn add_employee(department_employees: &mut HashMap<String, Vec<String>>) {
    println!("In which department do you want to add a new employee?");
    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read your choice!");
    // Remove enter newline input
    department.pop();

    println!("What is the name of this new employee?");
    let mut employee = String::new();
    io::stdin()
        .read_line(&mut employee)
        .expect("Failed to read your choice!");
    // Remove enter newline input
    employee.pop();

    let employees_vec = department_employees.entry(department).or_insert(Vec::new());

    employees_vec.push(employee);
}

fn list_department_employee(department_employees: &mut HashMap<String, Vec<String>>) {
    println!("Which department would you like to consult?");
    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read your choice!");
    // Remove enter newline input
    department.pop();

    let employees_vec = department_employees.get_mut(&department);

    match employees_vec {
        Some(employees) => {
            employees.sort();
            print_department_employees(&department, employees);
        },
        None => println!("The department {} doesn't exist", department)
    }
}

fn list_all_employees_per_department(department_employees: &HashMap<String, Vec<String>>) {
    let mut departments: Vec<&String> = department_employees.keys().collect();

    departments.sort();

    for department in departments {
        let employees = department_employees.get(department);

        match employees {
            Some(value) => print_department_employees(department, value),
            None => {}
        }
    }
}

fn print_department_employees(department: &str, employees: &Vec<String>) {
    println!("Department: {department}");

    for employee in employees {
        println!("- {employee}");
    }
}

fn continue_question() -> bool {
    println!("Do you want to continue? Y/n");
    let mut continue_input = String::new();
    io::stdin()
        .read_line(&mut continue_input)
        .expect("Failed to read your choice!");
    // Remove enter newline input
    continue_input.pop();

    continue_input != "n"
}

fn app(department_employees: &mut HashMap<String, Vec<String>>, wrong_try_count: i32) {
        menu_ui();
        let menu_choice = take_menu_choice();
        menu_action(menu_choice, department_employees, wrong_try_count);
}

fn main() {
    let mut department_employees: HashMap<String, Vec<String>> = HashMap::new();

    app(&mut department_employees, 0);
}
