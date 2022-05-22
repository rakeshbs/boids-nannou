use nannou::{color::WHITE, prelude::Vec2};

const MAX_CAPACITY_QUADTREE: usize = 6;
const MIN_SQUARE_SIZE: usize = 5;
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }

    pub fn intersects(&self, rect: &Rectangle) -> bool {
        !(rect.x > self.x + self.width
            || rect.x + rect.width < self.x
            || rect.y > self.y + self.height
            || rect.y + rect.height < self.y)
    }
    pub fn point_inside_rect(&self, point: Vec2) -> bool {
        self.x <= point.x
            && self.y <= point.y
            && self.x + self.width > point.x
            && self.y + self.height > point.y
    }
}

pub trait HasLocation {
    fn get_location(&self) -> Vec2 {
        return Vec2::new(0.0, 0.0);
    }
}

pub struct QuadTree<'a, T: 'a + HasLocation> {
    boundary: Rectangle,
    elements: Vec<&'a T>,
    is_divided: bool,
    top_right: Option<Box<QuadTree<'a, T>>>,
    top_left: Option<Box<QuadTree<'a, T>>>,
    bottom_right: Option<Box<QuadTree<'a, T>>>,
    bottom_left: Option<Box<QuadTree<'a, T>>>,
}

impl<'a, T: HasLocation> QuadTree<'a, T> {
    pub fn new(boundary: Rectangle) -> Self {
        QuadTree {
            boundary,
            elements: Vec::new(),
            is_divided: false,
            top_left: None,
            top_right: None,
            bottom_right: None,
            bottom_left: None,
        }
    }

    fn split(&mut self) {
        let x = self.boundary.x;
        let y = self.boundary.y;
        let h_w = self.boundary.width / 2.0;
        let h_h = self.boundary.height / 2.0;
        let tl = Rectangle::new(x, y, h_w, h_h);
        let tr = Rectangle::new(x + h_w, y, h_w, h_h);
        let bl = Rectangle::new(x, y + h_h, h_w, h_h);
        let br = Rectangle::new(x + h_w, y + h_h, h_w, h_h);

        self.top_left = Some(Box::new(QuadTree::new(tl)));
        self.top_right = Some(Box::new(QuadTree::new(tr)));
        self.bottom_left = Some(Box::new(QuadTree::new(bl)));
        self.bottom_right = Some(Box::new(QuadTree::new(br)));
    }

    fn insert_into_node(node: &mut Option<Box<QuadTree<'a, T>>>, element: &'a T) {
        if let Some(n) = node {
            n.insert(element);
        }
    }

    fn draw_node(node: &Option<Box<QuadTree<'a, T>>>, draw: &nannou::prelude::Draw) {
        if let Some(n) = node {
            n.draw(draw)
        }
    }

    fn query_node(
        node: &Option<Box<QuadTree<'a, T>>>,
        rect: &Rectangle,
        mut found: Vec<&'a T>,
    ) -> Vec<&'a T> {
        if let Some(n) = node {
            found = n.query(rect, found);
        }
        found
    }

    pub fn query(&self, rect: &Rectangle, mut found: Vec<&'a T>) -> Vec<&'a T> {
        if !self.boundary.intersects(&rect) {}
        for element in &self.elements {
            let point = element.get_location();
            if rect.point_inside_rect(point) {
                found.push(element);
                if self.is_divided {
                    found = QuadTree::query_node(&self.top_left, rect, found);
                    found = QuadTree::query_node(&self.top_right, rect, found);
                    found = QuadTree::query_node(&self.bottom_right, rect, found);
                    found = QuadTree::query_node(&self.bottom_left, rect, found);
                }
            }
        }
        found
    }

    pub fn draw(&self, draw: &nannou::prelude::Draw) {
        draw.rect()
            .x_y(self.boundary.x, self.boundary.y)
            .width(self.boundary.width)
            .height(self.boundary.height)
            .no_fill()
            .stroke_weight(1.0)
            .stroke_color(WHITE);
        if self.is_divided {
            QuadTree::draw_node(&self.top_right, draw);
            QuadTree::draw_node(&self.top_left, draw);
            QuadTree::draw_node(&self.bottom_right, draw);
            QuadTree::draw_node(&self.bottom_left, draw);
        }
    }

    pub fn insert(&mut self, element: &'a T) {
        let point = element.get_location();
        if self.boundary.point_inside_rect(point) {
            if self.elements.len() < MAX_CAPACITY_QUADTREE {
                self.elements.push(element);
            } else {
                if !self.is_divided {
                    self.split();
                    self.is_divided = true;
                }
                QuadTree::insert_into_node(&mut self.top_left, element);
                QuadTree::insert_into_node(&mut self.top_right, element);
                QuadTree::insert_into_node(&mut self.bottom_left, element);
                QuadTree::insert_into_node(&mut self.bottom_right, element);
            }
        }
    }
}
