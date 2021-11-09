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
}

impl Frame {
    fn new(width: u16, height: u16) -> Self {
        let cells = vec![0u8; (width * height).into()];
        let frame = Frame {
            width: width,
            height: height,
            cells: cells,
        };

        frame
    }

    fn get(&self, x: i32, y: i32) -> u8 {
        let (x, y) = adjust_coords(x, y, self.width, self.height);

        self.cells[(y * self.height as i32 + x) as usize]
    }

    fn set(&mut self, x: i32, y: i32, val: u8) {
        let (x, y) = adjust_coords(x, y, self.width, self.height);

        self.cells[(y * self.height as i32 + x) as usize] = val
    }
}

fn adjust_coords(x: i32, y: i32, width: u16, height: u16) -> (i32, i32) {
    if x < 0 {
        return adjust_coords(x + width as i32, y, width, height);
    }

    if y < 0 {
        return adjust_coords(x, y + height as i32, width, height);
    }

    if x >= width as i32 {
        return adjust_coords(x - width as i32, y, width, height);
    }

    if y >= height as i32 {
        return adjust_coords(x, y - height as i32, width, height);
    }

    (x, y)
}

#[cfg(test)]
#[path = "./domain_tests.rs"]
mod domain_tests;
