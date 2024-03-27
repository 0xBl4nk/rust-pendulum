use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main() {
    // Create window
    let window = Window::new_centered("Pendulum", (800, 480)).unwrap();

    let win = MyWindowHandler {
        p: Pendulum::new(400.0, 0.0, 200.0),
        p2: Pendulum::new(400.0, 0.0, 400.0),
    };

    // Run the loop
    window.run_loop(win);
}

struct MyWindowHandler {
    p: Pendulum,
    p2: Pendulum,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        // Clear screen
        graphics.clear_screen(Color::from_rgb(0.0, 0.0, 0.0));
        
        self.p.update();
        self.p.draw(graphics);

        self.p2.update();
        self.p2.draw(graphics);

        // Draw the frame
        helper.request_redraw();
    }
}

struct Pendulum {
    // Position of the pendulum
    origin:vector::Vector,
    
    // Position of the ball
    position:vector::Vector,

    // This is the angle of the pendulum.
    angle:f32,

    angular_velocity:f32,
    angular_acceleration:f32,

    r:f32, // The Lenght of the pendulum.
    m:f32, // The mass of the ball.
    g:f32, // The gravity
}

impl Pendulum {
    fn new(x:f32, y:f32, r:f32) -> Pendulum {
        Pendulum {
            origin: vector::Vector::new(x, y),
            position: vector::Vector::new(0.0, 0.0),
            angle: 1.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            r: r,
            m: 1.0,
            g: 0.5,
        }
    }

    fn update(&mut self) {
        // Pendulum equation to calcule the angular acceleration
        self.angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;

        // Angular velocity
        self.angular_velocity += self.angular_acceleration;

        // Angle
        self.angle += self.angular_velocity;

        // The position is the polar coordinates translated to cartesian coordinates.
        self.position
            .set(self.r * self.angle.sin(), self.r * self.angle.cos());

        // The final position of the ball
        self.position.add(&self.origin);
    }

    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::RED,
        );
        
        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::RED)
            
    }
}

pub mod vector {

    pub struct Vector {
        pub x:f32,
        pub y:f32,
    }

    impl Vector {
        pub fn new(x:f32, y:f32) -> Vector {
            Vector { x, y } // return
        }

        pub fn add(&mut self, other:&Vector) -> &Vector {
            self.x += other.x;
            self.y += other.y;
            self // return
        }

        pub fn set(&mut self, x:f32, y:f32) -> &Vector {
            self.x =x;
            self.y = y;
            self // return
        }
    }
}
