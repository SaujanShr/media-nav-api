use super::{gap_too_small, target_index};

#[test]
fn target_index_is_zero_for_an_empty_playlist() {
    assert_eq!(target_index(0, 0, &[]), 0.0);
}

#[test]
fn target_index_halves_the_first_neighbor_when_moving_to_the_front() {
    assert_eq!(target_index(3, 0, &[4.0]), 2.0);
}

#[test]
fn target_index_signals_normalization_when_the_front_gap_is_too_small() {
    assert_eq!(target_index(3, 0, &[1e-10]), -1.0);
}

#[test]
fn target_index_adds_one_past_the_last_neighbor_when_moving_to_the_end() {
    assert_eq!(target_index(3, 3, &[5.0]), 6.0);
}

#[test]
fn target_index_averages_the_two_surrounding_neighbors_in_the_middle() {
    assert_eq!(target_index(3, 1, &[2.0, 4.0]), 3.0);
}

#[test]
fn gap_too_small_checks_the_front_gap_when_moving_to_the_front() {
    assert!(gap_too_small(3, 0, &[1.0 + 1e-10], 1.0));
    assert!(!gap_too_small(3, 0, &[2.0], 1.0));
}

#[test]
fn gap_too_small_is_never_true_when_moving_to_the_end() {
    assert!(!gap_too_small(3, 3, &[5.0], 6.0));
}

#[test]
fn gap_too_small_checks_both_surrounding_gaps_in_the_middle() {
    assert!(gap_too_small(3, 1, &[3.0, 3.0 + 1e-10], 3.0));
    assert!(!gap_too_small(3, 1, &[2.0, 4.0], 3.0));
}
