// we're going to create a big huge adjacency matrix

use std::fmt;

// names are alphabetized
enum NamesAndCorrespondingNumbers {
    Ben = 1,
    Boris = 2,
    Calvin = 3,
    Izzie = 4,
    Jonas = 5,
    Javier = 6,
    Nathan = 7,
    Tim = 8,
    Voya = 9
}

impl fmt::Display for NamesAndCorrespondingNumbers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NamesAndCorrespondingNumbers::Ben => write!(f, "Ben"),
            NamesAndCorrespondingNumbers::Boris => write!(f, "Boris"),
            NamesAndCorrespondingNumbers::Calvin => write!(f, "Calvin"),
            NamesAndCorrespondingNumbers::Izzie => write!(f, "Izzie"),
            NamesAndCorrespondingNumbers::Jonas => write!(f, "Jonas"),
            NamesAndCorrespondingNumbers::Javier => write!(f, "Javier"),
            NamesAndCorrespondingNumbers::Nathan => write!(f, "Nathan"),
            NamesAndCorrespondingNumbers::Voya => write!(f, "Voya"),
        }
    }
}

fn master_adjacency_matrix_creator() {
    let adj_matrix = [ //Ben, Boris, Calvin, Izzie, Jonas, Javier, Nathan,  Tim,   Voya
                        [0],   [32],  [23],   [43],   [25],  [25],  [28],   [26],  [27], //Ben
                        [32],  [0],   [],     [],     [],    [],    [],     [],    [],   // Boris
                        [23],  [],    [0],    [],     [],    [],    [],     [],    [],   // Calvin
                        [43],  [],    [],     [0],    [],    [],    [],     [],    [],   // Izzie
                        [25],  [],    [],     [],     [0],   [],    [],     [],    [],   // Jonas 
                        [25],  [],    [],     [],     [],    [0],   [],     [],    [],   // Javier
                        [28],  [],    [],     [],     [],    [],    [0],    [],    [],   // Nathan
                        [26],  [],    [],     [],     [],    [],    [],     [0],   [],   // Tim
                        [27],  [],    [],     [],     [],    [],    [],     [],    [0]   // Voya
    ];
    adj_matrix
}

fn real_adjacency_matrix_creator() {
    // if the person needs to get picked up, load in the distances.
    // otherwise,
        // set the distance = infinity.
}

// temporary main, delete later.
fn main(){
    
}
