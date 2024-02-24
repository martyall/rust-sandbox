#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Default for Point {
    fn default() -> Point {
        Point { x: 0, y: 0 }
    }
}

pub struct Polyline {
    points: Vec<Point>,
}

impl Polyline {
    pub fn new(p: Point) -> Polyline {
        let points = vec![p];
        Polyline { points }
    }

    pub fn insert(&mut self, p: Point) -> () {
        self.points.push(p);
    }

    pub fn get(self, p: Point) -> Option<Point> {
        for q in self.points {
            if q == p {
                return Some(q);
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(Point { x: 0, y: 0 }, Point::default())
    }

    #[test]
    fn test_new() {
        let p = Point { x: 1, y: 2 };
        let line = Polyline::new(p);
        assert!(line.get(p).is_some())
    }

    #[test]
    fn test_insert() {
        let p = Point { x: -1, y: -2 };
        let mut line = Polyline::new(Point::default());
        line.insert(p);
        assert!(line.get(p).is_some())
    }
}
