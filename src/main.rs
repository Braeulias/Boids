use macroquad::prelude::*;

const BOIDS_COUNT: usize = 100;
const VIEW_RADIUS: f32 = 80.0;
const AVOID_RADIUS: f32 = 60.0;
const MAX_SPEED: f32 = 6.0;
const MAX_FORCE: f32 = 0.7;

const MAX_COHESION_FORCE: f32 = 0.3;

const MAX_SEPARATION_FORCE: f32 = 0.7; // Define a max separation force constant
const MAX_ALIGNMENT_FORCE: f32 = 0.5; // Define a max alignment force constant
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
        self.apply_separation(birds);
        self.apply_alignment(birds);
        self.apply_cohesion(birds);

        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length(MAX_SPEED / 2.0, MAX_SPEED);
        self.position += self.velocity;
        self.acceleration = Vec2::new(0.0, 0.0);

        // Wrap-around screen
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

    fn calculate_cohesion(&self, neighbors: &[&Bird]) -> Vec2 {
        let mut average_position = Vec2::new(0.0, 0.0);
        let count = neighbors.len() as f32;

        if count > 0.0 {
            for neighbor in neighbors {
                average_position += neighbor.position;
            }
            average_position /= count;
            let cohesion_force = (average_position - self.position).normalize() * MAX_SPEED;

            // Optionally reduce the force applied
            cohesion_force * 0.5
        } else {
            Vec2::new(0.0, 0.0)
        }
    }


    fn apply_cohesion(&mut self, birds: &[Bird]) {
        let neighbors = self.find_neighbors(birds);
        let cohesion_force = self.calculate_cohesion(&neighbors) - self.velocity;
        let clamped = cohesion_force.clamp_length_max(MAX_COHESION_FORCE); // Use clamped max cohesion force

        self.apply_force(clamped);
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

        let clamped = alignment_force.clamp_length_max(MAX_ALIGNMENT_FORCE); // Use clamped max alignment force

        self.apply_force(clamped);
    }

    fn calculate_separation(&self, neighbors: &[&Bird]) -> Vec2 {
        let mut separation_force = Vec2::new(0.0, 0.0);

        for neighbor in neighbors {
            let distance = self.position.distance(neighbor.position);
            if distance < AVOID_RADIUS && distance > 0.0 {
                let diff = self.position - neighbor.position;
                separation_force += diff.normalize() / distance;
            }
        }
        separation_force * 1.5 // Increase the effect of separation
    }
    fn apply_separation(&mut self, birds: &[Bird]) {
        let neighbors = self.find_neighbors(birds);
        let separation_force = self.calculate_separation(&neighbors);

        let clamped = separation_force.clamp_length_max(MAX_SEPARATION_FORCE); // Use clamped max separation force

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