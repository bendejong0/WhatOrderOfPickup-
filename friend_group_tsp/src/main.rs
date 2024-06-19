use std::collections::HashSet;
use std::io;
mod solver;
use solver::dijkstra_path;
mod friendgroupdata;
pub use friendgroupdata::real_adjacency_matrix_creator;

//TODO: 
// put the different functions in different files. Probably group the menu files in a file, and the solver in a file.
// get data from each persons house (this part is INSANELY important, but it'll be annoying)
// allow editing of the names vector provided by get_names() i.e., if you forgot to you needed to pick up someone, or if you accidentally added someone.
// 

// This function gets the initial names
fn get_names(valid_names: &HashSet<String>) -> Vec<String> {
    let mut names = Vec::<String>::new();
    let mut input = String::new();

    println!("Who do you need to pick up?");
    println!("Boris, Ben, Calvin, Izzie, Jonas, Javier, Nathan, Voya");

    loop {
        println!("Please input a name and hit enter. When you are done, put in q");
        input.clear();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let name = input.trim().to_lowercase();

        if name == "q" {
            break;
        }

        if !valid_names.contains(&name) {
            println!("\nError: invalid name! Please stick to the valid names.\n");
            continue;
        }

        if names.iter().any(|n| n == &name) {
            println!("{} is already in the list!\n", name);
            continue;
        }

        names.push(name.clone());
        println!("{} added successfully!\n", name);
    }

    names.sort();
    names
}

// this functions prints all the names in a given vector.
fn list_names(names: &[String]) {
    println!("To be picked up:");
    if names.is_empty() {
        println!("None! Why are you using this?");
    } else {
        for name in names {
            print!("{}, ", name);
        }
    }
}

// this function asks the user for confirmation that they have all the correct names.
fn confirm_names(names: &Vec<String>) -> bool {
    list_names(&names);
    println!("\nIs this all the people?");
    println!("(Options are Yes, Y, N, or No)");

    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice)
        .expect("Failed to read user choice");
    user_choice = user_choice.trim().to_lowercase();

    match user_choice.as_str() {
        "yes" | "y" => true,
        "no" | "n" => false,
        _ => {
            println!("Please choose a valid option!");
            confirm_names(&names)
        }
    }
}

// Add a name to the names vector
fn add_to_names(names: &mut Vec<String>) {
    print!("Enter a name: ");
    let mut name: String = "junkval".to_string();
    io::stdin().read_line(&mut name)
        .expect("Failed to read name in add_to_names()");
    name = name.trim().to_lowercase();

    if !names.contains(&name) {
        names.push(name.to_lowercase());
    }
}

// Remove a name from the names vector
fn remove_from_names(names: &mut Vec<String>) {
    println!("Enter the name to remove:");
    let mut name_to_remove = String::new();
    io::stdin().read_line(&mut name_to_remove)
        .expect("Failed to read name to remove");
    names.retain(|name| name.to_lowercase() != name_to_remove.to_lowercase());
}


fn add_or_remove_names(names: &mut Vec<String>) {
    let mut userchoice: String = "filler_text".to_string();
    userchoice.clear();

    println!("Would you like to (A)dd or (R)emove a name? Or input Q to quit.");
    io::stdin().read_line(&mut userchoice)
        .expect("Failed to get user choice in add_or_remove_names()");
    userchoice=userchoice.trim().to_lowercase();
    match userchoice.as_str() {
        "a" => add_to_names(names),
        "r" => remove_from_names(names),
        "q" => return,
        _ => {
            println!("Invalid choice");
            add_or_remove_names(names);
            },
    }
}

fn main() {
    let valid_names: HashSet<String> = [
        "javier", "boris", "voya", "ben", "calvin", "jonas", "nathan", "izzie", "tim"
    ].iter().map(|&s| s.to_string()).collect();

    let mut names: Vec<String> = get_names(&valid_names);
    println!();

    let mut ready_to_pathfind = confirm_names(&names);

    while !ready_to_pathfind {
        // Add or remove names logic
        println!("Press q to quit");
        ready_to_pathfind = confirm_names(&names);

        if !ready_to_pathfind{
            add_or_remove_names(&mut names);
        }

    }
    
}