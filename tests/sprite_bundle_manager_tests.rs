use materiel::sprite_bundle_manager::{AvailableSprites, SpriteBundleManager};

#[test]
fn retrieve_details_of_sprite_after_one_sprite_added()
{
    let mut manager = SpriteBundleManager::instantiate_new();
    let target_tuple: (AvailableSprites, f32, f32) = (AvailableSprites::Water, 100.0, 100.0);

    manager.add_sprite_bundle(AvailableSprites::Water, 100.0, 100.0, 1);

    assert_eq!(manager.get_details_by_uid(1), target_tuple);
}

#[test]
fn retrieve_details_of_sprite_after_multiple_sprites_added()
{
    let mut manager = SpriteBundleManager::instantiate_new();
    let target_tuple: (AvailableSprites, f32, f32) = (AvailableSprites::Water, 100.0, 100.0);

    manager.add_sprite_bundle(AvailableSprites::Grass, 50.0, 50.0, 1);
    manager.add_sprite_bundle(AvailableSprites::Grass, 50.0, 50.0, 2);
    manager.add_sprite_bundle(AvailableSprites::Grass, 50.0, 50.0, 3);
    manager.add_sprite_bundle(AvailableSprites::Grass, 50.0, 50.0, 4);
    manager.add_sprite_bundle(AvailableSprites::Water, 100.0, 100.0, 5);
    manager.add_sprite_bundle(AvailableSprites::Grass, 50.0, 50.0, 6);

    assert_eq!(manager.get_details_by_uid(5), target_tuple);
}

#[test]
fn return_false_if_adding_a_uid_already_in_use()
{
    let mut manager = SpriteBundleManager::instantiate_new();
    manager.add_sprite_bundle(AvailableSprites::Grass, 50.0, 50.0, 1);

    assert_eq!(manager.add_sprite_bundle(AvailableSprites::Grass, 50.0, 50.0, 1), false);
}

#[test]
fn return_true_if_adding_a_uid_not_already_in_use()
{
    let mut manager = SpriteBundleManager::instantiate_new();
    manager.add_sprite_bundle(AvailableSprites::Grass, 50.0, 50.0, 1);

    assert_eq!(manager.add_sprite_bundle(AvailableSprites::Grass, 50.0, 50.0, 2), true);
}

#[test]
fn return_default_values_if_an_invalid_uid_is_provided()
{
    let mut manager = SpriteBundleManager::instantiate_new();
    let target_tuple: (AvailableSprites, f32, f32) = (AvailableSprites::ImageNotFound, 0.0, 0.0);

    manager.add_sprite_bundle(AvailableSprites::Grass, 50.0, 50.0, 1);

    assert_eq!(manager.get_details_by_uid(2), target_tuple);
}