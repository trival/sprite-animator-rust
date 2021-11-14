use std::fmt::Pointer;

mod colors {

    pub struct Color {
        r: f32,
        g: f32,
        b: f32,
    }

    impl Color {
        const fn gray(val: f32) -> Color {
            Color {
                r: val,
                g: val,
                b: val,
            }
        }
    }

    pub const LIGHT: Color = Color::gray(0.25);
    pub const MEDIUM: Color = Color::gray(0.5);
    pub const DARK: Color = Color::gray(0.75);
}

struct Pallete {
    colors: [colors::Color; 3],
}

impl Default for Pallete {
    fn default() -> Self {
        Pallete {
            colors: [colors::LIGHT, colors::MEDIUM, colors::DARK],
        }
    }
}

struct Frame {
    width: u16,
    height: u16,
    cells: Vec<u8>,
}

struct Cell<'a> {
    x: u16,
    y: u16,
    frame: &'a Frame,
    val: u8,
}

impl Frame {
    fn new(width: u16, height: u16) -> Self {
        let cells = vec![0u8; (width * height).into()];
        let frame = Frame {
            width,
            height,
            cells,
        };

        frame
    }

    fn get(&self, x: i32, y: i32) -> u8 {
        let (x, y) = adjust_coords(x, y, self.width, self.height);

        self.cells[index(y, x, self.height)]
    }

    fn set(&mut self, x: i32, y: i32, val: u8) {
        let (x, y) = adjust_coords(x, y, self.width, self.height);

        self.cells[index(y, x, self.height)] = val
    }

    fn cell(&self, x: i32, y: i32) -> Cell {
        let (x, y) = adjust_coords(x, y, self.width, self.height);

        Cell {
            x,
            y,
            frame: self,
            val: self.cells[index(y, x, self.height)],
        }
    }
}

impl Cell<'_> {
    fn left(&self) -> Self {
        self.frame.cell(self.x as i32 - 1, self.y as i32)
    }
    fn right(&self) -> Self {
        self.frame.cell(self.x as i32 + 1, self.y as i32)
    }
    fn top(&self) -> Self {
        self.frame.cell(self.x as i32, self.y as i32 - 1)
    }
    fn bottom(&self) -> Self {
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
