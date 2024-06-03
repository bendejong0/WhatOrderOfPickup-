use std::io;

//TODO: 
// put the different functions in different files. Probably group the menu files in a file, and the solver in a file.
// get data from each persons house (this part is INSANELY important, but it'll be annoying)
// allow editing of the names vector provided by get_names() i.e., if you forgot to you needed to pick up someone, or if you accidentally added someone.
// 

// This function gets the initial names
fn get_names() -> Vec::<String>{
    // initially, we'll store a name of all the people who need to be picked up
    let mut names = Vec::<String>::new();

    // here's a list of valid names
    // TODO: research a better way than to do "name".to_string();
    let valid_names = vec!["javier".to_string(), "boris".to_string(), "voya".to_string(), "ben".to_string(), "calvin".to_string(), "jonas".to_string(), "nathan".to_string(), "izzie".to_string()];

    // query the user for the names
    println!("Who do you need to pick up?");
    // create a spot to hold the name
    let mut name: String = " ".to_string();

    // kinda doing this really ugly but i dont really care.
    // I could probabyl clean this up using some kind of recursion, but thats a TODO
    while name.trim() != "q"{
        // list of friends
        println!("Boris, Ben, Calvin, Izzie, Jonas, Javier, Nathan, Voya");
        println!("Please input a name and hit enter. When you are done, put in q");
        // you have to clear the name for some reason.
        name.clear();
        // save the user input
        io::stdin().read_line(&mut name)
            .expect("Failed to read line");

        // convert to all lowercase for easier processing
        let name = name.trim().to_lowercase();

        // if its not a valid name,
        if !valid_names.contains(&name) && &name!="q" {
            // tell the user,
            println!("\nError: invalid name! Please stick to the valid names.\n");
            continue;
        }

        if names.contains(&name){
            println!("{name} is already in the list!\n");
            continue;
        }

        // if it's "q" then just break.
        if name == "q" {
            break;
        }

        // add to the vector of names
        names.push(name.clone());

        // give a confirmation.
        println!("{} added successfully!\n", name);
    }
    // return the vector.
    names
}

fn list_names(names: Vec::<String>){
    println!("To be picked up:");
    if names.len() == 0 {
        println!("None! Why are you using this?");
    }
    for name in names {
        print!("{name}, ");
    }
}

fn confirm_names() -> bool {

    println!("\n");
    println!("Is this all the people?");
    println!("(Options are Yes, Y, N, or No)");

    let mut user_choice: String = " ".to_string();
    io::stdin().read_line(&mut user_choice)
        .expect("Failed to read user_choice in confirm_names()");
    user_choice = user_choice.trim().to_lowercase();

    match user_choice.as_str(){
        "yes" | "y" => true,
        "no" | "n" => false,
        _ => {
            println!("Please choose a valid option!");
            confirm_names()
        }
    }
}

fn main(){
    // get the names of the people to be picked up.
    let names = get_names();
    print!("\n");
    // print them so the user can see.
    list_names(names);
    // this is to make sure its ready to run the pathfinding algorithm
    let run_pathfinder: bool = confirm_names();
    // actually do something later. this is to make sure its working lol.
    print!("{run_pathfinder}");
}
