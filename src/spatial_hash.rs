use nannou::prelude::*;

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

pub struct SpatialHashGrid {
    cell_size: f32,
    inv_cell_size: f32,
    bounds: Rectangle,
    cells: Vec<Vec<usize>>,
    grid_width: usize,
    grid_height: usize,
}

impl SpatialHashGrid {
    pub fn new(bounds: Rectangle, cell_size: f32) -> Self {
        let grid_width = (bounds.width / cell_size).ceil().max(1.0) as usize;
        let grid_height = (bounds.height / cell_size).ceil().max(1.0) as usize;
        let num_cells = grid_width * grid_height;
        let mut cells = Vec::with_capacity(num_cells);
        for _ in 0..num_cells {
            cells.push(Vec::new());
        }
        SpatialHashGrid {
            cell_size,
            inv_cell_size: 1.0 / cell_size,
            bounds,
            cells,
            grid_width,
            grid_height,
        }
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.clear();
        }
    }

    fn cell_coords(&self, x: f32, y: f32) -> (isize, isize) {
        let cx = ((x - self.bounds.x) * self.inv_cell_size).floor() as isize;
        let cy = ((y - self.bounds.y) * self.inv_cell_size).floor() as isize;
        (cx, cy)
    }

    fn cell_index(&self, cx: isize, cy: isize) -> Option<usize> {
        if cx < 0 || cy < 0 || cx >= self.grid_width as isize || cy >= self.grid_height as isize {
            return None;
        }
        Some((cy as usize) * self.grid_width + (cx as usize))
    }

    pub fn insert(&mut self, position: Vec2, index: usize) {
        let (cx, cy) = self.cell_coords(position.x, position.y);
        if let Some(idx) = self.cell_index(cx, cy) {
            self.cells[idx].push(index);
        }
    }

    pub fn query<F>(&self, rect: Rectangle, mut callback: F)
    where
        F: FnMut(usize),
    {
        let min_cx =
            ((rect.x - self.bounds.x) * self.inv_cell_size).floor() as isize;
        let min_cy =
            ((rect.y - self.bounds.y) * self.inv_cell_size).floor() as isize;
        let max_cx = ((rect.x + rect.width - self.bounds.x) * self.inv_cell_size).floor() as isize;
        let max_cy = ((rect.y + rect.height - self.bounds.y) * self.inv_cell_size).floor() as isize;

        for cy in min_cy..=max_cy {
            for cx in min_cx..=max_cx {
                if let Some(idx) = self.cell_index(cx, cy) {
                    for &index in &self.cells[idx] {
                        callback(index);
                    }
                }
            }
        }
    }
}
