use glam::Vec2;

pub struct Boundary {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Boundary {
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.x - self.width / 2.0
            && point.x < self.x + self.width / 2.0
            && point.y >= self.y - self.height / 2.0
            && point.y < self.y + self.height / 2.0
    }

    pub fn intersects(&self, other: &Boundary) -> bool {
        !(other.x - other.width / 2.0 > self.x + self.width / 2.0
            || other.x + other.width / 2.0 < self.x - self.width / 2.0
            || other.y - other.height / 2.0 > self.y + self.height / 2.0
            || other.y + other.height / 2.0 < self.y - self.height / 2.0)
    }
}

pub struct QuadTree {
    pub boundary: Boundary,
    pub capacity: usize,
    pub points: Vec<(Vec2, usize)>,
    pub divided: bool,
    pub northwest: Option<Box<QuadTree>>,
    pub northeast: Option<Box<QuadTree>>,
    pub southwest: Option<Box<QuadTree>>,
    pub southeast: Option<Box<QuadTree>>,
}

impl QuadTree {
    pub fn new(boundary: Boundary, capacity: usize) -> Self {
        Self {
            boundary,
            capacity,
            points: Vec::new(),
            divided: false,
            northwest: None,
            northeast: None,
            southwest: None,
            southeast: None,
        }
    }

    pub fn subdivide(&mut self) {
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.width / 2.0;
        let h = self.boundary.height / 2.0;

        let nw = Boundary {
            x: x - w / 2.0,
            y: y - h / 2.0,
            width: w,
            height: h,
        };
        self.northwest = Some(Box::new(QuadTree::new(nw, self.capacity)));

        let ne = Boundary {
            x: x + w / 2.0,
            y: y - h / 2.0,
            width: w,
            height: h,
        };
        self.northeast = Some(Box::new(QuadTree::new(ne, self.capacity)));

        let sw = Boundary {
            x: x - w / 2.0,
            y: y + h / 2.0,
            width: w,
            height: h,
        };
        self.southwest = Some(Box::new(QuadTree::new(sw, self.capacity)));

        let se = Boundary {
            x: x + w / 2.0,
            y: y + h / 2.0,
            width: w,
            height: h,
        };
        self.southeast = Some(Box::new(QuadTree::new(se, self.capacity)));

        self.divided = true;
    }

    pub fn insert(&mut self, point: Vec2, index: usize) -> bool {
        if !self.boundary.contains(point) {
            return false;
        }

        if self.points.len() < self.capacity {
            self.points.push((point, index));
            return true;
        }

        if !self.divided {
            self.subdivide();
        }

        if self.northwest.as_mut().unwrap().insert(point, index)
            || self.northeast.as_mut().unwrap().insert(point, index)
            || self.southwest.as_mut().unwrap().insert(point, index)
            || self.southeast.as_mut().unwrap().insert(point, index)
        {
            return true;
        }

        false
    }

    pub fn query(&self, range: &Boundary, found: &mut Vec<(Vec2, usize)>) {
        if !self.boundary.intersects(range) {
            return;
        }

        for point in &self.points {
            if range.contains(point.0) {
                found.push(*point);
            }
        }

        if self.divided {
            self.northwest.as_ref().unwrap().query(range, found);
            self.northeast.as_ref().unwrap().query(range, found);
            self.southwest.as_ref().unwrap().query(range, found);
            self.southeast.as_ref().unwrap().query(range, found);
        }
    }
}
