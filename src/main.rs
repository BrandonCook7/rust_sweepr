extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod grid;
use grid::Tile;

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
    fn render(&mut self, args: &RenderArgs, vec: Vec<Vec<Tile>>) {
        use graphics::*;

        //Get 2d vector dimensions
        let row_len = vec.len();
        let col_len = vec[0].len();
        // let tile_size: [f64; 2] = [args.window_size[0]/row_len as f64, args.window_size[1]/row_len as f64];
        //Color format is Red, Green, Blue,  Alpha
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GRAY: [f32; 4] = [0.76, 0.84, 0.84, 1.0];
        const DARK_GRAY: [f32; 4] = [0.36, 0.4, 0.4, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GRAY, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

                //.trans(x, y);
                //.trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.

            //rectangle(RED, square, transform, gl);
            //line(GREEN, 2.0, [0.0, 0.0, 15.0, 15.0], transform2, gl);
            vec = self.draw_grid(vec, &mut gl, c, args);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }

    fn draw_grid(&self, mut vec: Vec<Vec<Tile>>, &mut gl: &mut opengl_graphics::GlGraphics, c: graphics::Context, args: &RenderArgs) -> Vec<Vec<Tile>>{
        use graphics::*;
    
        let row_len = vec.len();
        let col_len = vec[0].len();
        
    
        let transform2 = c
        .transform;
    
        let tile_size: [f64; 2] = [args.window_size[0]/row_len as f64, args.window_size[1]/row_len as f64];
    
    
        const GRAY: [f32; 4] = [0.76, 0.84, 0.84, 1.0];
        const DARK_GRAY: [f32; 4] = [0.36, 0.4, 0.4, 1.0];
    
        for row in 1..row_len as i32 {
            line(DARK_GRAY, 1.0, [row as f64 *tile_size[1], 0.0, row as f64 *tile_size[1], args.window_size[1] as f64], transform2, gl);
        }
        for col in 1..col_len as i32 {
            line(DARK_GRAY, 1.0, [0.0, col as f64 *tile_size[0], args.window_size[0] as f64, col as f64 *tile_size[0]], transform2, gl);
        }
        vec
    }
}



fn main() {
    let mut grid_size: [i32; 2] = [0, 0];

    let mut vec = grid::create_grid();
    println!("vec: {:?}", vec[0][0].value);
    vec = grid::plant_bombs(vec);
    grid::debug_map(&vec, false);


    //Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [400, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, vec.clone());
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

