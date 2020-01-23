use rltk::RGB;
use specs::{Component, VecStorage};
use std::cmp::Ordering;

pub mod layer {
  pub const BACK: i8 = -64;
  pub const FORE: i8 = 63;
  pub const MID: i8 = 0;
}

/// Renderable is a glyph definition
#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
  pub glyph: u8,
  pub fg: RGB,
  pub bg: Option<RGB>,
  pub layer: i8,
}

impl Default for Renderable {
  fn default() -> Self {
    Renderable {
      glyph: 0,
      fg: RGB::named(rltk::BLACK),
      bg: None,
      layer: layer::MID,
    }
  }
}

impl Eq for Renderable {}
impl PartialEq for Renderable {
  fn eq(&self, other: &Self) -> bool {
    self.glyph == other.glyph
      && self.fg == other.fg
      && self.bg == other.bg
      && self.layer == other.layer
  }
}

impl Ord for Renderable {
  fn cmp(&self, other: &Self) -> Ordering {
    self.layer.cmp(&other.layer)
  }
}
impl PartialOrd for Renderable {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(&other))
  }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct AdditionalRenderables {
  pub renderables: Vec<AdditionalRenderable>,
}

impl Eq for AdditionalRenderables {}
impl PartialEq for AdditionalRenderables {
  fn eq(&self, other: &Self) -> bool {
    if self.renderables.len() != other.renderables.len() {
      return false;
    }

    for i in 0..self.renderables.len() {
      if self.renderables[i] != other.renderables[i] {
        return false;
      }
    }

    true
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct AdditionalRenderable {
  pub offset: RenderOffset,
  pub renderable: Renderable,
}
impl AdditionalRenderable {
  pub fn new(render: Renderable, h: i32, v: i32) -> AdditionalRenderable {
    AdditionalRenderable {
      renderable: render,
      offset: RenderOffset {
        horizontal: h,
        vertical: v,
      },
    }
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct RenderOffset {
  pub vertical: i32,
  pub horizontal: i32,
}
