use specs::{Component, VecStorage};

/// Pos is a screen position
#[derive(PartialEq, Eq, PartialOrd, Ord, Component)]
#[storage(VecStorage)]
pub struct Pos {
  pub y: i32,
  pub x: i32,
}

/// Marker for this is the player
#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

/// Marker for objects that block movement
#[derive(Component)]
#[storage(VecStorage)]
pub struct Solid {}
