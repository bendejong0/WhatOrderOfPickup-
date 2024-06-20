// we're going to create a big huge adjacency matrix

use std::fmt;
use std::collections::HashMap;
use std::u8::MAX;

// names are alphabetized
// currently unused but may be used later
enum NamesAndCorrespondingNumbers {
    Ben = 1,
    Boris = 2,
    Calvin = 3,
    Izzie = 4,
    Jonas = 5,
    Javier = 6,
    Nathan = 7,
    Tim = 8,
    Voya = 9,
}

// currently unused but may be used later
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
            NamesAndCorrespondingNumbers::Tim => write!(f, "Tim"),
            NamesAndCorrespondingNumbers::Voya => write!(f, "Voya"),
        }
    }
}

fn get_friend_group_hashmap() -> HashMap<&'static str,u8> {
    let mut mymap: HashMap<&str, u8> = HashMap::from([
    ("ben", 1),
    ("boris", 2),
    ("calvin", 3),
    ("izzie", 4),
    ("jonas", 5),
    ("javier", 6),
    ("nathan", 7),
    ("tim", 8),
    ("voya", 9)
    ]);

    
    mymap
}

pub fn master_adjacency_matrix_creator() -> Vec<Vec<u8>> {
    // in miles, because miles dont change.
    // TODO: fix the empty values
    let adj_matrix: Vec<Vec<u8>> = [ 
                           //Ben, Boris, Calvin, Izzie, Jonas, Javier, Nathan,  Tim,   Voya
                        vec![0,   26,    12,     36,    12,    13,     20,      20,    13], // Ben
                        vec![26,  0,     13,     12,    12,    13,     10,      9,     12], // Boris
                        vec![23,  13,    0,      23,    1,     4,      2,       3,     2],  // Calvin
                        vec![43,  12,    23,     0,     21,    22,     19,      19,    21], // Izzie
                        vec![25,  12,    1,      21,    0,     MAX,    MAX,     MAX,   MAX],// Jonas 
                        vec![25,  13,    4,      22,    MAX,   0,      MAX,     MAX,   MAX],// Javier
                        vec![28,  10,    2,      19,    MAX,   MAX,    0,       MAX,   MAX],// Nathan
                        vec![26,  9,     3,      19,    MAX,   MAX,    MAX,     0,     MAX],// Tim
                        vec![27,  12,    2,      21,    MAX,   MAX,    MAX,     MAX,   0]   // Voya
    ].to_vec();
    adj_matrix
}

pub fn real_adjacency_matrix_creator(pick_up_these_people: Vec<String>) -> Vec<Vec<u8>>{
    // Retrieve the master adjacency matrix.
    let master_matrix = master_adjacency_matrix_creator();
    let names = vec!["ben", "boris", "calvin", "izzie", "jonas", "javier", "nathan", "tim", "voya"];
    let name_map = get_friend_group_hashmap();

    // Initialize the real adjacency matrix with `i32::MAX` to denote infinity.
    let mut real_matrix = vec![vec![MAX; pick_up_these_people.len()]; pick_up_these_people.len()];
    
    // Iterate over the provided names and their indices.
    /*for (i, name) in pick_up_these_people.iter().enumerate() {
        // Find the corresponding index in the master matrix using the name map.
        if let Some(&idx) = name_map.get(name.as_str()) {
            // For each name, copy the distances from the master matrix to the real matrix.
            for j in 0..names.len() {
                real_matrix[i][j] = master_matrix[idx as usize][j] as u8;
            }
        }
    }*/

    // Figure out who needs to get picked up
    let pick_up_these_people_indices: Vec<u8> = pick_up_these_people.iter()
        .filter_map(|name| name_map.get(name.as_str()).cloned())
        .collect();

    // Iterate over the people who need to get picked up
    for &i in &pick_up_these_people_indices {
        // gather the mileage at each person's row and column to the other people in the vector
        for &j in &pick_up_these_people_indices {
            // if i and j or j and i
            if usize::from(i) < master_matrix.len() && usize::from(j) < master_matrix.len() {
                real_matrix.push(vec![master_matrix[usize::from(i)][usize::from(j)]]);
            }
        }
    }

    // set the distance to self = 0
    for i in 0..pick_up_these_people_indices.len() {
        real_matrix[i][i] = 0;
    }
    real_matrix
}
