// we're going to create a big huge adjacency matrix

use std::fmt;
use std::collections::HashMap;

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

fn get_friend_group_hashmap() -> HashMap<String,u8> {
    let mut mymap = HashMap::from([
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

fn master_adjacency_matrix_creator() {
    // in miles, because miles dont change.
    // TODO: fix the empty values
    let adj_matrix: Vec<Vec<usize>> = [ 
                           //Ben, Boris, Calvin, Izzie, Jonas, Javier, Nathan,  Tim,   Voya
                        vec![0,   26,    12,     36,    12,    13,     20,      20,    13], // Ben
                        vec![26,  0,     13,     12,    12,    13,     10,      9,     12], // Boris
                        vec![23,  13,    0,      23,    1,     4,      2,       3,     2],  // Calvin
                        vec![43,  12,    23,     0,     21,    22,     19,      19,    21], // Izzie
                        vec![25,  12,    1,      21,    0,     ,       ,        ,      ],   // Jonas 
                        vec![25,  13,    4,      22,    ,      0,      ,        ,      ],   // Javier
                        vec![28,  10,    2,      19,    ,      ,       0,       ,      ],   // Nathan
                        vec![26,  9,     3,      19,    ,      ,       ,        0,     ],   // Tim
                        vec![27,  12,    2,      21,    ,      ,       ,        ,      0]   // Voya
    ];
    adj_matrix
}

pub fn real_adjacency_matrix_creator() {
    // Retrieve the master adjacency matrix.
    let master_matrix = master_adjacency_matrix_creator();
    let names = vec!["ben", "boris", "calvin", "izzie", "jonas", "javier", "nathan", "tim", "voya"];
    let name_map = get_friend_group_hashmap();

    // Initialize the real adjacency matrix with `i32::MAX` to denote infinity.
    let mut real_matrix = vec![vec![i32::MAX; names.len()]; names.len()];
    
    // Iterate over the provided names and their indices.
    for (i, name) in names.iter().enumerate() {
        // Find the corresponding index in the master matrix using the name map.
        if let Some(&idx) = name_map.get(*name) {
            // For each name, copy the distances from the master matrix to the real matrix.
            for j in 0..names.len() {
                real_matrix[i][j] = master_matrix[idx][j];
            }
        }
    }
    // set the distance to self = 0
    for i in 0..names.len() {
        real_matrix[i][i] = 0;
    }
    // otherwise,
        // set the distance = infinity.
    real_matrix
}
