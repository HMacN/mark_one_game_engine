use materiel::sprite_tracker::{SpriteTracker};
use materiel::sprite_bundle_spawner::AvailableSprites::*;

#[test]
fn retrieve_details_of_sprite_after_one_sprite_added()
{
    let mut manager = SpriteTracker::instantiate_new();

    manager.add_sprite_bundle(Water, 100.0, 100.0, 1);

    let entry = manager.get_details_by_uid(1);

    assert_eq!(entry.get_sprite(), Water);
    assert_eq!(entry.get_x_coord(), 100.0);
    assert_eq!(entry.get_y_coord(), 100.0);
    assert_eq!(entry.get_uid(), 1);
}

#[test]
fn retrieve_details_of_sprite_after_multiple_sprites_added()
{
    let mut manager = SpriteTracker::instantiate_new();

    manager.add_sprite_bundle(Grass, 50.0, 50.0, 1);
    manager.add_sprite_bundle(Grass, 50.0, 50.0, 2);
    manager.add_sprite_bundle(Grass, 50.0, 50.0, 3);
    manager.add_sprite_bundle(Grass, 50.0, 50.0, 4);
    manager.add_sprite_bundle(Water, 100.0, 100.0, 5);
    manager.add_sprite_bundle(Grass, 50.0, 50.0, 6);

    let entry = manager.get_details_by_uid(5);

    assert_eq!(entry.get_sprite(), Water);
    assert_eq!(entry.get_x_coord(), 100.0);
    assert_eq!(entry.get_y_coord(), 100.0);
    assert_eq!(entry.get_uid(), 5);
}

#[test]
fn return_false_if_adding_a_uid_already_in_use()
{
    let mut manager = SpriteTracker::instantiate_new();
    manager.add_sprite_bundle(Grass, 50.0, 50.0, 1);

    assert_eq!(manager.add_sprite_bundle(Grass, 50.0, 50.0, 1), false);
}

#[test]
fn return_true_if_adding_a_uid_not_already_in_use()
{
    let mut manager = SpriteTracker::instantiate_new();
    manager.add_sprite_bundle(Grass, 50.0, 50.0, 1);

    assert_eq!(manager.add_sprite_bundle(Grass, 50.0, 50.0, 2), true);
}

#[test]
fn return_default_values_if_an_invalid_uid_is_provided()
{
    let mut manager = SpriteTracker::instantiate_new();

    manager.add_sprite_bundle(Grass, 50.0, 50.0, 1);

    let entry = manager.get_details_by_uid(2);

    assert_eq!(entry.get_sprite(), ImageNotFound);
    assert_eq!(entry.get_x_coord(), 0.0);
    assert_eq!(entry.get_y_coord(), 0.0);
    assert_eq!(entry.get_uid(), 0);
}

#[test]
fn can_update_values_of_a_given_entry()
{
    let mut manager = SpriteTracker::instantiate_new();

    manager.add_sprite_bundle(Grass, 50.0, 50.0, 1);

    manager.update_details(1, Water, 100.0, 100.0);

    let entry = manager.get_details_by_uid(1);

    assert_eq!(entry.get_sprite(), Water);
    assert_eq!(entry.get_x_coord(), 100.0);
    assert_eq!(entry.get_y_coord(), 100.0);
    assert_eq!(entry.get_uid(), 1);
}

#[test]
fn update_details_returns_true_when_successful()
{
    let mut manager = SpriteTracker::instantiate_new();

    manager.add_sprite_bundle(Grass, 50.0, 50.0, 1);

    assert_eq!(manager.update_details(1, Water, 100.0, 100.0), true);
}

#[test]
fn update_details_returns_false_when_unsuccessful()
{
    let mut manager = SpriteTracker::instantiate_new();

    manager.add_sprite_bundle(Grass, 50.0, 50.0, 1);

    assert_eq!(manager.update_details(2, Water, 100.0, 100.0), false);
}

#[test]
fn find_if_any_entry_not_listed_as_spawned()
{
    let mut manager = SpriteTracker::instantiate_new();

    manager.add_sprite_bundle(Grass, 50.0, 50.0, 1);
    manager.add_sprite_bundle(Grass, 50.0, 50.0, 2);

    assert!(manager.find_next_entry_to_spawn().1);
}

#[test]
fn get_next_entry_to_be_spawned()
{
    let mut manager = SpriteTracker::instantiate_new();

    manager.add_sprite_bundle(Grass, 50.0, 50.0, 1);
    manager.add_sprite_bundle(Water, 100.0, 100.0, 2);

    let entry = manager.find_next_entry_to_spawn().0;

    assert_eq!(entry.get_sprite(), Grass);
    assert_eq!(entry.get_x_coord(), 50.0);
    assert_eq!(entry.get_y_coord(), 50.0);
    assert_eq!(entry.get_uid(), 1);
}

#[test]
fn marking_an_entry_as_spawned_stops_it_coming_up_as_next_entry_to_be_spawned()
{
    let mut manager = SpriteTracker::instantiate_new();

    manager.add_sprite_bundle(Grass, 50.0, 50.0, 1);
    manager.add_sprite_bundle(Water, 100.0, 100.0, 2);

    manager.mark_entry_as_spawned(1);

    let entry = manager.find_next_entry_to_spawn().0;

    assert_eq!(entry.get_uid(), 2);
}

#[test]
fn handles_marking_an_invalid_entry_as_spawned()
{
    let mut manager = SpriteTracker::instantiate_new();

    manager.add_sprite_bundle(Grass, 50.0, 50.0, 1);
    manager.add_sprite_bundle(Water, 100.0, 100.0, 2);

    manager.mark_entry_as_spawned(3);

    let entry = manager.find_next_entry_to_spawn().0;

    assert_eq!(entry.get_uid(), 1);
}