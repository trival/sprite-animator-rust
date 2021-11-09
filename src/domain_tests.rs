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
    let c = f.cell(1, 1);
}
