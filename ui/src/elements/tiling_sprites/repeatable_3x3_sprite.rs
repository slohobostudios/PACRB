use super::traits::*;
use crate::{elements::traits::Element, utils::positioning::UIPosition};
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
    TopLeft,
    Top,
    TopRight,
    Left,
    Middle,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl SliceName {
    fn repr(&self) -> &'static str {
        match self {
            Self::TopLeft => "top_left",
            Self::Top => "top",
            Self::TopRight => "top_right",
            Self::Left => "left",
            Self::Middle => "middle",
            Self::Right => "right",
            Self::BottomLeft => "bottom_left",
            Self::Bottom => "bottom",
            Self::BottomRight => "bottom_right",
        }
    }
}

/// This struct NEEDS to be defined on the heap.
/// It stores and internal array that if defined on the stack, can cause stack oveflow.
#[derive(Debug, Default, Clone)]
pub struct Repeatable3x3Sprite {
    global_bounds: IntRect,
    position: UIPosition,
    pub desired_size: Vector2<u32>,
    pub scale: f32,
    num_tiles: Vector2<u16>,
    top_left_sprite: RcSprite,
    top_sprite: RcSprite,
    top_vertex_array: Quad,
    top_right_sprite: RcSprite,
    left_sprite: RcSprite,
    left_vertex_array: Quad,
    middle_sprite: RcSprite,
    middle_vertex_array: Quad,
    right_sprite: RcSprite,
    right_vertex_array: Quad,
    bottom_left_sprite: RcSprite,
    bottom_sprite: RcSprite,
    bottom_vertex_array: Quad,
    bottom_right_sprite: RcSprite,
}

impl Repeatable3x3Sprite {
    pub fn new(
        resource_manager: &ResourceManager,
        asset_id: &str,
        frame_id: usize,
        position: UIPosition,
        desired_size: Vector2<u32>,
        scale: f32,
    ) -> Self {
        let asset = resource_manager.fetch_asset(&asset_id);
        let mut rps = Self {
            position,
            desired_size,
            scale,
            top_left_sprite: asset
                .get_rc_sprite_with_slice_name_and_frame_num(SliceName::TopLeft.repr(), frame_id),
            top_sprite: asset
                .get_rc_sprite_with_slice_name_and_frame_num(SliceName::Top.repr(), frame_id),
            top_vertex_array: Default::default(),
            top_right_sprite: asset
                .get_rc_sprite_with_slice_name_and_frame_num(SliceName::TopRight.repr(), frame_id),
            left_sprite: asset
                .get_rc_sprite_with_slice_name_and_frame_num(SliceName::Left.repr(), frame_id),
            left_vertex_array: Default::default(),
            middle_sprite: asset
                .get_rc_sprite_with_slice_name_and_frame_num(SliceName::Middle.repr(), frame_id),
            middle_vertex_array: Default::default(),
            right_sprite: asset
                .get_rc_sprite_with_slice_name_and_frame_num(SliceName::Right.repr(), frame_id),
            right_vertex_array: Default::default(),
            bottom_left_sprite: asset.get_rc_sprite_with_slice_name_and_frame_num(
                SliceName::BottomLeft.repr(),
                frame_id,
            ),
            bottom_sprite: asset
                .get_rc_sprite_with_slice_name_and_frame_num(SliceName::Bottom.repr(), frame_id),
            bottom_vertex_array: Default::default(),
            bottom_right_sprite: asset.get_rc_sprite_with_slice_name_and_frame_num(
                SliceName::BottomRight.repr(),
                frame_id,
            ),
            ..Default::default()
        };
        for sprite in rps.compact_sprites_mut() {
            sprite.set_scale(Vector2::new(scale, scale));
        }
        rps.update_size();

        rps
    }

    fn compact_sprites_mut(&mut self) -> [&mut RcSprite; 9] {
        [
            &mut self.top_left_sprite,
            &mut self.top_sprite,
            &mut self.top_right_sprite,
            &mut self.left_sprite,
            &mut self.middle_sprite,
            &mut self.right_sprite,
            &mut self.bottom_left_sprite,
            &mut self.bottom_sprite,
            &mut self.bottom_right_sprite,
        ]
    }
}

impl TilingSprite for Repeatable3x3Sprite {
    fn set_desired_size(&mut self, desired_size: Vector2u) {
        self.desired_size = desired_size;

        self.update_size();
    }

    fn box_clone(&self) -> Box<dyn TilingSprite> {
        Box::new(self.clone())
    }

    fn desired_size(&self) -> Vector2u {
        self.desired_size
    }
}

impl Element for Repeatable3x3Sprite {
    fn update_position(&mut self, relative_rect: IntRect) {
        self.global_bounds = self
            .position
            .center_with_size(relative_rect, self.global_bounds.size());
        let gl_f = self.global_bounds.as_other();
        let gl_f_size_pos = bottom_right_rect_coords(gl_f);
        self.top_left_sprite.set_position(gl_f.position());
        self.top_right_sprite.set_position(Vector2::new(
            gl_f_size_pos.x - self.top_right_sprite.global_bounds().width,
            gl_f.top,
        ));
        self.bottom_left_sprite.set_position(Vector2::new(
            gl_f.left,
            gl_f_size_pos.y - self.bottom_left_sprite.global_bounds().height,
        ));
        self.bottom_right_sprite.set_position(Vector2::new(
            gl_f_size_pos.x - self.bottom_right_sprite.global_bounds().width,
            gl_f_size_pos.y - self.bottom_right_sprite.global_bounds().height,
        ));

        let tls_right_side_x =
            self.top_left_sprite.global_bounds().left + self.top_left_sprite.global_bounds().width;
        self.top_vertex_array = Quad::with_positions_from_rect(
            &self.top_vertex_array,
            FloatRect::new(
                tls_right_side_x,
                self.top_left_sprite.global_bounds().top,
                self.top_right_sprite.global_bounds().left - tls_right_side_x,
                self.top_left_sprite.global_bounds().height,
            ),
        );
        let tls_bottom_side_y =
            self.top_left_sprite.global_bounds().top + self.top_left_sprite.global_bounds().width;
        self.left_vertex_array = Quad::with_positions_from_rect(
            &self.left_vertex_array,
            FloatRect::new(
                self.top_left_sprite.global_bounds().left,
                tls_bottom_side_y,
                self.top_left_sprite.global_bounds().width,
                self.bottom_left_sprite.position().y - tls_bottom_side_y,
            ),
        );
        let trs_bottom_side_y = self.top_right_sprite.global_bounds().top
            + self.top_right_sprite.global_bounds().height;
        self.right_vertex_array = Quad::with_positions_from_rect(
            &self.right_vertex_array,
            FloatRect::new(
                self.top_right_sprite.global_bounds().left,
                trs_bottom_side_y,
                self.top_right_sprite.global_bounds().width,
                self.bottom_right_sprite.global_bounds().top - trs_bottom_side_y,
            ),
        );
        let bls_right_side_x = self.bottom_left_sprite.global_bounds().left
            + self.bottom_left_sprite.global_bounds().width;
        self.bottom_vertex_array = Quad::with_positions_from_rect(
            &self.bottom_vertex_array,
            FloatRect::new(
                bls_right_side_x,
                self.bottom_left_sprite.global_bounds().top,
                self.bottom_right_sprite.global_bounds().left - bls_right_side_x,
                self.bottom_left_sprite.global_bounds().height,
            ),
        );
        self.middle_vertex_array = Quad::with_positions_from_rect(
            &self.middle_vertex_array,
            FloatRect::new(
                tls_right_side_x,
                tls_bottom_side_y,
                self.bottom_right_sprite.global_bounds().left - tls_right_side_x,
                self.bottom_right_sprite.global_bounds().top - tls_bottom_side_y,
            ),
        )
    }

    fn update_size(&mut self) {
        self.num_tiles = Vector2::new(
            i32_ceil_div(
                self.desired_size.x.try_into().unwrap(),
                self.middle_sprite.global_bounds().width as i32,
            ) as u16,
            i32_ceil_div(
                self.desired_size.y.try_into().unwrap(),
                self.middle_sprite.global_bounds().height as i32,
            ) as u16,
        );

        // It's kinda ugly if it's too small
        if self.num_tiles.x < 3 {
            self.num_tiles.x = 3;
        }
        if self.num_tiles.y < 3 {
            self.num_tiles.y = 3;
        }

        // calculate size_x, and size_y with sprite data. This is because of scaling.
        let size_x = self.left_sprite.global_bounds().width
            + self.middle_sprite.global_bounds().width * (f32::from(self.num_tiles.x) - 2.)
            + self.right_sprite.global_bounds().width;
        let size_y = self.top_sprite.global_bounds().height
            + self.middle_sprite.global_bounds().height * (f32::from(self.num_tiles.y) as f32 - 2.)
            + self.bottom_sprite.global_bounds().height;

        self.global_bounds.width = size_x as i32;
        self.global_bounds.height = size_y as i32;

        self.top_vertex_array = Quad::from(self.top_sprite.clone());
        self.left_vertex_array = Quad::from(self.left_sprite.clone());
        self.right_vertex_array = Quad::from(self.right_sprite.clone());
        self.middle_vertex_array = Quad::from(self.middle_sprite.clone());
        self.bottom_vertex_array = Quad::from(self.bottom_sprite.clone());
    }

    fn render(&mut self, window: &mut RenderTexture) {
        let mut rs = RenderStates::default();
        rs.set_texture(self.middle_sprite.texture());
        window.draw_primitives(&self.middle_vertex_array.0, PrimitiveType::QUADS, &rs);
        let mut rs = RenderStates::default();
        rs.set_texture(self.top_sprite.texture());
        window.draw_primitives(&self.top_vertex_array.0, PrimitiveType::QUADS, &rs);
        let mut rs = RenderStates::default();
        rs.set_texture(self.left_sprite.texture());
        window.draw_primitives(&self.left_vertex_array.0, PrimitiveType::QUADS, &rs);
        let mut rs = RenderStates::default();
        rs.set_texture(self.right_sprite.texture());
        window.draw_primitives(&self.right_vertex_array.0, PrimitiveType::QUADS, &rs);
        let mut rs = RenderStates::default();
        rs.set_texture(self.bottom_sprite.texture());
        window.draw_primitives(&self.bottom_vertex_array.0, PrimitiveType::QUADS, &rs);

        window.draw(&self.top_left_sprite);
        window.draw(&self.top_right_sprite);
        window.draw(&self.bottom_left_sprite);
        window.draw(&self.bottom_right_sprite);
    }

    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    fn box_clone(&self) -> Box<dyn Element> {
        Box::new(self.clone())
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.position = ui_position;
        self.update_size();
        self.update_position(relative_rect);
    }
}

impl TilingSpriteElement for Repeatable3x3Sprite {
    fn as_mut_element(&mut self) -> &mut dyn Element {
        self
    }

    fn as_mut_tiling_sprite(&mut self) -> &mut dyn TilingSprite {
        self
    }

    fn as_element(&self) -> &dyn Element {
        self
    }

    fn as_tiling_sprite(&self) -> &dyn TilingSprite {
        self
    }

    fn box_clone(&self) -> Box<dyn TilingSpriteElement> {
        Box::new(self.clone())
    }
}
