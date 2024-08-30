use macroquad::prelude::*;

const BOIDS_COUNT: usize = 100;
const VIEW_RADIUS: f32 = 50.0;
const AVOID_RADIUS: f32 = 35.0;
const MAX_SPEED: f32 = 4.0;
const MAX_FORCE: f32 = 0.65;

const VIEW_ANGLE: f32 = std::f32::consts::PI * 3.0 / 2.0; //(270 degrees)

#[derive(Clone)]
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

    fn update(&mut self, birds: &[Bird]) {
        self.apply_seperation(birds);
        self.apply_alignment(birds);

        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length(MAX_SPEED / 2.0 , MAX_SPEED);
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
        let direction = self.velocity.normalize_or_zero();
        let perp_direction = Vec2::new(-direction.y, direction.x);

        let p1 = self.position + direction * 10.0;
        let p2 = self.position - direction * 7.0 + perp_direction * 5.0;
        let p3 = self.position - direction * 7.0 - perp_direction * 5.0;

        draw_triangle(p1, p2, p3, WHITE);

    }

    fn find_neighbors<'a>(&self, birds: &'a [Bird]) -> Vec<&'a Bird> {
        let mut neighbors = Vec::new();
        let view_cos = (VIEW_ANGLE / 2.0).cos(); // Cosine of 135 degrees

        for bird in birds {
            let distance = self.position.distance(bird.position);

            // Check if within view radius
            if distance > 0.0 && distance < VIEW_RADIUS {
                let direction_to_neighbor = (bird.position - self.position).normalize();
                let dot_product = self.velocity.normalize().dot(direction_to_neighbor);

                // Check if within view angle
                if dot_product > view_cos {
                    neighbors.push(bird);
                }
            }
        }

        neighbors
    }

    fn calculate_alignment(&self,neighbors: &[&Bird]) -> Vec2 {
        let mut average_velocity = Vec2::new(0.0, 0.0);
        let count = neighbors.len() as f32;

        if count > 0.0 {
            for neighbor in neighbors {
                average_velocity += neighbor.velocity;
            }
            average_velocity /= count;
            average_velocity.normalize() * MAX_SPEED
        } else {
            Vec2::new(0.0, 0.0)
        }

    }
    fn apply_alignment(&mut self, birds: &[Bird]) {
        let neighbors = self.find_neighbors(birds);
        let alignment_force = self.calculate_alignment(&neighbors);
        let clamped = alignment_force.clamp_length_max(MAX_FORCE);

        self.apply_force(clamped);
    }

    fn calculate_seperation(&self, neighbors: &[&Bird]) -> Vec2 {
        let mut seperation_force = Vec2::new(0.0, 0.0);

        for neighbor in neighbors {
            let distance = self.position.distance(neighbor.position);
            if distance < AVOID_RADIUS && distance > 0.0 {
                let diff = self.position - neighbor.position;
                seperation_force += diff.normalize() / distance
            }
        }
        seperation_force
    }

    fn apply_seperation(&mut self, birds: &[Bird]) {
        let neighbors = self.find_neighbors(birds);
        let seperation_force = self.calculate_seperation(&neighbors);

        let clamped = seperation_force.clamp_length_max(MAX_FORCE);

        self.apply_force(clamped);
    }


}
#[macroquad::main("Boids")]
async fn main() {
    let mut birds: Vec<Bird> = (0..BOIDS_COUNT).map(|_| Bird::new()).collect();

    loop {

        clear_background(BLACK);

        let birds_copy = birds.clone();
        for bird in birds.iter_mut() {

            bird.update(&birds_copy);
        }
        for bird in &birds {
            bird.draw();
        }


        next_frame().await;
    }



}