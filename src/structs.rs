
fn main () {
    let circle = Circle {
        x: 10.0,
        y: 10.0,
        radius: 10.0
    };

    println!("x: {}, y: {}, radius: {}", circle.x, circle.y, circle.radius);
    println!("Circle radius:  {}", circle.get_radius());
    println!("Circle Area: {}", circle.area());

    let rect = Rectangle {
        width: 10.0,
        height: 5.0
    };

    println!("Circle Area: {}", rect.area())
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64
}

impl Circle {
    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}