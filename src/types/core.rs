use specs::{Component, VecStorage};

/// Pos is a screen position
#[derive(Component)]
#[storage(VecStorage)]
pub struct Pos {
  pub x: i32,
  pub y: i32,
}

/// Marker for this is the player
#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

/// Marker for objects that block movement
#[derive(Component)]
#[storage(VecStorage)]
pub struct Solid {}
