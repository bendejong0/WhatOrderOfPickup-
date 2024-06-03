use std::io;

fn get_names() -> Vec::<String>{
    // initially, we'll store a name of all the people who need to be picked up
    let mut names = Vec::<String>::new();

    // query the user for the names
    println!("Who do you need to pick up?");
    // create a spot to hold the name
    let mut name: String = " ".to_string();

    // kinda doing this really ugly but i dont really care.
    // I could probabyl clean this up using some kind of recursion, but thats a TODO
    while name.trim() != "q"{
        // list of friends
        println!("Boris, Ben, Calvin, Jonas, Javier, Nathan, Voya");
        println!("Please input a name and hit enter. When you are done, put in q");
        // you have to clear the name for some reason.
        name.clear();
        // save the user input
        io::stdin().read_line(&mut name)
            .expect("Failed to read line");

        // convert to all lowercase for easier processing
        let name_to_add = name.trim().to_lowercase();

        if names.contains(&name_to_add){
            println!("{name_to_add} is already in the list!\n");
            continue;
        }

        // if it's "q" then just break.
        if name_to_add == "q" {
            break;
        }

        // add to the vector of names
        names.push(name_to_add.clone());

        // give a confirmation.
        println!("{} added successfully!\n", name_to_add);
    }
    // return the vector.
    names
}

fn list_names(names: Vec::<String>){
    println!("To be picked up:");
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
    let run_pathfinder: bool = confirm_names();
    print!("{run_pathfinder}");
}
