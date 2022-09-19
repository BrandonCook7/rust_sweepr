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

pub struct GameInstance {
    pub x_size: usize,
    pub y_size: usize,
    pub grid: Vec<Vec<Tile>>,
    pub window_size: [f64; 2],
    pub flag_count: i32,
    pub bomb_count: i32,
}
impl Default for GameInstance {
    fn default() -> GameInstance {
        GameInstance {
            x_size: 10,
            y_size: 10,
            grid: create_grid(10, 10),
            window_size: [400.0, 400.0],
            flag_count: 0,
            bomb_count: 0,
        }
    }
}
impl GameInstance {
    fn large() -> GameInstance {
        GameInstance {
            x_size: 20,
            y_size: 20,
            grid: create_grid(20, 20),
            window_size: [400.0, 400.0],
            flag_count: 0,
            bomb_count: 0,
        }
    }
}

pub fn create_grid(x_size: usize, y_size: usize) -> Vec<Vec<Tile>> {
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
pub fn plant_bombs(game: &mut GameInstance){
    let row_len = game.grid.len();
    let col_len = game.grid[0].len();
    let tile_count = game.grid.len() * game.grid[0].len();//Get the total number of tiles
    let mut rng = thread_rng();
    let y: f64 = rng.gen_range(0.75, 1.25);
    let mut bomb_count = ((tile_count/5) as f64 * y).round() as i32;
    println!("Bomb Count {}", bomb_count);
    while bomb_count > 0 {
        let row = rng.gen_range(0, row_len);
        let col = rng.gen_range(0, col_len);
        if game.grid[row as usize][col as usize].value != -1 {
            game.grid[row as usize][col as usize] = Tile::bomb();
            bomb_count -= 1;
        }
    }
    game.bomb_count = bomb_count;
    game.flag_count = bomb_count;
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
//Converts mouse coordinates to tile
fn find_tile(coords: [f64; 2], game: &GameInstance) -> [f64; 2]{
    let tile_size: [f64; 2] = [game.window_size[0]/game.x_size as f64, game.window_size[1]/game.y_size as f64];
    let x = coords[0] / tile_size[0];
    let y = coords[1] / tile_size[1];
    //println!("X: {}, Y: {}", x, y);
    [x.floor(), y.floor()]
}

//Calculates what value each tile should have
pub fn fill_numbers(game: &mut GameInstance){
    for row in 0..game.x_size as i32 {
        for col in 0..game.y_size as i32 {
            if game.grid[row as usize][col as usize].value != -1{
                let mut bomb_surronding = 0;
                //Top Left
                let mut x = 0;
                let mut y = 0;
                x = row - 1;y = col - 1;
                bomb_surronding += check_bomb(&game, x, y);
                //Top middle
                x = row - 1;y = col;
                bomb_surronding += check_bomb(&game, x, y);
                //Top Right
                x = row - 1;y = col + 1;
                bomb_surronding += check_bomb(&game, x, y);
                //Middle Left
                x = row;y = col - 1;
                bomb_surronding += check_bomb(&game, x, y);
                //Middle Right
                x = row;y = col + 1;
                bomb_surronding += check_bomb(&game, x, y);
                //Bottom Left
                x = row + 1; y = col - 1;
                bomb_surronding += check_bomb(&game, x, y);
                //Bottom Middle
                x = row + 1;y = col;
                bomb_surronding += check_bomb(&game, x, y);
                //Bottom Right
                x = row + 1; y = col + 1;
                bomb_surronding += check_bomb(&game, x, y);
                game.grid[row as usize][col as usize].value = bomb_surronding as i8;
            }
        }
    }
}

//Returns 1 if there is a bomb at coords
fn check_bomb(game: &GameInstance, x: i32, y: i32) -> i32{
    if x >= 0 && y >= 0 && x < game.x_size as i32 && y < game.y_size as i32 {
        println!("X: {}, Y {}",x, y);
        if game.grid[x as usize][y as usize].value == -1 {
            return 1;
        }
    }
    return 0;
    
}

//Checks if given coordinates is within bounds of game grid and is a value of 0
fn valid_empty(game: &GameInstance, x: i32, y: i32) -> bool{
    if x >= 0 && y >= 0 && x < game.x_size as i32 && y < game.y_size as i32 {
        if game.grid[x as usize][y as usize].value == 0 && game.grid[x as usize][y as usize].hidden == true {
            return true;
        }
    }
    return false;
}

//Uses flood fill algorithm to show empty adjacent tiles
pub fn flood_fill(game: &mut GameInstance, x: i32, y: i32) -> Vec<[i32; 2]>{
    let mut queue: Vec<[i32; 2]> = Vec::new();
    //This vector is to remember where all revealed tiles are for show_empty_edges()
    let mut shown_tiles: Vec<[i32; 2]> = Vec::new();

    //Create structure for storing coords
    let mut p = [x,y];
    //Add tile to back of queue
    queue.push(p);
    shown_tiles.push(p);



    game.grid[x as usize][y as usize].hidden = false;

    while queue.len() > 0{
        //Take the tile from back of queue
        let current_tile = queue[queue.len() - 1];
        queue.pop();

        let pos_x = current_tile[0];
        let pos_y = current_tile[1];

        if valid_empty(&game, pos_x + 1, pos_y) {
            game.grid[(pos_x + 1) as usize][pos_y as usize].hidden = false;
            p[0] = pos_x + 1;
            p[1] = pos_y;
            if !queue.contains(&p){
                queue.push(p);
                shown_tiles.push(p);
            }
            
        }
        if valid_empty(&game, pos_x - 1, pos_y) {
            game.grid[(pos_x - 1) as usize][pos_y as usize].hidden = false;
            p[0] = pos_x - 1;
            p[1] = pos_y;
            if !queue.contains(&p){
                queue.push(p);
                shown_tiles.push(p);
            }
        }
        if valid_empty(&game, pos_x, pos_y + 1) {
            game.grid[pos_x as usize][(pos_y + 1) as usize].hidden = false;
            p[0] = pos_x;
            p[1] = pos_y + 1;
            if !queue.contains(&p){
                queue.push(p);
                shown_tiles.push(p);
            }
        }
        if valid_empty(&game, pos_x, pos_y - 1) {
            game.grid[pos_x as usize][(pos_y - 1) as usize].hidden = false;
            p[0] = pos_x;
            p[1] = pos_y - 1;
            if !queue.contains(&p){
                queue.push(p);
                shown_tiles.push(p);
            }
        }
    }
    shown_tiles

}
//Reveals all tiles next to empty tiles
fn show_empty_edges(game: &mut GameInstance, shown_tiles: Vec<[i32; 2]>){
    let mut check_coords: Vec<[i32; 2]> = vec![[-1,-1],[-1,0],[-1,1],
                                           [0,-1],[0,1],
                                           [1,-1],[1,0],[1,1]];
    for i in 0..shown_tiles.len(){
        for j in 0..check_coords.len() {
            let x = shown_tiles[i][0] + check_coords[j][0];
            let y = shown_tiles[i][1] + check_coords[j][1];
            if x >= 0 && y >= 0 && x < game.x_size as i32 && y < game.y_size as i32 {
                let val = game.grid[(shown_tiles[i][0] + check_coords[j][0]) as usize][(shown_tiles[i][1] + check_coords[j][1]) as usize].value;
                if val != 0 && val != -1{
                    game.grid[(shown_tiles[i][0] + check_coords[j][0]) as usize][(shown_tiles[i][1] + check_coords[j][1]) as usize].hidden = false;
                }
            }
        }
    }
}

pub fn left_click(coords: [f64; 2], game: &mut GameInstance){
    let tile_coords = find_tile(coords, &game);
    if game.grid[tile_coords[1] as usize][tile_coords[0] as usize].value == -1 {
        //Do a animation for losing here
        println!("You Lost!");
    }
    if game.grid[tile_coords[1] as usize][tile_coords[0] as usize].hidden == true {
        game.grid[tile_coords[1] as usize][tile_coords[0] as usize].hidden = false;
        if game.grid[tile_coords[1] as usize][tile_coords[0] as usize].value == 0 {
            let shown_tiles = flood_fill(game, tile_coords[1].floor() as i32, tile_coords[0].floor() as i32);
            show_empty_edges(game, shown_tiles);
        }
    }
}

pub fn right_click(coords: [f64; 2], game: &mut GameInstance){
    let tile_coords = find_tile(coords, &game);
    if game.grid[tile_coords[1] as usize][tile_coords[0] as usize].hidden == true {
        if game.grid[tile_coords[1] as usize][tile_coords[0] as usize].flagged == false {
            game.grid[tile_coords[1] as usize][tile_coords[0] as usize].flagged = true;
            println!("FLAGGED");
        } else {
            game.grid[tile_coords[1] as usize][tile_coords[0] as usize].flagged = false;
        }
    }
    
}