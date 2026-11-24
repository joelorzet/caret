mod common;

use caret_layout::{distribute, Rect};
use common::AREA;

#[test]
fn distribute_sums_exactly() {
    for total in [0u16, 1, 7, 79, 80, 1000] {
        for weights in [vec![0.5, 0.5], vec![0.5, 0.25, 0.25], vec![1.0 / 3.0; 3]] {
            let sizes = distribute(total, &weights, 1);
            if (total as usize) >= weights.len() {
                assert_eq!(
                    sizes.iter().map(|&s| s as u32).sum::<u32>(),
                    total as u32,
                    "total {total} weights {weights:?}"
                );
            }
        }
    }
}

#[test]
fn distribute_splits_evenly_when_it_can() {
    assert_eq!(distribute(78, &[1.0 / 3.0; 3], 1), vec![26, 26, 26]);
    assert_eq!(distribute(80, &[0.5, 0.5], 1), vec![40, 40]);
}

#[test]
fn distribute_gives_the_remainder_to_the_largest_fraction() {
    assert_eq!(distribute(79, &[0.5, 0.5], 1), vec![40, 39]);
    assert_eq!(distribute(78, &[0.5, 0.25, 0.25], 1), vec![39, 20, 19]);
}

#[test]
fn distribute_honours_the_minimum() {
    let sizes = distribute(10, &[0.98, 0.01, 0.01], 2);
    assert!(sizes.iter().all(|&s| s >= 2), "got {sizes:?}");
    assert_eq!(sizes.iter().map(|&s| s as u32).sum::<u32>(), 10);
}

#[test]
fn distribute_degrades_evenly_when_the_minimum_cannot_be_met() {
    let sizes = distribute(4, &[0.25; 4], 10);
    assert_eq!(sizes, vec![1, 1, 1, 1]);
}

#[test]
fn distribute_handles_zero_and_negative_weights() {
    let sizes = distribute(10, &[0.0, 0.0], 0);
    assert_eq!(sizes.iter().map(|&s| s as u32).sum::<u32>(), 10);
    let sizes = distribute(10, &[-1.0, 1.0], 0);
    assert_eq!(sizes.iter().map(|&s| s as u32).sum::<u32>(), 10);
}

#[test]
fn rect_edges_and_overlap() {
    let a = Rect::new(0, 0, 10, 5);
    let b = Rect::new(10, 2, 10, 5);
    assert_eq!((a.right(), a.bottom()), (10, 5));
    assert_eq!(a.overlap_x(&b), 0);
    assert_eq!(a.overlap_y(&b), 3);
    assert!(a.contains(9, 4));
    assert!(!a.contains(10, 4));
    assert_eq!(a.area(), 50);
}

#[test]
fn rect_inset_saturates() {
    assert_eq!(Rect::new(0, 0, 2, 2).inset(5, 5), Rect::new(5, 5, 0, 0));
}

#[test]
fn a_single_pane_fills_the_area() {
    let t = common::tab();
    let r = t.resolve(AREA);
    assert_eq!(common::rect_of(&r, 0), AREA);
    assert!(r.dividers.is_empty());
}
