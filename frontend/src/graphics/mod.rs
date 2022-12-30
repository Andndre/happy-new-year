mod colour;
mod fireworks;
mod sim;

use core::panic;

use js_sys::Math;
use rand::seq::SliceRandom;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use fireworks::{ColourShiftFirework, Firework, StandardFirework};
use sim::{Particle, TwoVec};

use crate::LAUNCH_SOUNDS;

const STAR_RADIUS: f64 = 2.;
const STAR_COUNT: u32 = 20;

pub struct Graphics {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    stars: Vec<Particle>,
    fireworks: Vec<Box<dyn Firework>>,
}

impl Graphics {
    /* Create a new firework simulation on the given canvas. */
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let stars = Vec::new();
        let fireworks = Vec::new();

        context.set_fill_style(&JsValue::from_str("yellow"));
        context.set_text_baseline("middle");
        context.set_text_align("center");

        Self {
            canvas,
            context,
            stars,
            fireworks,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let (old_width, old_height) = (self.canvas.width(), self.canvas.height());
        let width_ratio = width as f64 / old_width as f64;
        let height_ratio = height as f64 / old_height as f64;

        for star in &mut self.stars {
            let old_pos = star.pos();

            let (new_x, new_y) = (old_pos.x() * width_ratio, old_pos.y() * height_ratio);

            star.set_pos(TwoVec::new(new_x, new_y));
        }

        self.canvas.set_width(width);
        self.canvas.set_height(height);
    }

    /* Create the stars. */
    pub unsafe fn init(&mut self) {
        // Generate the stars
        self.create_stars(STAR_COUNT, self.canvas.width(), self.canvas.height());
    }

    /* Spawn a firework, depending on what types of firework there are. */
    pub unsafe fn spawn_firework(&mut self, name: String) {
        let sound = LAUNCH_SOUNDS.choose(&mut rand::thread_rng()).unwrap();
        sound.set_current_time(0f64);
        if let Err(_res) = sound.play() {
            panic!("Cannot play sound");
        }
        match self.fireworks.len() % 2 {
            0 => {
                self.fireworks.push(Box::from(StandardFirework::new(
                    name,
                    self.canvas.width(),
                    self.canvas.height(),
                )));
            }
            1 => {
                self.fireworks.push(Box::from(ColourShiftFirework::new(
                    name,
                    self.canvas.width(),
                    self.canvas.height(),
                )));
            }
            _ => {
                panic!("This shouldn't happen ever.");
            }
        }
    }

    /* Draw the firework and stars. */
    pub fn draw(&self) {
        /* Clear the canvas. */
        self.context.clear_rect(
            0.,
            0.,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );

        /* Draw the stars. */
        self.draw_stars();

        /* Draw the fireworks. */
        for firework in &self.fireworks {
            firework.draw(&self.context);
        }
    }

    /* Simulate the fireworks. */
    pub unsafe fn step(&mut self) {
        for firework in &mut self.fireworks {
            firework.step(self.canvas.width(), self.canvas.height());
        }
    }

    /* Create stars at random positions on the canvas. */
    unsafe fn create_stars(&mut self, count: u32, canvas_width: u32, canvas_height: u32) {
        for _ in 0..count {
            let pos = Particle::new(
                TwoVec::new(
                    Math::random() * canvas_width as f64,
                    Math::random() * canvas_height as f64,
                ),
                TwoVec::zero(),
            );

            self.stars.push(pos);
        }
    }

    /* This function draws the stars on the canvas. */
    fn draw_stars(&self) {
        for star in &self.stars {
            star.draw(
                (&"").to_string(),
                &self.context,
                colour::YELLOW,
                STAR_RADIUS,
            );
        }
    }
}
