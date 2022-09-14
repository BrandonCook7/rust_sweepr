extern crate rand;

use rand::thread_rng;   
use rand::Rng;

#[derive(Clone)]
pub struct Tile {
    pub hidden: bool, // false is seen, true is hidden
    pub value: i8, // -1 is bomb, 0+ is how many bombs are nearby
    pub flagged: bool //Boolean for if the user flagged the bomb
}

impl Tile {
    fn new() -> Tile {
        Tile {
            hidden: true,
            value: 0,
            flagged: false,
        }
    }
    fn bomb() -> Tile {
        Tile {
            hidden: true,
            value: -1,
            flagged: false
        }
    }
}

pub fn create_grid() -> Vec<Vec<Tile>> {
    //TODO: Randomly create sizes
    const x_size: usize = 10;
    const y_size: usize = 10;
    // let s = Square{
    //     hidden: true,
    //     value: 0,
    // };
    let s = Tile::new();
    let mut vec = vec![vec![s; x_size]; y_size];
    //let mut arr: Vec<i32> = [[Square{hidden: true, value: 0}; 10], 10];
    //let mut arr:[[Square; 10]; 10] = [[s; 10]; 10];
    return vec;
}

//Bad designed plant_bombs func
pub fn plant_bombs2(mut vec: Vec<Vec<Tile>>) -> Vec<Vec<Tile>>{
    let row_len = vec.len();
    let col_len = vec[0].len();
    let tile_count = vec.len() * vec[0].len();//Get the total number of tiles
    let mut rng = thread_rng();
    let y: f64 = rng.gen_range(0.75, 1.25);
    let mut bomb_count = ((tile_count/6) as f64 * y).round() as i32;
    println!("Bomb Count {}", bomb_count);
    let mut col_index = 0;
    let mut row_index = 0;
    let chance = bomb_count as f32/tile_count as f32; // Chance there should be a bomb
    while bomb_count > 0 && row_index < row_len{
        let gen = rng.gen_range(0.0, 1.0);
        if gen <= chance {
            println!("PLANT");
            vec[row_index][col_index] = Tile::bomb(); //Plant bomb if chance 
            bomb_count -= 1;
        }
        col_index += 1;
        if col_index == col_len {
            row_index += 1;
            col_index = 0;
        }
    }
    println!("Left over Bombs: {}", bomb_count);
    println!("Last Loc col: {}, row: {}", col_index, row_index);

    vec
}
//Function that plants bombs over the grid
pub fn plant_bombs(mut vec: Vec<Vec<Tile>>) -> Vec<Vec<Tile>>{
    let row_len = vec.len();
    let col_len = vec[0].len();
    let tile_count = vec.len() * vec[0].len();//Get the total number of tiles
    let mut rng = thread_rng();
    let y: f64 = rng.gen_range(0.75, 1.25);
    let mut bomb_count = ((tile_count/6) as f64 * y).round() as i32;
    println!("Bomb Count {}", bomb_count);
    while bomb_count > 0 {
        let row = rng.gen_range(0, row_len);
        let col = rng.gen_range(0, col_len);
        if vec[row as usize][col as usize].value != -1 {
            vec[row as usize][col as usize] = Tile::bomb();
            bomb_count -= 1;
        }
    }
    vec
}

//Debugging function to see how grid looks
//Important: Uses & to barrow the vector
pub fn debug_map(vec: &Vec<Vec<Tile>>, hidden: bool){
    let row_len = vec.len();
    let col_len = vec[0].len();
    for row in 0..row_len as i32 {
        for col in 0..col_len as i32 {
            //Check if function ask to keep values hidden
            if vec[row as usize][col as usize].hidden && hidden {
                print!("X");
            } else {
                print!("{}", vec[row as usize][col as usize].value);
            }
        }
        println!("");
    }
}