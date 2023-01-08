use mark_one_game_engine::model::uid_tracker::UIDTracker;

#[test]
fn can_request_a_uid()
{
    let mut tracker = UIDTracker::instantiate_new();

    assert_eq!(tracker.request_new_uid(), 1);
}

#[test]
fn can_track_large_number_of_uid_s()
{
    let mut tracker = UIDTracker::instantiate_new();

    for _i in 0..500
    {
        tracker.request_new_uid();
    }

    assert_eq!(tracker.request_new_uid(), 501);
}

#[test]
fn can_instantiate_at_a_starting_int()
{
    let mut tracker = UIDTracker::instantiate_new_from_starting_int(500);

    assert_eq!(tracker.request_new_uid(), 500);
}