use rltk::RGB;
use specs::{Component, VecStorage};

/// Renderable is a glyph definition
#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
  pub glyph: u8,
  pub fg: RGB,
  pub bg: RGB,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct AdditionalRenderables {
  pub renderables: Vec<AdditionalRenderable>,
}
pub struct AdditionalRenderable {
  pub renderable: Renderable,
  pub offset: RenderOffset,
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
pub struct RenderOffset {
  pub horizontal: i32,
  pub vertical: i32,
}
