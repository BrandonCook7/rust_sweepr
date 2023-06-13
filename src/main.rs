// extern crate glutin_window;
// extern crate graphics;
// extern crate opengl_graphics;
// extern crate piston;

// use glutin_window::GlutinWindow as Window;
// use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, Texture};
// use graphics::{Image, clear};
// use graphics::rectangle::square;
// use piston::event_loop::{EventSettings, Events};
// use piston::input::{RenderArgs, Button, MouseButton, MouseCursorEvent, PressEvent, RenderEvent, UpdateArgs, UpdateEvent};
// use piston::window::WindowSettings;
// use piston_window::*;
// use std::path::Path;

use grid::create_grid;
//External Packages
use macroquad::prelude::*;
use macroquad::text;
use macroquad::ui::widgets;
use macroquad::ui::widgets::InputText;
use std::collections::HashMap;
use std::process::exit;
use std::string;
use std::thread::sleep;
use macroquad::ui::{
    hash, root_ui,
    widgets:: {Group},
    Drag, Ui,
};

use chrono::Utc;

//Internal Packages
mod grid;
use grid::Tile;
use grid::GameInstance;

use crate::grid::{flood_fill, click_on_grid};



fn window_conf() -> Conf {
    Conf {
        window_title: "Minesweeper".to_owned(),
        window_width: 400,
        window_height: 450,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

async fn limit_fps(frame_limit: f32){
    let minimum_frame_time = 1.0 / frame_limit;
    let frame_time = get_frame_time();
    if frame_time < minimum_frame_time {
        next_frame().await
        //let time_to_sleep = (minimum_frame_time - frame_time) * 1000.0;
        //std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
    }
}

// enum GameStatus {
//     Menu,
//     Playing,
//     Lost,
//     Won,
//     PreGame //When the user clicks play but has not selected a tile
// }

//#[macroquad::main("Minesweeper")]
#[macroquad::main(window_conf)]
async fn main() {
    let mut texture_map: HashMap<String,Texture2D> = HashMap::new();
    let mut texture: Texture2D = load_texture("assets/tileset_01/clock.png").await.unwrap();
    texture_map.insert("clock".to_string(), texture);
    texture = load_texture("assets/tileset_01/flag.png").await.unwrap();
    texture_map.insert("flag".to_string(), texture);
    texture = load_texture("assets/Images/Default_Grid.png").await.unwrap();
    texture_map.insert("grid1".to_string(), texture);
    texture = load_texture("assets/Images/background.png").await.unwrap();
    texture_map.insert("background".to_string(), texture);
    texture = load_texture("assets/Images/flag-marker.png").await.unwrap();
    texture_map.insert("flag_marker".to_string(), texture);

    let font = load_ttf_font("assets/Roboto-Medium.ttf").await.unwrap();
    let mut game = GameInstance::default();
    let mut in_menu: bool = true;
    //let mut game_status = GameStatus::Menu;
    let mut x_input: f32 = 10.0;
    let mut y_input: f32 = 10.0;
    loop {
        limit_fps(35.0).await;
        // if game.game_state == -1 {
        //     game.game_status = grid::GameStatus::Lost;
        // }
        match game.game_status {
            grid::GameStatus::Playing => {
                render(&mut game, &texture_map, &Some(font)); 
                check_clicks(&mut game);
            },
            grid::GameStatus::PreGame => {
                render(&mut game, &texture_map, &Some(font)); 
                check_clicks(&mut game);
            },
            grid::GameStatus::Menu => {
                game.game_status = draw_menu(&mut game, &texture_map, &Some(font), &mut x_input, &mut y_input);
                x_input = x_input.floor();
                y_input = y_input.floor();
            },
            grid::GameStatus::Lost => {
                //draw_texture_ex(texture_map["background"], 0.0, 0.0, WHITE, DrawTextureParams { dest_size: Some(Vec2{x: 400.0, y: 500.0}), ..Default::default()});
                for row in 0..(game.grid.len() as i32) {
                    for col in 0..(game.grid[0].len() as i32)  {
                        game.grid[row as usize][col as usize].hidden = false;
                        game.grid[row as usize][col as usize].flagged = false;
                    }
                }
                render(&mut game, &texture_map, &Some(font)); 
                let win_height = 100.0;
                let win_width = 150.0;
                
                widgets::Window::new(hash!(), vec2((game.window_size[0]/2.) -(win_width/2.), (game.window_size[1]/2.) -(win_height/2.)), vec2(150.0, 100.0))
                .label("You Lost")
                .titlebar(true)
                .ui(&mut *root_ui(), |ui| {
                    //TODO: Allow grid to render x and y sizes independently with each cell still being a square
                    //ui.slider(hash!(), "[4 .. 20]", 4f32..20f32, y_input);
                    //x_input.round();
                    if ui.button(Vec2::new(57.0, 20.0), "Menu"){
                        //let grid_x = x_input.;
                        game.game_status = grid::GameStatus::Menu;
                    }
                    if ui.button(Vec2::new(57.0, 45.0), "Quit"){
                        exit(0);
                    }
                });
                
            },
            grid::GameStatus::Won => {
                for row in 0..(game.grid.len() as i32) {
                    for col in 0..(game.grid[0].len() as i32)  {
                        game.grid[row as usize][col as usize].hidden = false;
                    }
                }
                render(&mut game, &texture_map, &Some(font)); 
                let win_height = 100.0;
                let win_width = 150.0;
                
                widgets::Window::new(hash!(), vec2((game.window_size[0]/2.) -(win_width/2.), (game.window_size[1]/2.) -(win_height/2.)), vec2(150.0, 100.0))
                .label("You Won")
                .titlebar(true)
                .ui(&mut *root_ui(), |ui| {
                    //TODO: Allow grid to render x and y sizes independently with each cell still being a square
                    //ui.slider(hash!(), "[4 .. 20]", 4f32..20f32, y_input);
                    //x_input.round();
                    if ui.button(Vec2::new(57.0, 20.0), "Menu"){
                        //let grid_x = x_input.;
                        game.game_status = grid::GameStatus::Menu;
                    }
                    if ui.button(Vec2::new(57.0, 45.0), "Quit"){
                        exit(0);
                    }
                });
            }
        }
        next_frame().await;
        std:: thread ::sleep(std::time::Duration::from_millis(30));
    }
}

// fn lost_animation(game: &mut GameInstance, texture_map: &HashMap<String,Texture2D>, font: &Option<Font>) {
//     for row in game.grid.iter_mut() {
//         for tile in row.iter_mut() {
//             tile.hidden = false;
//             render(game, texture_map, font);
//         }
//     }
// }

fn render(game: &mut GameInstance, texture_map: &HashMap<String,Texture2D>, font: &Option<Font>) {
    clear_background(WHITE);
    draw_grid(game, &texture_map, font);
    draw_info_bar(game, &texture_map, font)
}
fn draw_menu(game: &mut GameInstance, texture_map: &HashMap<String,Texture2D>, font: &Option<Font>, x_input: &mut f32, y_input: &mut f32) -> grid::GameStatus{
    let mut in_menu = true;
    //Draws background for menu
    draw_texture_ex(texture_map["background"], 0.0, 0.0, WHITE, DrawTextureParams { dest_size: Some(Vec2{x: 400.0, y: 500.0}), ..Default::default()});
    widgets::Window::new(hash!(), vec2(50.0, 200.0), vec2(300.0, 150.0))
    .label("Menu")
    .titlebar(true)
    .ui(&mut *root_ui(), |ui| {
        ui.slider(hash!(), "[4 .. 20]", 4f32..20f32, x_input);

        if ui.button(Vec2::new(100.0, 50.0), "Start"){
            game.game_status = grid::GameStatus::PreGame;
            game.x_size = x_input.floor() as usize;
            game.y_size = x_input.floor() as usize;
            game.grid = create_grid(game.x_size, game.y_size);
            in_menu = false;
        }
        if ui.button(Vec2::new(150.0, 50.0), "Quit"){
            exit(0);
        }
    });

    draw_text_ex("MineSweeper", (game.window_size[0]/2.0)-80.0, game.window_size[1]/4.0,
    TextParams {
        font_size: 25,
        font: font.unwrap_or(Default::default()),
        color: BLACK,
        ..Default::default()
    },);
    if in_menu {
        return grid::GameStatus::Menu;
    } else {
        return grid::GameStatus::PreGame;
    }
    
}

fn check_clicks(game: &mut GameInstance) -> bool{
    let cursor_tuple = mouse_position();
    let cursor = [cursor_tuple.0, cursor_tuple.1];
    if is_mouse_button_pressed(MouseButton::Left){
        //Checking if this is the first click
        if game.game_status == grid::GameStatus::PreGame{
            if click_on_grid(&cursor, &game){
                grid::plant_bombs(game, cursor);
                grid::fill_numbers(game);
                grid::left_click(cursor, game);
                grid::debug_map(&game.grid, false);
                game.game_status = grid::GameStatus::Playing;
                game.start_time = chrono::offset::Utc::now().time();
                println!("Click on Grid");
            }
        } else if game.game_status == grid::GameStatus::Playing{
            grid::left_click(cursor, game);
            if grid::check_win(&game) {
                println!("YOU WIN!");
                game.game_status = grid::GameStatus::Won;
            }
        }
        return true;
    } else if is_mouse_button_pressed(MouseButton::Right){
        if game.game_status == grid::GameStatus::Playing {
            grid::right_click(cursor, game);
            if grid::check_win(&game) {
                println!("YOU WIN!");
                game.game_status = grid::GameStatus::Won;
            }
        }  
        return true;
    } else {
        return false;
    }

}
fn draw_info_bar(game: &mut GameInstance, texture_map: &HashMap<String,Texture2D>, font: &Option<Font>) {
    let middle_header = game.grid_size[1] + ((game.window_size[1] - game.grid_size[1])/2.0);

    //Draw Timer
    let timer = chrono::offset::Utc::now().time() - game.start_time;
    let mut time_string = timer.num_seconds().to_string();
    if game.game_status == grid::GameStatus::PreGame{
        time_string = String::from("0");
    }
    draw_text_ex(&time_string, game.window_size[0]-45.0, middle_header+10.0,
    TextParams {
        font_size: 25,
        font: font.unwrap_or(Default::default()),
        color: BLACK,
        ..Default::default()
    },);
    //Draw Flag Count
    draw_texture_ex(texture_map["flag"], 5.0, middle_header-20.0, WHITE, DrawTextureParams { dest_size: Some(Vec2{x: 40.0, y: 40.0}), ..Default::default()});
    draw_text_ex(&game.flag_count.to_string(), 45.0, middle_header+8.0,
    TextParams {
        font_size: 25,
        font: font.unwrap_or(Default::default()),
        color: BLACK,
        ..Default::default()
    },);
}
//
fn draw_grid(game: &mut GameInstance, texture_map: &HashMap<String,Texture2D>, font: &Option<Font>) {
    let tile_size: [f32; 2] = [game.grid_size[0]/game.x_size as f32, game.grid_size[0]/game.y_size as f32];
    let mut x = 0.0;//x position of tile
    let mut y = 0.0;//y position of tile
    //let mut text = gfx_text::new(factory).build().unwrap();
    for row in game.grid.iter() {
        for tile in row.iter() {

            if tile.flagged {
                draw_rectangle(x * tile_size[0], y * tile_size[1], tile_size[0], tile_size[1], Color::new(1.0, 0.941, 0.831, 1.0));
                draw_texture_ex(texture_map["flag_marker"], x * tile_size[0],  y * tile_size[1], WHITE, DrawTextureParams { dest_size: Some(Vec2{x: 40.0, y: 40.0}), ..Default::default()});
            } else if tile.hidden {
                draw_rectangle(x * tile_size[0], y * tile_size[1], tile_size[0], tile_size[1], WHITE);
            } else {
                if tile.value == -1 {
                    draw_rectangle(x * tile_size[0], y * tile_size[1], tile_size[0], tile_size[1], Color::new(1.0, 0.68, 0.68, 1.0));
                } else {
                    let color_holder: Color;
                    match tile.value{
                        5 => color_holder = Color::new(1.0, 0.78, 1.0, 1.0),
                        4 => color_holder = Color::new(0.74, 0.7, 1.0, 1.0),
                        3 => color_holder = Color::new(0.63, 0.77, 1.0, 1.0),
                        2 => color_holder = Color::new(0.61, 0.96, 1.0, 1.0),
                        1 => color_holder = Color::new(0.79, 1.0, 0.75, 1.0),
                        0 => color_holder = Color::new(0.99, 1.0, 0.71, 1.0),
                        _ => color_holder = Color::new(1.0, 0.84, 0.3529, 1.0),
                    }

                    draw_rectangle(x * tile_size[0], y * tile_size[1], tile_size[0], tile_size[1], color_holder);
                    
                    if tile.value != 0 {
                        draw_text_ex(&tile.value.to_string(), x * tile_size[0] + (tile_size[0]/4.0), (y * tile_size[1]) + (tile_size[1]/1.25),
                        TextParams {
                            font_size: (32.0/(game.x_size as f32 *0.1)) as u16,
                            font: font.unwrap_or(Default::default()),
                            color: WHITE,
                            ..Default::default()
                        },);
                    }
                }
            }
            x += 1.0;
        }
        y += 1.0;
        x = 0.0;
    }

    let row_len = game.y_size;
    let col_len = game.x_size;
    for row in 1..row_len as i32 + 1 {
        draw_line(row as f32 *tile_size[1], 0.0,row as f32 *tile_size[1], game.grid_size[1] as f32, 2.0, DARKGRAY);
    }
    for col in 1..col_len as i32 + 1 {
        draw_line(0.0, col as f32 *tile_size[0], game.grid_size[0] as f32, col as f32 *tile_size[0], 2.0, DARKGRAY);
    }
}