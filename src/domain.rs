mod Colors {

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
    colors: [Colors::Color; 3],
}

impl Default for Pallete {
    fn default() -> Self {
        Pallete {
            colors: [Colors::LIGHT, Colors::MEDIUM, Colors::DARK],
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

    fn val(&self, x: i32, y: i32) -> u8 {
        if x < 0 {
            return self.val(x + self.width as i32, y);
        }

        if y < 0 {
            return self.val(x, y + self.height as i32);
        }

        if x >= self.width as i32 {
            return self.val(x - self.width as i32, y);
        }

        if y >= self.height as i32 {
            return self.val(x, y - self.height as i32);
        }

        self.cells[(y * self.height as i32 + x) as usize]
    }
}
