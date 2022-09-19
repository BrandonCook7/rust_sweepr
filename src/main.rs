extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, Button, MouseButton, MouseCursorEvent, PressEvent, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston_window::*;


mod grid;
use grid::Tile;
use grid::GameInstance;

use crate::grid::flood_fill;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}



// impl App {
//     fn render(&mut self, args: &RenderArgs) {
//         use graphics::*;

//         const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
//         const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

//         let square = rectangle::square(0.0, 0.0, 50.0);
//         let rotation = self.rotation;
//         let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

//         self.gl.draw(args.viewport(), |c, gl| {
//             // Clear the screen.
//             clear(GREEN, gl);

//             let transform = c
//                 .transform
//                 .trans(x, y)
//                 .rot_rad(rotation)
//                 .trans(-25.0, -25.0);

//             // Draw a box rotating around the middle of the screen.
//             rectangle(RED, square, transform, gl);
//         });
//     }

//     fn update(&mut self, args: &UpdateArgs) {
//         // Rotate 2 radians per second.
//         self.rotation += 2.0 * args.dt;
//     }
// }
impl App {
    fn render(&mut self, args: &RenderArgs, game: &GameInstance) {
        use graphics::*;

        //Get 2d vector dimensions
        let tile_size: [f64; 2] = [args.window_size[0]/game.x_size as f64, args.window_size[1]/game.y_size as f64];
        //Color format is Red, Green, Blue,  Alpha
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GRAY: [f32; 4] = [0.76, 0.84, 0.84, 1.0];
        const DARK_GRAY: [f32; 4] = [0.36, 0.4, 0.4, 1.0];
        const WHITE: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        const BLACK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GRAY, gl);

            let transform = c
                .transform;

            let row_len = game.y_size;
            let col_len = game.x_size;
            
            //Draw tiles

            let mut glyph_cache = GlyphCache::new("assets/Roboto-Medium.ttf", (), TextureSettings::new()).unwrap();
    
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
                        rectangle(DARK_GRAY, rect, transform, gl);
                    } else if tile.hidden {
                        rectangle(WHITE, rect, transform, gl);
                    } else {
                        if tile.value == -1 {
                            rectangle(RED, rect, transform, gl);
                        } else {
                            let mut color:[f32; 4] = [0.0, 1.0, 0.2*tile.value as f32, 1.0];
                            rectangle(color, rect, transform, gl);
                            if tile.value != 0 {
                                text::Text::new_color(BLACK, 32/(game.x_size as f32 *0.1) as u32).draw(&tile.value.to_string(), &mut glyph_cache, &DrawState::default(), c.transform.trans(x * tile_size[0] + (tile_size[0]/4.5), (y * tile_size[1]) + (tile_size[1]/1.2)), gl).unwrap();
                            }
                        }
                    }
                    x += 1.0;
                }
                y += 1.0;
                x = 0.0;
            }

            //Draw grid
            for row in 1..row_len as i32 {
                line(DARK_GRAY, 1.0, [row as f64 *tile_size[1], 0.0, row as f64 *tile_size[1], args.window_size[1] as f64], transform, gl);
            }
            for col in 1..col_len as i32 {
                line(DARK_GRAY, 1.0, [0.0, col as f64 *tile_size[0], args.window_size[0] as f64, col as f64 *tile_size[0]], transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs, game: &GameInstance) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        //Check for tile clicks here
        
    }

}



fn main() {

    let mut game = GameInstance::default();
    println!("vec: {:?}", game.grid[0][0].value);
    grid::plant_bombs(&mut game);
    grid::debug_map(&game.grid, false);
    grid::fill_numbers(&mut game);
    grid::debug_map(&game.grid, false);
    


    //Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Minesweeper", [game.window_size[0], game.window_size[1]])
        //.graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };
    let mut cursor = [0.0, 0.0];
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        e.mouse_cursor(|pos| cursor=pos);

        if let Some(args) = e.render_args() {
            app.render(&args, &game);
            
        }

        if let Some(args) = e.update_args() {
            app.update(&args, &game);
        }
        if let Some(Button::Mouse(button)) = e.press_args() {
            println!("Mouse button pressed: {:?}", button);
            match button {
                MouseButton::Left => {
                    println!("Left mouse button pressed");
                    grid::left_click(cursor, &mut game);
                }
                MouseButton::Right => {
                    println!("Right mouse button pressed");
                    grid::right_click(cursor, &mut game);
                }
                _ => ()
            }
        }
    }
    
}

