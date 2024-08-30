
# Boids 🦅

Boids is an implementation of Craig Reynolds' famous [Boids algorithm](https://www.red3d.com/cwr/boids/), which simulates the flocking behavior of birds. This project is built in Rust using the [macroquad](https://github.com/not-fl3/macroquad) game framework. It offers a visual demonstration of how simple rules can create complex and lifelike group behavior.

## 🖼️ Demo Video

[![Boids Demo](https://img.youtube.com/vi/your-video-id-here/0.jpg)](https://www.youtube.com/watch?v=your-video-id-here)

## 🚀 Features

- **Cohesion**: Birds steer towards the average position of their neighbors.
- **Separation**: Birds avoid crowding neighbors (avoiding collisions).
- **Alignment**: Birds align their direction with their neighbors.
- **Obstacle Avoidance**: Birds detect and avoid obstacles in their path.
- **Mouse Attraction**: Birds are attracted to the mouse pointer, simulating a leader.
- **Dynamic Controls**: Toggle between different flocking behaviors and manage obstacles in real time.

## 📂 Installation

To run this project locally, you'll need Rust installed. You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

1. Clone the repository:

    ```bash
    git clone https://github.com/your-username/boids.git
    cd boids
    ```

2. Run the project:

    ```bash
    cargo run
    ```

## 🎮 Controls

- **Left Mouse Button**: Attract birds towards the mouse.
- **Right Mouse Button**: Add or remove obstacles (depending on delete mode).
- **D**: Toggle delete mode for obstacles.
- **C**: Clear all obstacles.
- **R**: Reset birds and obstacles.
- **F / ESC**: Toggle fullscreen.
- **1**: Toggle cohesion behavior.
- **2**: Toggle separation behavior.
- **3**: Toggle alignment behavior.

## ⚙️ Configuration

You can customize various parameters in the source code to tweak the behavior of the Boids:

- `BOIDS_COUNT`: Number of boids in the simulation.
- `VIEW_RADIUS`: How far a boid can see others.
- `AVOID_RADIUS`: Minimum distance to avoid collisions.
- `MAX_SPEED`: Maximum speed a boid can achieve.
- `MAX_FORCE`: Maximum steering force applied to a boid.
- `MOUSE_ATTRACTION_RADIUS`: Distance within which the mouse affects boids.

## 📜 License


## 🛠️ Built With

- [Rust](https://www.rust-lang.org/)
- [macroquad](https://github.com/not-fl3/macroquad)

## 🙏 Acknowledgements

- **Craig Reynolds** - For the original [Boids algorithm](https://www.red3d.com/cwr/boids/).
- **macroquad** - For the easy-to-use game framework.

---

