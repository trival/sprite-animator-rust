use super::*;

#[test]
fn frame_vals() {
    let mut f = Frame::new(3, 3);

    assert_eq!(f.get(1, 1), 0);
    assert_eq!(f.get(4, 4), 0);
    assert_eq!(f.get(-2, -2), 0);

    f.set(1, 1, 2);
    assert_eq!(f.get(1, 1), 2);
    assert_eq!(f.get(4, 4), 2);
    assert_eq!(f.get(-2, -2), 2);
}

#[test]
fn frame_cells() {
    let mut f = Frame::new(3, 3);
    let c1 = f.cell(1, 1);

    assert_eq!(c1.val, 0);
    assert_eq!(c1.x, 1);
    assert_eq!(c1.y, 1);
    assert_eq!(c1.top().val, 0);
    assert_eq!(c1.bottom().val, 0);
    assert_eq!(c1.right().val, 0);
    assert_eq!(c1.left().val, 0);
    assert_eq!(c1.right().x, 2);

    f.set(1, 1, 1);
    f.set(0, 1, 2);
    f.set(1, 0, 3);
    f.set(2, 1, 4);
    f.set(1, 2, 5);
    let c2 = f.cell(1, 1);

    assert_eq!(c2.val, 1);
    assert_eq!(c2.top().val, 3);
    assert_eq!(c2.bottom().val, 5);
    assert_eq!(c2.right().val, 4);
    assert_eq!(c2.left().val, 2);
}
