// use chrono::Utc;

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


mod grid;
use bevy::render::color;
use bevy::render::view::window;
use bevy::sprite::Anchor;
use grid::Tile;
use grid::GameInstance;
use grid::plant_bombs;

use crate::grid::{flood_fill, click_on_grid};

// pub struct App {
//     gl: GlGraphics, // OpenGL drawing backend.
//     rotation: f64,  // Rotation for the square.
//     timer: f64,
// }

// impl App {
//     fn render(&mut self, args: &RenderArgs, game: &mut GameInstance) {
//         use graphics::*;

//         //Get 2d vector dimensions
//         let tile_size: [f64; 2] = [game.grid_size[0]/game.x_size as f64, game.grid_size[0]/game.y_size as f64];
//         //Color format is Red, Green, Blue, Alpha
//         const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
//         const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
//         const LIGHT_RED: [f32; 4] = [1.0, 0.33, 0.2, 1.0];
//         const GRAY: [f32; 4] = [0.76, 0.84, 0.84, 1.0];
//         const DARK_GRAY: [f32; 4] = [0.3, 0.4, 0.4, 1.0];
//         const SOFT_GRAY: [f32; 4] = [0.56, 0.56, 0.53, 1.0];
//         const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
//         const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

//         //let square = rectangle::square(0.0, 0.0, 50.0);
//         let rotation = self.rotation;
//         let timer2 = self.timer;
//         //let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

//         self.gl.draw(args.viewport(), |c, gl| {
//             // Clear the screen.
//             clear(GRAY, gl);

//             let transform = c
//                 .transform;

//             let row_len = game.y_size;
//             let col_len = game.x_size;
            
//             //Draw tiles
//             let ts = TextureSettings::new();
//             let ds = DrawState::default();
//             //ts.set_border_color(GRAY);
//             //let mut font2 = GlyphCache::new("assets/Inversionz.otf", (), TextureSettings::new()).unwrap();
            

//             //Draw Menu header HERE
//             if game.game_state == 0{
//                 let temp: [f64;4] = [
//                     0.0,
//                     50.0,
//                     50.0,
//                     50.0,
//                 ];
//                 //rectangle::square(30.0, 30.0, 32.0);
//                 rectangle(RED, temp, c.transform.trans(game.window_size[0]/2.0, game.window_size[1]/3.0), gl);
//                 let default_button_image = Image::new().rect(rectangle::square(game.window_size[0]/2.0, 80.0, 32.0));
//                 let default_texture = Texture::from_path(Path::new("./assets/Images/Default_Grid.png"), &ts).unwrap();
//                 default_button_image.draw(&default_texture, &DrawState::default(), c.transform, gl);
//                 //game.game_state = 1;
//                 //text::Text::new_color(BLACK, 12 as u32).draw(&game.flag_count.to_string(), &mut font2, &DrawState::default(), c.transform.trans(30.0, 70.0), gl).unwrap();
//             } else {


//                 let header_background: [f64;4] = [
//                     0.0,
//                     game.grid_size[1],
//                     game.window_size[0],
//                     game.window_size[1],
//                 ];
//                 let mut glyph_cache = GlyphCache::new("assets/Roboto-Medium.ttf", (), TextureSettings::new()).unwrap();
//                 let middle_header = game.grid_size[1] + ((game.window_size[1] - game.grid_size[1])/2.0);
//                 //Draws header background
//                 rectangle([1.0, 1.0, 1.0, 1.0], header_background, transform, gl);
//                 //Draw Flag icon
//                 let flag_image = Image::new().rect(rectangle::square(0.0, middle_header-16.0, 32.0));
//                 let flag_texture = Texture::from_path(Path::new("assets/tileset_01/flag.png"), &ts).unwrap();
//                 flag_image.draw(&flag_texture, &ds, transform, gl);
//                 //Draws flag count
//                 text::Text::new_color(BLACK, 12 as u32).draw(&game.flag_count.to_string(), &mut glyph_cache, &DrawState::default(), c.transform.trans(30.0, middle_header+4.0), gl).unwrap();
//                 //Draws time
//                 //let mut timer = chrono::offset::Utc::now().time() - game.start_time;
//                 let time_string = (self.timer as i32).to_string();
//                 text::Text::new_color(BLACK, 12 as u32).draw(&time_string, &mut glyph_cache, &DrawState::default(), c.transform.trans(game.window_size[0]-20.0, middle_header+4.0), gl).unwrap();
//                 //Draw clock icon
//                 //TODO: Some reason clock image significantly slows down timer
//                 // let clock_image = Image::new().rect(rectangle::square(game.window_size[0]-50.0, middle_header-12.0, 24.0));
//                 // let clock_texture = Texture::from_path(Path::new("assets/tileset_01/clock.png"), &ts).unwrap();
//                 // clock_image.draw(&clock_texture, &ds, transform, gl);

//                 let mut x = 0.0;//x position of tile
//                 let mut y = 0.0;//y position of tile
//                 //let mut text = gfx_text::new(factory).build().unwrap();
//                 for row in game.grid.iter() {
//                     for tile in row.iter() {
//                         let rect: [f64;4] = [
//                             x * tile_size[0],
//                             y * tile_size[1],
//                             tile_size[0],
//                             tile_size[1],
//                         ];
//                         if tile.flagged {
//                             rectangle(SOFT_GRAY, rect, transform, gl);
//                             //Polygon()
//                             //polygon()
//                             //let p = c.draw_state.
//                             //rectangle()
//                         } else if tile.hidden {
//                             rectangle(WHITE, rect, transform, gl);
//                         } else {
//                             if tile.value == -1 {
//                                 rectangle([1.0, 0.68, 0.68, 1.0], rect, transform, gl);
                                
//                             } else {
//                                 let mut color:[f32; 4];
//                                 match tile.value{
//                                     5 => color = [1.0, 0.78, 1.0, 1.0],
//                                     4 => color = [0.74, 0.7, 1.0, 1.0],
//                                     3 => color = [0.63, 0.77, 1.0, 1.0],
//                                     2 => color = [0.61, 0.96, 1.0, 1.0],
//                                     1 => color = [0.79, 1.0, 0.75, 1.0],
//                                     0 => color = [0.99, 1.0, 0.71, 1.0],
//                                     _ => color = [1.0, 0.84, 0.3529, 1.0],
//                                 }
//                                 //let mut color:[f32; 4] = [0.0, 1.0, 0.1 + 0.35*tile.value as f32, 1.0];
//                                 rectangle(color, rect, transform, gl);
//                                 if tile.value != 0 {
//                                     text::Text::new_color(WHITE, 32/(game.x_size as f32 *0.1) as u32).draw(&tile.value.to_string(), &mut glyph_cache, &DrawState::default(), c.transform.trans(x * tile_size[0] + (tile_size[0]/4.5), (y * tile_size[1]) + (tile_size[1]/1.2)), gl).unwrap();
//                                 }
//                             }
//                         }
//                         x += 1.0;
//                     }
//                     y += 1.0;
//                     x = 0.0;
//                 }

//                 //Draw grid
//                 for row in 1..row_len as i32 + 1 {
//                     line(DARK_GRAY, 1.0, [row as f64 *tile_size[1], 0.0, row as f64 *tile_size[1], game.grid_size[1] as f64], transform, gl);
//                 }
//                 for col in 1..col_len as i32 + 1 {
//                     line(DARK_GRAY, 1.0, [0.0, col as f64 *tile_size[0], game.grid_size[0] as f64, col as f64 *tile_size[0]], transform, gl);
//                 }
//             }
//         });
//     }

//     fn update(&mut self, args: &UpdateArgs, game: &GameInstance) {
//         //Only start updating timer once game is started
//         if game.bomb_count != 0 {
//             self.timer += 1.0 * args.dt;
//         }
        
//     }

// }

// fn main() {

//     let mut game_start = false;
//     let mut game = GameInstance::empty();

//     //Change this to OpenGL::V2_1 if not working.
//     let opengl = OpenGL::V3_2;
//     // Create a Glutin window.
//     let mut window: Window = WindowSettings::new("Minesweeper", [game.window_size[0], game.window_size[1]])
//         .graphics_api(opengl)
//         .exit_on_esc(true)
//         .build()
//         .unwrap();

//     // Create a new game and run it.
//     let mut app = App {
//         gl: GlGraphics::new(opengl),
//         rotation: 0.0,
//         timer: 0.0,
//     };
//     let mut cursor = [0.0, 0.0];
//     let mut events = Events::new(EventSettings::new());
//     while let Some(e) = events.next(&mut window) {

//         e.mouse_cursor(|pos| cursor=pos);

//         if let Some(args) = e.render_args() {
//             app.render(&args, &mut game);
            
//         }

//         if let Some(args) = e.update_args() {
//             app.update(&args, &game);
//         }
//         if let Some(Button::Mouse(button)) = e.press_args() {
//             println!("Mouse button pressed: {:?}", button);
//             match button {
//                 MouseButton::Left => {
//                     if !game_start {
//                         if click_on_grid(&cursor, &game) && game.game_state == 1{
//                             grid::plant_bombs(&mut game, cursor);
//                             grid::fill_numbers(&mut game);
//                             grid::left_click(cursor, &mut game);
//                             grid::debug_map(&game.grid, false);
//                             game_start = true;
//                         }
//                     } else {
//                         grid::left_click(cursor, &mut game);
//                         if grid::check_win(&game) {
//                             println!("YOU WIN!")
//                     }

//                     }
//                 }
//                 MouseButton::Right => {
//                     if game_start {
//                         grid::right_click(cursor, &mut game);
//                         if grid::check_win(&game) {
//                             println!("YOU WIN!")
//                         }
//                     }

//                 }
//                 _ => ()
//             }
//         }
//     }
    
// }
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Start,
    Playing,
    GameOver,
}
const X_RESOLUTION: f32 = 400.0;
const Y_RESOLUTION: f32 = 400.0;

#[derive(Component)]
struct GameComponent (GameInstance);

fn main() {
    //let mut game = GameInstance::default();
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugin(HelloPlugin)
        .add_startup_system(create_game_instance)
        .add_startup_system(window_resize_system)
        .add_state(GameState::Start)
        .add_system_set(SystemSet::on_enter(GameState::Start).with_system(draw_grid))
        .add_system(mouse_button_input)
        //.add_system(draw_grid)
        //.add_startup_stage_after(StartupStage::Startup, "d", draw_grid)
        //.add_startup_system(draw_grid)
        //.add_stage_after(draw_grid, "create_game")
        //.add_system(print_query)
        //.add_system(draw_grid)
        //.add_startup_system(render_grid)
        .run();
}
fn create_game_instance(mut commands: Commands){
    let mut game = GameInstance::default();
    //let mut game = game.0;
    // grid::plant_bombs(&mut game, [100.0, 0.0]);
    // grid::fill_numbers(&mut game);
    // grid::left_click([100.0, 0.0], &mut game);
    grid::debug_map(&game.grid, false);
    commands.spawn().insert(GameComponent(game));
    //GameInstance
    // GameInstancegrid::plant_bombs(&mut game, cursor);
    // grid::fill_numbers(&mut game);
}
fn window_resize_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    //println!("Window size was: {},{}", window.width(), window.height());
    window.set_resolution(X_RESOLUTION, Y_RESOLUTION);
}

fn draw_grid(
    mut query: Query<&mut GameComponent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    let mut game = query.single_mut();

    commands.spawn_bundle(Camera2dBundle::default());
    let tile_size = 40.0;//((game.0.grid_size[0]+1.0)/10.0) as f32;
    let mut x = 0.0;//x position of tile
    let mut y = 0.0;//y position of tile
    for row in game.0.grid.iter() {
        for tile in row.iter() {
            let mut color_holder: bevy::prelude::Color;
            if tile.flagged {
                color_holder = Color::rgb(0.56, 0.56, 0.53);
            } else if tile.hidden {
                color_holder = Color::rgb(1.0, 1.0, 1.0);
                //rectangle(WHITE, rect, transform, gl);
            } else {
                if tile.value == -1 {
                    color_holder = Color::rgb(1.0, 0.68, 0.68);
                    //rectangle([1.0, 0.68, 0.68, 1.0], rect, transform, gl);
                } else {
                    match tile.value{
                        5 => color_holder = Color::rgb(1.0, 0.78, 1.0),
                        4 => color_holder = Color::rgb(0.74, 0.7, 1.0),
                        3 => color_holder = Color::rgb(0.63, 0.77, 1.0),
                        2 => color_holder = Color::rgb(0.61, 0.96, 1.0),
                        1 => color_holder = Color::rgb(0.79, 1.0, 0.75),
                        0 => color_holder = Color::rgb(0.99, 1.0, 0.71),
                        _ => color_holder = Color::rgb(1.0, 0.84, 0.3529),
                    }
                }
            }
            let coords = convert_coords(x*40.0, y*40.0);
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: color_holder,
                    custom_size: Some(Vec2::new(40.0, 40.0)),
                    anchor: Anchor::TopLeft,
                    
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(coords[0], coords[1], 0.0),
                    //scale: 1.0,
                    ..default()
                },
                ..default()
            });
        x += 1.0;
    }
    y += 1.0;
    x = 0.0;
    }

    for row in 1..game.0.grid_size[0] as i32 + 1 {
        let coords = convert_coords(0.0, (row as f32) * tile_size);
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.76, 0.84, 0.84),
                custom_size: Some(Vec2::new(game.0.grid_size[0] as f32, 2.0)),
                anchor: Anchor::TopLeft,
                
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(coords[0], coords[1], 0.0),
                //scale: 1.0,
                ..default()
            },
            ..default()
        });
    }
    for col in 1..game.0.grid_size[1] as i32 + 1 {
        let coords = convert_coords((col as f32) * tile_size, 0.0);
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.76, 0.84, 0.84),
                custom_size: Some(Vec2::new(2.0, game.0.grid_size[0] as f32)),
                anchor: Anchor::TopLeft,
                
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(coords[0], coords[1], 0.0),
                //scale: 1.0,
                ..default()
            },
            ..default()
        });
    }
}

fn convert_coords(x_coord: f32, y_coord: f32) -> [f32;2]{
    let mut coords:[f32;2] = [0.0, 0.0];

    if x_coord >= X_RESOLUTION/2.0 {
        coords[0] = x_coord - X_RESOLUTION/2.0;
    } else {
        coords[0] = (X_RESOLUTION/2.0 - x_coord) * -1.0;
    }
    if y_coord >= Y_RESOLUTION/2.0 {
        coords[1] = (y_coord - Y_RESOLUTION/2.0) * -1.0;
    } else {
        coords[1] = (Y_RESOLUTION/2.0 - y_coord);
    }
    coords
}

fn print_query(mut query: Query<&mut GameComponent>){
    let game = query.single_mut();
    println!("{}", game.0.x_size);
}
fn mouse_button_input(
    mut query: Query<&mut GameComponent>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut game_state: ResMut<State<GameState>>,
) {
    let mut game = query.single_mut();
    let window = windows.get_primary().unwrap();
    if buttons.just_pressed(MouseButton::Left) {
        // Left button was pressed
        if let Some(_position) = window.cursor_position() {
            // cursor is inside the window, position given
            //if let GameState::Start = GameState {
            println!("X: {}, Y: {}", _position.x, _position.y);
            match game_state.current() {
                GameState::Start => {
                    let cursor = convert_coords(_position.x, _position.y);
                    if click_on_grid(&cursor, &game.0){
                        grid::plant_bombs(&mut game.0, cursor);
                        grid::fill_numbers(&mut game.0);
                        grid::left_click(cursor, &mut game.0);
                        grid::debug_map(&game.0.grid, false);
                        game_state.set(GameState::Playing).unwrap();
                    }
                }
                GameState::Playing => {
                    let cursor = convert_coords(_position.x, _position.y);
                    if click_on_grid(&cursor, &game.0){
                        grid::left_click(cursor, &mut game.0);
                        grid::debug_map(&game.0.grid, false);
                    }
                }
                _ => {}
            }

            }
        } else {
            // cursor is not inside the window
        }
        
    if buttons.just_pressed(MouseButton::Right) {
        // Right Button is being held down
    }
}
// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     commands.spawn_bundle(Camera2dBundle::default());

//     // Rectangle
//     commands.spawn_bundle(SpriteBundle {
//         sprite: Sprite {
//             color: Color::rgb(0.25, 0.25, 0.75),
//             custom_size: Some(Vec2::new(50.0, 100.0)),
//             ..default()
//         },
//         ..default()
//     });

//     // Circle
//     commands.spawn_bundle(MaterialMesh2dBundle {
//         mesh: meshes.add(shape::Circle::new(50.).into()).into(),
//         material: materials.add(ColorMaterial::from(Color::PURPLE)),
//         transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
//         ..default()
//     });

//     // Hexagon
//     commands.spawn_bundle(MaterialMesh2dBundle {
//         mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
//         material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
//         transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
//         ..default()
//     });
// }