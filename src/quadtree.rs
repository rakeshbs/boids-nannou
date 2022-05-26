use nannou::{draw::mesh::vertex::Color, prelude::Vec2};

const MAX_CAPACITY_QUADTREE: usize = 4;
#[derive(Debug, Copy, Clone)]
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
    fn get_location(&self) -> Vec2;
}

pub struct QuadTree<'a, T>
where
    T: HasLocation,
{
    boundary: Rectangle,
    points: Vec<Vec2>,
    objects: Vec<&'a T>,
    is_divided: bool,
    top_right: Option<Box<QuadTree<'a, T>>>,
    top_left: Option<Box<QuadTree<'a, T>>>,
    bottom_right: Option<Box<QuadTree<'a, T>>>,
    bottom_left: Option<Box<QuadTree<'a, T>>>,
}

impl<'a, T> QuadTree<'a, T>
where
    T: HasLocation,
{
    pub fn new(boundary: Rectangle) -> Self {
        QuadTree {
            boundary,
            points: Vec::new(),
            objects: Vec::new(),
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

    fn query_node(
        node: &Option<Box<QuadTree<'a, T>>>,
        rect: Rectangle,
        mut found: Vec<&'a T>,
    ) -> Vec<&'a T> {
        if let Some(n) = node {
            found = n.query(rect, found);
        }
        found
    }

    pub fn query(&self, rect: Rectangle, mut found: Vec<&'a T>) -> Vec<&'a T> {
        if self.boundary.intersects(&rect) {
            self.objects.iter().for_each(|object| {
                if rect.point_inside_rect(object.get_location()) {
                    found.push(&object);
                }
            });
            if self.is_divided {
                found = QuadTree::query_node(&self.top_left, rect, found);
                found = QuadTree::query_node(&self.top_right, rect, found);
                found = QuadTree::query_node(&self.bottom_left, rect, found);
                found = QuadTree::query_node(&self.bottom_right, rect, found);
            }
        }
        found
    }

    fn draw_node(node: &Option<Box<QuadTree<T>>>, draw: &nannou::prelude::Draw) {
        if let Some(n) = node {
            n.draw(draw);
        }
    }

    pub fn draw(&self, draw: &nannou::prelude::Draw) {
        let w = self.boundary.width;
        let h = self.boundary.height;
        draw.rect()
            .x_y(self.boundary.x + w / 2.0, self.boundary.y + h / 2.0)
            .width(self.boundary.width)
            .height(self.boundary.height)
            .no_fill()
            .stroke_weight(1.0)
            .stroke_color(Color::new(1.0, 1.0, 1.0, 0.3));
        if self.is_divided {
            QuadTree::draw_node(&self.top_right, draw);
            QuadTree::draw_node(&self.top_left, draw);
            QuadTree::draw_node(&self.bottom_right, draw);
            QuadTree::draw_node(&self.bottom_left, draw);
        }
    }

    fn insert_into_node(node: &mut Option<Box<QuadTree<'a, T>>>, object: &'a T) {
        if let Some(n) = node {
            n.insert(object);
        }
    }

    pub fn insert(&mut self, object: &'a T) {
        if self.boundary.point_inside_rect(object.get_location()) {
            if self.objects.len() < MAX_CAPACITY_QUADTREE {
                self.objects.push(object);
            } else {
                if !self.is_divided {
                    self.split();
                    self.is_divided = true;
                }
                QuadTree::insert_into_node(&mut self.top_left, object);
                QuadTree::insert_into_node(&mut self.top_right, object);
                QuadTree::insert_into_node(&mut self.bottom_left, object);
                QuadTree::insert_into_node(&mut self.bottom_right, object);
            }
        }
    }
}
