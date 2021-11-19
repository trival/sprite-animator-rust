use animator::domain::{Color, Frame};
use bracket_lib::prelude::*;

struct State {
    frame: Frame,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        for x in 0..self.frame.width {
            for y in 0..self.frame.height {
                ctx.print(
                    x,
                    y,
                    match self.frame.get(x.into(), y.into()) {
                        Color::Dark => " ",
                        Color::Light => "#",
                        Color::Medium => "|",
                    },
                );
            }
        }
        self.frame = self.frame.evolve(|c| c.right().val)
    }
}

fn main() -> BError {
    let w = 9;
    let h = 9;
    let context = BTermBuilder::simple(w, h)?
        .with_title("Hello Minimal Bracket World")
        .with_fps_cap(5.0)
        .build()?;

    let gs: State = State {
        frame: create_frame(w, h),
    };
    main_loop(context, gs)
}

fn create_frame(w: i32, h: i32) -> Frame {
    let mut f = Frame::new(w as u16, h as u16);
    for x in 0..w {
        for y in 0..h {
            if (x + y) % 3 == 0 {
                f.set(x, y, Color::Light)
            }
            if (x + y + 1) % 3 == 0 {
                f.set(x, y, Color::Medium)
            }
        }
    }
    f
}
