use super::traits::*;
use crate::{
    elements::traits::{cast_element, Element},
    utils::positioning::UIPosition,
};
use sfml::{
    graphics::{
        FloatRect, IntRect, PrimitiveType, RcSprite, RenderStates, RenderTarget, RenderTexture,
        Transformable,
    },
    system::{Vector2, Vector2u},
};
use utils::{
    arithmetic_util_functions::i32_ceil_div, quads::Quad, resource_manager::ResourceManager,
    sfml_util_functions::bottom_right_rect_coords,
};

#[repr(usize)]
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum SliceName {
    Left,
    Middle,
    Right,
}

impl SliceName {
    fn repr(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Middle => "middle",
            Self::Right => "right",
        }
    }
}

/// This struct NEEDS to be defined on the heap.
/// It stores and internal array that if defined on the stack, can cause stack oveflow.
#[derive(Debug, Default, Clone)]
pub struct Repeatable3x1Sprite {
    global_bounds: IntRect,
    position: UIPosition,
    desired_size: u16,
    num_tiles: u16,
    left_sprite: RcSprite,
    middle_sprite: RcSprite,
    middle_vertex_array: Quad,
    right_sprite: RcSprite,
}

impl Repeatable3x1Sprite {
    pub fn new(
        resource_manager: &ResourceManager,
        asset_id: &str,
        frame_id: usize,
        position: UIPosition,
        desired_size: u16,
        scale: f32,
    ) -> Self {
        let mut rps = Self {
            position,
            desired_size,
            left_sprite: resource_manager
                .fetch_asset(asset_id)
                .get_rc_sprite_with_slice_name_and_frame_num(SliceName::Left.repr(), frame_id),
            middle_sprite: resource_manager
                .fetch_asset(asset_id)
                .get_rc_sprite_with_slice_name_and_frame_num(SliceName::Middle.repr(), frame_id),
            middle_vertex_array: Quad::default(),
            right_sprite: resource_manager
                .fetch_asset(asset_id)
                .get_rc_sprite_with_slice_name_and_frame_num(SliceName::Right.repr(), frame_id),
            ..Default::default()
        };
        for sprite in rps.compact_sprites_mut() {
            sprite.set_scale(Vector2::new(scale, scale));
        }
        rps.update_size();

        rps
    }

    fn compact_sprites_mut(&mut self) -> [&mut RcSprite; 3] {
        [
            &mut self.left_sprite,
            &mut self.middle_sprite,
            &mut self.right_sprite,
        ]
    }
}

impl TilingSprite for Repeatable3x1Sprite {
    fn set_desired_size(&mut self, desired_size: Vector2u) {
        self.desired_size = desired_size.x.try_into().unwrap_or(u16::MAX);

        self.update_size();
    }

    fn box_clone(&self) -> Box<dyn TilingSprite> {
        Box::new(self.clone())
    }

    fn desired_size(&self) -> Vector2u {
        Vector2::new(self.desired_size.into(), 1)
    }
}

impl Element for Repeatable3x1Sprite {
    cast_element!();
    fn update_position(&mut self, relative_rect: IntRect) {
        self.global_bounds = self
            .position
            .center_with_size(relative_rect, self.global_bounds.size());

        let gl_f = self.global_bounds.as_other();
        let gl_f_size_pos = bottom_right_rect_coords(gl_f);
        self.left_sprite.set_position(gl_f.position());
        self.right_sprite.set_position(Vector2::new(
            gl_f_size_pos.x - self.right_sprite.global_bounds().width,
            gl_f.top,
        ));

        let ls_size_pos = bottom_right_rect_coords(self.left_sprite.global_bounds());
        self.middle_vertex_array = Quad::with_positions_from_rect(
            &self.middle_vertex_array,
            FloatRect::new(
                ls_size_pos.x,
                gl_f.top,
                self.right_sprite.global_bounds().left - ls_size_pos.x,
                self.left_sprite.global_bounds().height,
            ),
        )
    }

    fn update_size(&mut self) {
        self.num_tiles = i32_ceil_div(
            self.desired_size.into(),
            self.middle_sprite.global_bounds().width as i32,
        )
        .try_into()
        .unwrap_or_default();

        if self.num_tiles < 3 {
            self.num_tiles = 3;
        }

        self.global_bounds.width = (self.left_sprite.global_bounds().width
            + self.middle_sprite.global_bounds().width * f32::from(self.num_tiles)
            + self.right_sprite.global_bounds().width) as i32;
        self.global_bounds.height = self.middle_sprite.global_bounds().height as i32;

        self.middle_vertex_array = Quad::from(self.middle_sprite.clone());
    }

    fn render(&mut self, window: &mut RenderTexture) {
        let mut rs = RenderStates::default();
        rs.set_texture(self.middle_sprite.texture());
        window.draw_primitives(&self.middle_vertex_array.0, PrimitiveType::QUADS, &rs);

        window.draw(&self.left_sprite);
        window.draw(&self.right_sprite);
    }

    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.position = ui_position;
        self.update_size();
        self.update_position(relative_rect);
    }
}

impl TilingSpriteElement for Repeatable3x1Sprite {
    fn box_clone(&self) -> Box<dyn TilingSpriteElement> {
        Box::new(self.clone())
    }
}
