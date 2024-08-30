use macroquad::prelude::*;

const BOIDS_COUNT: usize = 150; //amount of Boids, change if you want
const VIEW_RADIUS: f32 = 80.0;  //Boid ViewRadius
const AVOID_RADIUS: f32 = 60.0; //Boid avoid radius
const MAX_SPEED: f32 = 6.0; //Max boid speed
const MAX_FORCE: f32 = 0.7; // Max force a boid can be pulled by
const MOUSE_ATTRACTION_RADIUS: f32 = 150.0; //Name says it

const MAX_COHESION_FORCE: f32 = 0.2;    //Force that groups them together

const MAX_SEPARATION_FORCE: f32 = 0.7; // Force that keeps them seperate
const MAX_ALIGNMENT_FORCE: f32 = 0.3; // Force that aligns them
const VIEW_ANGLE: f32 = std::f32::consts::PI * 3.0 / 2.0; //(270 degrees)


//Obstacles Start
struct Obstacle {
    position: Vec2,
    radius: f32,
}

impl Obstacle {
    fn new(position: Vec2, radius: f32) -> Obstacle{
        Obstacle { position, radius }
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, RED);
    }

    fn contains(&self, point: Vec2) -> bool {   //checks if somethings in the radius of the object
        self.position.distance(point) < self.radius
    }
}
//Obstacles End

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

    fn apply_force(&mut self, force: Vec2) { //name says it
        self.acceleration += force;
    }

    fn update(&mut self, birds: &[Bird], obstacles: &[Obstacle]) {  //keeps the birds updated :)
        self.apply_separation(birds);
        self.apply_alignment(birds);
        self.apply_cohesion(birds);

        self.apply_obstacle_avoidance(obstacles);
        self.apply_random_force();

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

    fn apply_random_force(&mut self) {  //makes sure that everything has its randomness
        let random_force = Vec2::new(
            rand::gen_range(-0.1, 0.1),
            rand::gen_range(-0.1, 0.1)
        );
        self.apply_force(random_force);
    }


    fn draw(&self) {    //just read the fn name
        let direction = self.velocity.normalize_or_zero();
        let perp_direction = Vec2::new(-direction.y, direction.x);

        let p1 = self.position + direction * 10.0;
        let p2 = self.position - direction * 7.0 + perp_direction * 5.0;
        let p3 = self.position - direction * 7.0 - perp_direction * 5.0;

        draw_triangle(p1, p2, p3, WHITE);

    }

    fn calculate_obstacle_avoidance(&self, obstacles: &[Obstacle]) -> Vec2 {    //checks that birds dont hit obstacles
        let mut avoidance_force = Vec2::new(0.0, 0.0);
        let mut total_weight = 0.0;

        for obstacle in obstacles {
            let distance = self.position.distance(obstacle.position);

            if distance < (obstacle.radius + 20.0) { // Adjust this threshold as needed
                let diff = self.position - obstacle.position;
                let weight = 1.0 / distance; // Increase weight with closer obstacles
                avoidance_force += diff.normalize() * weight;
                total_weight += weight;
            }
        }

        if total_weight > 0.0 {
            avoidance_force /= total_weight;
        }
        avoidance_force * MAX_SEPARATION_FORCE // Scale the force
    }

    fn apply_obstacle_avoidance(&mut self, obstacles: &[Obstacle]) {    //applying it
        let avoidance_force = self.calculate_obstacle_avoidance(obstacles);
        self.apply_force(avoidance_force);
    }

    fn apply_mouse_attraction(&mut self, mouse_pos: Vec2, attraction_strength: f32) {   //name says it again
        let direction = (mouse_pos - self.position).normalize();
        let force = direction * attraction_strength;
        self.apply_force(force);
    }

    fn find_neighbors<'a>(&self, birds: &'a [Bird]) -> Vec<&'a Bird> {  //checks for other boids within the View Angle and Radius of the boid
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

    fn calculate_cohesion(&self, neighbors: &[&Bird]) -> Vec2 {     //groups them together
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


    fn apply_cohesion(&mut self, birds: &[Bird]) { //read
        let neighbors = self.find_neighbors(birds);
        let cohesion_force = self.calculate_cohesion(&neighbors) - self.velocity;
        let clamped = cohesion_force.clamp_length_max(MAX_COHESION_FORCE);

        self.apply_force(clamped);
    }


    fn calculate_alignment(&self,neighbors: &[&Bird]) -> Vec2 {     //makes sure they fly in the same direction
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
    fn apply_alignment(&mut self, birds: &[Bird]) { //yeah
        let neighbors = self.find_neighbors(birds);
        let alignment_force = self.calculate_alignment(&neighbors);

        let clamped = alignment_force.clamp_length_max(MAX_ALIGNMENT_FORCE);

        self.apply_force(clamped);
    }

    fn calculate_separation(&self, neighbors: &[&Bird]) -> Vec2 {   //makes sure they dont touch each other
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
    fn apply_separation(&mut self, birds: &[Bird]) {    //read it
        let neighbors = self.find_neighbors(birds);
        let separation_force = self.calculate_separation(&neighbors);

        let clamped = separation_force.clamp_length_max(MAX_SEPARATION_FORCE);

        self.apply_force(clamped);
    }


}


fn create_birds() -> Vec<Bird> {    //self explanatory
    (0..BOIDS_COUNT).map(|_| Bird::new()).collect()
}

fn draw_key_bindings() {    //Read the Name
    let key_bindings = [
        ("D", "Toggle Obstacle Delete Mode"),
        ("C", "Clear Obstacles"),
        ("R", "Reset"),
        ("F", "Toggle Fullscreen"),
    ];

    let text_color = LIGHTGRAY;
    let mut y_position = 10.0;

    for (key, description) in key_bindings.iter() {
        draw_text(
            &format!("{}: {}", key, description),
            10.0,
            y_position,
            20.0,
            text_color,
        );
        y_position += 30.0; // Adjust spacing as needed
    }
}


fn window_conf() -> Conf {      //Window Configuration
    Conf {
        window_title: "Boids".to_owned(),
        fullscreen: true, // Enable fullscreen
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let mut birds = create_birds();     //track boids
    let mut fullscreen = true; // Track fullscreen state

    let mut obstacles: Vec<Obstacle> = Vec::new();  //track obstacles
    let obstacle_radius = 20.0; // change if you want

    let mut delete_mode = false;    //tracks deletemode state

    let mouse_attraction_strength = 0.4;    //strength of mouse attraction

    loop {

        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Right) {    //place/delete obstacles
            let mouse_pos = mouse_position();
            if is_mouse_button_pressed(MouseButton::Right) {
                if delete_mode {
                    // Remove obstacle if in delete mode
                    obstacles.retain(|obstacle| !obstacle.contains(Vec2::new(mouse_pos.0, mouse_pos.1)));
                } else {
                    // Add new obstacle if not in delete mode
                    obstacles.push(Obstacle::new(Vec2::new(mouse_pos.0, mouse_pos.1), obstacle_radius));
                }
            }

        }

        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);

        if is_mouse_button_down(MouseButton::Left) {    //mouse attraction
            for bird in birds.iter_mut() {
                let distance = bird.position.distance(mouse_pos);
                if distance < MOUSE_ATTRACTION_RADIUS {
                    bird.apply_mouse_attraction(mouse_pos, mouse_attraction_strength);
                }
            }
        }

        if is_key_pressed(KeyCode::D) {
            delete_mode = !delete_mode; // Toggle delete mode
        }

        if is_key_pressed(KeyCode::C) { //clear obstacles
            obstacles.clear();
        }

        if is_key_pressed(KeyCode::R) {     //Reset everything
            birds = create_birds(); // Reset birds
            obstacles.clear(); // Clear obstacles
        }

        if is_key_pressed(KeyCode::F) {
            fullscreen = !fullscreen; // Toggle fullscreen state
            set_fullscreen(fullscreen);
        }


        let birds_copy = birds.clone();
        for bird in birds.iter_mut() {

            bird.update(&birds_copy, &obstacles);
        }
        for bird in &birds {
            bird.draw();
        }

        for obstacle in &obstacles {
            obstacle.draw();
        }

        draw_key_bindings();

        next_frame().await;
    }



}

