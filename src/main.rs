use chrono::Utc;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, Texture};
use graphics::{Image, clear};
use graphics::rectangle::square;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, Button, MouseButton, MouseCursorEvent, PressEvent, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston_window::*;
use std::path::Path;


mod grid;
use grid::Tile;
use grid::GameInstance;

use crate::grid::{flood_fill, click_on_grid};

// struct Images {
//     box_1: String,
//     box_2: String,
//     box_3: String,
//     box_4:
// }

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    timer: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs, game: &mut GameInstance) {
        use graphics::*;

        //Get 2d vector dimensions
        let tile_size: [f64; 2] = [game.grid_size[0]/game.x_size as f64, game.grid_size[0]/game.y_size as f64];
        //Color format is Red, Green, Blue, Alpha
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const LIGHT_RED: [f32; 4] = [1.0, 0.33, 0.2, 1.0];
        const GRAY: [f32; 4] = [0.76, 0.84, 0.84, 1.0];
        const DARK_GRAY: [f32; 4] = [0.3, 0.4, 0.4, 1.0];
        const SOFT_GRAY: [f32; 4] = [0.56, 0.56, 0.53, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        //let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let timer2 = self.timer;
        //let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GRAY, gl);

            let transform = c
                .transform;

            let row_len = game.y_size;
            let col_len = game.x_size;
            
            //Draw tiles
            let ts = TextureSettings::new();
            let ds = DrawState::default();
            //ts.set_border_color(GRAY);
            //let mut font2 = GlyphCache::new("assets/Inversionz.otf", (), TextureSettings::new()).unwrap();
            

            //Draw Menu header HERE
            if game.game_state == 0{
                let temp: [f64;4] = [
                    0.0,
                    50.0,
                    50.0,
                    50.0,
                ];
                //rectangle::square(30.0, 30.0, 32.0);
                rectangle(RED, temp, c.transform.trans(game.window_size[0]/2.0, game.window_size[1]/3.0), gl);
                let default_button_image = Image::new().rect(rectangle::square(game.window_size[0]/2.0, 80.0, 32.0));
                let default_texture = Texture::from_path(Path::new("./assets/Images/Default_Grid.png"), &ts).unwrap();
                default_button_image.draw(&default_texture, &DrawState::default(), c.transform, gl);
                //game.game_state = 1;
                //text::Text::new_color(BLACK, 12 as u32).draw(&game.flag_count.to_string(), &mut font2, &DrawState::default(), c.transform.trans(30.0, 70.0), gl).unwrap();
            } else {


                let header_background: [f64;4] = [
                    0.0,
                    game.grid_size[1],
                    game.window_size[0],
                    game.window_size[1],
                ];
                let mut glyph_cache = GlyphCache::new("assets/Roboto-Medium.ttf", (), TextureSettings::new()).unwrap();
                let middle_header = game.grid_size[1] + ((game.window_size[1] - game.grid_size[1])/2.0);
                //Draws header background
                rectangle([1.0, 1.0, 1.0, 1.0], header_background, transform, gl);
                //Draw Flag icon
                let flag_image = Image::new().rect(rectangle::square(0.0, middle_header-16.0, 32.0));
                let flag_texture = Texture::from_path(Path::new("assets/tileset_01/flag.png"), &ts).unwrap();
                flag_image.draw(&flag_texture, &ds, transform, gl);
                //Draws flag count
                text::Text::new_color(BLACK, 12 as u32).draw(&game.flag_count.to_string(), &mut glyph_cache, &DrawState::default(), c.transform.trans(30.0, middle_header+4.0), gl).unwrap();
                //Draws time
                //let mut timer = chrono::offset::Utc::now().time() - game.start_time;
                let time_string = (self.timer as i32).to_string();
                text::Text::new_color(BLACK, 12 as u32).draw(&time_string, &mut glyph_cache, &DrawState::default(), c.transform.trans(game.window_size[0]-20.0, middle_header+4.0), gl).unwrap();
                //Draw clock icon
                //TODO: Some reason clock image significantly slows down timer
                // let clock_image = Image::new().rect(rectangle::square(game.window_size[0]-50.0, middle_header-12.0, 24.0));
                // let clock_texture = Texture::from_path(Path::new("assets/tileset_01/clock.png"), &ts).unwrap();
                // clock_image.draw(&clock_texture, &ds, transform, gl);

                let mut x = 0.0;//x position of tile
                let mut y = 0.0;//y position of tile
                //let mut text = gfx_text::new(factory).build().unwrap();
                for row in game.grid.iter() {
                    for tile in row.iter() {
                        let rect: [f64;4] = [
                            x * tile_size[0],
                            y * tile_size[1],
                            tile_size[0],
                            tile_size[1],
                        ];
                        if tile.flagged {
                            rectangle(SOFT_GRAY, rect, transform, gl);
                            //Polygon()
                            //polygon()
                            //let p = c.draw_state.
                            //rectangle()
                        } else if tile.hidden {
                            rectangle(WHITE, rect, transform, gl);
                        } else {
                            if tile.value == -1 {
                                rectangle([1.0, 0.68, 0.68, 1.0], rect, transform, gl);
                                
                            } else {
                                let mut color:[f32; 4];
                                match tile.value{
                                    5 => color = [1.0, 0.78, 1.0, 1.0],
                                    4 => color = [0.74, 0.7, 1.0, 1.0],
                                    3 => color = [0.63, 0.77, 1.0, 1.0],
                                    2 => color = [0.61, 0.96, 1.0, 1.0],
                                    1 => color = [0.79, 1.0, 0.75, 1.0],
                                    0 => color = [0.99, 1.0, 0.71, 1.0],
                                    _ => color = [1.0, 0.84, 0.3529, 1.0],
                                }
                                //let mut color:[f32; 4] = [0.0, 1.0, 0.1 + 0.35*tile.value as f32, 1.0];
                                rectangle(color, rect, transform, gl);
                                if tile.value != 0 {
                                    text::Text::new_color(WHITE, 32/(game.x_size as f32 *0.1) as u32).draw(&tile.value.to_string(), &mut glyph_cache, &DrawState::default(), c.transform.trans(x * tile_size[0] + (tile_size[0]/4.5), (y * tile_size[1]) + (tile_size[1]/1.2)), gl).unwrap();
                                }
                            }
                        }
                        x += 1.0;
                    }
                    y += 1.0;
                    x = 0.0;
                }

                //Draw grid
                for row in 1..row_len as i32 + 1 {
                    line(DARK_GRAY, 1.0, [row as f64 *tile_size[1], 0.0, row as f64 *tile_size[1], game.grid_size[1] as f64], transform, gl);
                }
                for col in 1..col_len as i32 + 1 {
                    line(DARK_GRAY, 1.0, [0.0, col as f64 *tile_size[0], game.grid_size[0] as f64, col as f64 *tile_size[0]], transform, gl);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs, game: &GameInstance) {
        //Only start updating timer once game is started
        if game.bomb_count != 0 {
            self.timer += 1.0 * args.dt;
        }
        
    }

}

fn main() {

    let mut game_start = false;
    let mut game = GameInstance::empty();

    //Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Minesweeper", [game.window_size[0], game.window_size[1]])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        timer: 0.0,
    };
    let mut cursor = [0.0, 0.0];
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        e.mouse_cursor(|pos| cursor=pos);

        if let Some(args) = e.render_args() {
            app.render(&args, &mut game);
            
        }

        if let Some(args) = e.update_args() {
            app.update(&args, &game);
        }
        if let Some(Button::Mouse(button)) = e.press_args() {
            println!("Mouse button pressed: {:?}", button);
            match button {
                MouseButton::Left => {
                    if !game_start {
                        if click_on_grid(&cursor, &game) && game.game_state == 1{
                            grid::plant_bombs(&mut game, cursor);
                            grid::fill_numbers(&mut game);
                            grid::left_click(cursor, &mut game);
                            grid::debug_map(&game.grid, false);
                            game_start = true;
                        }
                    } else {
                        grid::left_click(cursor, &mut game);
                        if grid::check_win(&game) {
                            println!("YOU WIN!")
                    }

                    }
                }
                MouseButton::Right => {
                    if game_start {
                        grid::right_click(cursor, &mut game);
                        if grid::check_win(&game) {
                            println!("YOU WIN!")
                        }
                    }

                }
                _ => ()
            }
        }
    }
    
}

