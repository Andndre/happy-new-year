use colour::random_colour;
use js_sys::Math;
use rand::seq::SliceRandom;
use web_sys::CanvasRenderingContext2d;

use crate::EXPLOTION_SOUNDS;

use super::super::colour;
use super::super::colour::Colour;
use super::super::sim::{Particle, TwoVec};
use super::{Firework, Rocket, GRAVITY, PARTICLE_COUNT, PARTICLE_LIFETIME};

use super::vel_min_max;

/* This struct represents a plain firework with one colour.. */
pub struct StandardFirework {
    name: String,
    rocket: Particle,
    exploded: bool,
    particles: Vec<Particle>,
    colour: Colour,
    lifetime: u32,
}

/* Implement the standard rocket behaviour for this struct. */
impl Rocket for StandardFirework {
    fn rocket_mut(&mut self) -> &mut Particle {
        &mut self.rocket
    }

    fn rocket(&self) -> &Particle {
        &self.rocket
    }

    fn exploded(&self) -> bool {
        self.exploded
    }

    /* Explode the firework. */
    unsafe fn explode(&mut self) -> () {
        self.exploded = true;
        let sound = EXPLOTION_SOUNDS.choose(&mut rand::thread_rng()).unwrap();
        sound.set_current_time(0f64);
        if let Err(_res) = sound.play() {
            panic!("Cannot play sound");
        }
        let radius = 1.5 + Math::random() * 1.5;

        /* Create the explosion. */
        for _ in 0..PARTICLE_COUNT {
            let mut particle =
                Particle::random_at(self.rocket.pos().clone(), radius + Math::random() * 0.5);
            particle.set_vel(particle.vel() + self.rocket.vel());
            self.particles.push(particle);
        }
    }

    /* Simulate one step of the explosion. */
    unsafe fn sim_explosion(&mut self, width: u32, height: u32) -> () {
        self.particles.iter_mut().for_each(|particle| {
            particle.apply_force(GRAVITY);
            particle.step();
        });

        self.lifetime -= 1;

        if self.lifetime == 0 {
            self.reset(width, height);
        }
    }

    /* Draw the explosion. */
    fn draw_explosion(&self, context: &CanvasRenderingContext2d) -> () {
        for particle in &self.particles {
            particle.draw_rgba(
                &self.name,
                context,
                self.colour,
                (self.lifetime as f64) / (PARTICLE_LIFETIME as f64),
                2.4,
            );
        }
    }

    /* Reset the explosion. */
    unsafe fn reset_explosion(&mut self) -> () {
        self.exploded = false;
        self.particles.clear();
        self.colour = colour::random_colour();
        self.lifetime = PARTICLE_LIFETIME;
    }
}

impl StandardFirework {
    /* Create new firework at random position on the bottom, with random colour. */
    pub unsafe fn new(name: String, width: u32, height: u32) -> Self {
        let (vel_min, vel_max) = vel_min_max(height);

        Self {
            name,
            rocket: Particle::new(
                TwoVec::new(Math::random() * width as f64, height as f64),
                TwoVec::new(0., vel_min + (vel_max - vel_min) * Math::random()),
            ),
            exploded: false,
            particles: Vec::new(),
            colour: random_colour(),
            lifetime: PARTICLE_LIFETIME,
        }
    }
}
