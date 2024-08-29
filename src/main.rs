use macroquad::prelude::*;

const BOIDS_COUNT: usize = 100;
const VIEW_RADIUS: f32 = 50.0;
const AVOID_RADIUS: f32 = 20.0;
const MAX_SPEED: f32 = 4.0;
const MAX_FORCE: f32 = 0.1;


struct Bird {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
}

impl Bird {
    fn new() -> Bird {
        Bird {
            position: Vec2::new(
                rand::gen_range(0.0, screen_width()),
                rand::gen_range(0.0, screen_height())
            ),
            velocity: Vec2::new(
                rand::gen_range(-2.0, 2.0),
                rand::gen_range(-2.0, 2.0)
            ),
            acceleration: Vec2::new(0.0, 0.0),
        }
    }

    fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length_max(MAX_SPEED);
        self.position += self.velocity;     //updating position
        self.acceleration = Vec2::new(0.0, 0.0);

        if self.position.x > screen_width() {
            self.position.x = 0.0;
        } else if self.position.x < 0.0 {
            self.position.x = screen_width();
        }

        if self.position.y > screen_height() {
            self.position.y = 0.0;
        } else if self.position.y < 0.0 {
            self.position.y = screen_height();
        }

    }

    fn draw(&self) {
        let direction = self.velocity.normalize();
        let perp_direction = Vec2::new(-direction.y, direction.x);

        let p1 = self.position + direction * 10.0;
        let p2 = self.position - direction * 7.0 + perp_direction * 5.0;
        let p3 = self.position - direction * 7.0 - perp_direction * 5.0;

        draw_triangle(p1, p2, p3, WHITE);

    }


}
#[macroquad::main("Boids")]
async fn main() {
    let mut birds: Vec<Bird> = (0..BOIDS_COUNT).map(|_| Bird::new()).collect();

    loop {

        clear_background(BLACK);

        for bird in birds.iter_mut() {
            let random_acceleration = Vec2::new(rand::gen_range(-0.1,0.1), rand::gen_range(-0.1,0.1));
            bird.apply_force(random_acceleration);

            bird.update();
            bird.draw();

        }



        next_frame().await;
    }



}