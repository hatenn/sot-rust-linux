extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
use crate::settings::settings::{SCREEN_HEIGHT, SCREEN_WIDTH};

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use glfw_window::GlfwWindow;
use input::{Button, Key, MouseButton, PressEvent};
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::EventLoop;
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use window::{AdvancedWindow, Window};
use crate::gametypes::Service;

use crate::viewtypes::{Actor, BACKGROUND, ObjectType};

pub struct View {
    gl: GlGraphics, // OpenGL drawing backend.
    window : GlfwWindow,
    glyphs : GlyphCache<'static>, //Font
    pub update_actors: Arc<Mutex<Vec<Actor>>>,
    render_actors: Vec<Actor>,
}

impl View {
    pub fn new(window_width : f64, window_height : f64, window_pos_x : i32, window_pos_y : i32) -> Arc<Mutex<Vec<Actor>>> {
        //Creating a dynamic array to add actors to render
        let update_actors : Arc<Mutex<Vec<Actor>>> = Arc::new(Mutex::new(Vec::with_capacity(100)));

        //Cloning our list to pass it to the overlay/rendering thread
        let list_obj = Arc::clone(&update_actors);

        thread::spawn(move ||{
            View::start_overlay(list_obj, window_width, window_height, window_pos_x, window_pos_y);
        });

        return update_actors;
    }

    fn start_overlay(update_actors : Arc<Mutex<Vec<Actor>>>, window_width : f64, window_height : f64, window_pos_x : i32, window_pos_y : i32) {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        let mut window = GlfwWindow::new(
            &WindowSettings::new("SimpleScreenRecorder", (window_width, window_height))
                .fullscreen(false)
                .transparent(true)
                .vsync(true)
                .decorated(true) //borderless
                .graphics_api(opengl)
        ).unwrap();

        // Set window position and size
        window.set_position([window_pos_x, window_pos_y]);

        //Loading a font
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
            .expect("Could not load font");

        let mut view = View {
            gl: GlGraphics::new(opengl),
            window: window,
            glyphs: glyphs,
            update_actors: update_actors,
            render_actors: Vec::with_capacity(100),
        };

        view.render_loop();
    }

    fn render_loop(&mut self){
        let mut events = Events::new(EventSettings::new());
        events.max_fps(144);

        //Main rendering loop
        while let Some(e) = events.next(&mut self.window) {

            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            if let Some(args) = e.update_args() {
                self.update(&args);
            }
        }
    }


    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // We use double buffers, so we don't have to clone a buffer each time, so we improve our performance by using
        // already existing allocated buffers and clearing them.
        {
            let mut update_actors_lock = self.update_actors.lock().unwrap();
            std::mem::swap(&mut *update_actors_lock, &mut self.render_actors);
            update_actors_lock.clear(); // Clear the update_actors and free the lock, hence the scope
        }

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);

            //Draw actors loop
            for actor in &self.render_actors {
                match actor.object_type {
                    ObjectType::Circle => {
                        ellipse(actor.colour, [actor.x - actor.width / 2.0, actor.y - actor.height / 2.0, actor.width, actor.height], c.transform, gl);
                    },
                    ObjectType::Box => {
                        let rectangle = Rectangle::new_round_border(actor.colour, 1.0, 2.0);
                        rectangle.draw([actor.x - actor.width / 2.0, actor.y, actor.width, actor.height], &c.draw_state, c.transform, gl);
                    },
                    ObjectType::Text => {
                        let transform = c.transform.trans(actor.x - actor.width / 2.0, actor.y - actor.height / 2.0);
                        Text::new_color(actor.colour, actor.font_size)
                            .draw(&actor.name, &mut self.glyphs, &c.draw_state, transform, gl)
                            .expect("Failed to draw text");
                    },
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        //No state changes made here atm
    }
}
