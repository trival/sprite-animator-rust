#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Light,
    Medium,
    Dark,
}

pub struct Frame {
    pub width: u16,
    pub height: u16,
    cells: Vec<Color>,
}

pub struct Cell<'a> {
    pub x: u16,
    pub y: u16,
    pub val: Color,
    frame: &'a Frame,
}

impl Frame {
    pub fn new(width: u16, height: u16) -> Self {
        let cells = vec![Color::Dark; (width * height).into()];
        let frame = Frame {
            width,
            height,
            cells,
        };

        frame
    }

    pub fn get(&self, x: i32, y: i32) -> Color {
        let (x, y) = adjust_coords(x, y, self.width, self.height);

        self.cells[index(y, x, self.height)]
    }

    pub fn set(&mut self, x: i32, y: i32, val: Color) {
        let (x, y) = adjust_coords(x, y, self.width, self.height);

        self.cells[index(y, x, self.height)] = val
    }

    pub fn cell(&self, x: i32, y: i32) -> Cell {
        let (x, y) = adjust_coords(x, y, self.width, self.height);

        Cell {
            x,
            y,
            frame: self,
            val: self.cells[index(y, x, self.height)],
        }
    }

    pub fn evolve<F>(&self, f: F) -> Self
    where
        F: Fn(Cell) -> Color,
    {
        let mut frame = Frame::new(self.width, self.height);
        for x in 0..self.width as i32 {
            for y in 0..self.height as i32 {
                frame.set(x, y, f(self.cell(x, y)));
            }
        }
        frame
    }
}

impl Cell<'_> {
    pub fn left(&self) -> Self {
        self.frame.cell(self.x as i32 - 1, self.y as i32)
    }
    pub fn right(&self) -> Self {
        self.frame.cell(self.x as i32 + 1, self.y as i32)
    }
    pub fn top(&self) -> Self {
        self.frame.cell(self.x as i32, self.y as i32 - 1)
    }
    pub fn bottom(&self) -> Self {
        self.frame.cell(self.x as i32, self.y as i32 + 1)
    }
}

fn index(y: u16, x: u16, height: u16) -> usize {
    (y * height + x) as usize
}

fn adjust_coords(x: i32, y: i32, width: u16, height: u16) -> (u16, u16) {
    let w = width as i32;
    let h = height as i32;
    if x < 0 {
        return adjust_coords(x + w, y, width, height);
    }
    if y < 0 {
        return adjust_coords(x, y + h, width, height);
    }
    if x >= w {
        return adjust_coords(x - w, y, width, height);
    }
    if y >= h {
        return adjust_coords(x, y - h, width, height);
    }

    (x as u16, y as u16)
}

#[cfg(test)]
#[path = "./domain_tests.rs"]
mod domain_tests;
