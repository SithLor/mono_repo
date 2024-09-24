mod p {

    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    pub trait Distance {
        fn distance(&self, other: &Self) -> f64;
    }
    pub trait Getter {
        fn get_x(&self) -> f64;
        fn get_y(&self) -> f64;
    }
    pub trait Scale {
        fn scale_x_by(&mut self, factor: f64);
        fn scale_y_by(&mut self, factor: f64);
    }
    pub trait Setter {
        fn set_x(&mut self, x: f64);
        fn set_y(&mut self, y: f64);
    }
    impl Setter for Point {
        fn set_x(&mut self, x: f64) {
            self.x = x;
        }
        fn set_y(&mut self, y: f64) {
            self.y = y;
        }
    }
    impl Distance for Point {
        fn distance(&self, other: &Self) -> f64 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            (dx * dx + dy * dy).sqrt()
        }
    }
    impl Getter for Point {
        fn get_x(&self) -> f64 {
            self.x
        }
        fn get_y(&self) -> f64 {
            self.y
        }
    }
    impl Scale for Point {
        fn scale_x_by(&mut self, factor: f64) {
            self.x *= factor;
        }
        fn scale_y_by(&mut self, factor: f64) {
            self.y *= factor;
        }
    }
}

fn main() {
    use crate::p::Point;
    use crate::p::Distance;
    use crate::p::Getter;
    use crate::p::Scale;
    use crate::p::Setter;

    let mut p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 4.0, y: 6.0 };
}
