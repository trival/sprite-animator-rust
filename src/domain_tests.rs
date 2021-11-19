use super::Color::{Dark, Light, Medium};
use super::*;

#[test]
fn frame_vals() {
    let mut f = Frame::new(3, 3);

    assert_eq!(f.get(1, 1), Dark);
    assert_eq!(f.get(4, 4), Dark);
    assert_eq!(f.get(-2, -2), Dark);

    f.set(1, 1, Light);
    assert_eq!(f.get(1, 1), Light);
    assert_eq!(f.get(4, 4), Light);
    assert_eq!(f.get(-2, -2), Light);
}

#[test]
fn frame_cells() {
    let mut f = Frame::new(3, 3);
    let c1 = f.cell(1, 1);

    assert_eq!(c1.val, Dark);
    assert_eq!(c1.x, 1);
    assert_eq!(c1.y, 1);
    assert_eq!(c1.top().val, Dark);
    assert_eq!(c1.bottom().val, Dark);
    assert_eq!(c1.right().val, Dark);
    assert_eq!(c1.left().val, Dark);
    assert_eq!(c1.right().x, 2);

    f.set(1, 1, Light);
    f.set(0, 1, Medium);
    f.set(1, 0, Light);
    f.set(2, 1, Medium);
    f.set(1, 2, Light);
    let c2 = f.cell(1, 1);

    assert_eq!(c2.val, Light);
    assert_eq!(c2.top().val, Light);
    assert_eq!(c2.bottom().val, Light);
    assert_eq!(c2.right().val, Medium);
    assert_eq!(c2.left().val, Medium);
}

#[test]
fn can_evolve() {
    let mut f = Frame::new(2, 2);
    f.set(0, 0, Light);
    f.set(0, 1, Light);

    let f2 = f.evolve(|cell| if cell.val == Light { Dark } else { Light });

    assert_eq!(f2.get(0, 0), Dark);
    assert_eq!(f2.get(0, 1), Dark);
    assert_eq!(f2.get(1, 0), Light);
    assert_eq!(f2.get(1, 1), Light);

    let f3 = f.evolve(|cell| cell.right().val);

    assert_eq!(f3.get(0, 0), Dark);
    assert_eq!(f3.get(0, 1), Dark);
    assert_eq!(f3.get(1, 0), Light);
    assert_eq!(f3.get(1, 1), Light);
}
