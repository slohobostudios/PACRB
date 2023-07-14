use super::super::{
    traits::{cast_element, Element as ElementTrait},
    Element,
};
use crate::{
    events::*,
    ui_settings::UISettings,
    utils::{grid::UIGrid, positioning::UIPosition},
};
use sfml::{
    graphics::{IntRect, RenderTexture},
    system::{Vector2, Vector2i},
    window::Event as SFMLEvent,
};
use std::iter;
use utils::resource_manager::ResourceManager;

#[derive(Debug, Default, Clone)]
pub struct Grid {
    position: UIPosition,
    global_bounds: IntRect,
    grid: UIGrid,
    elements: Vec<Vec<Element>>,
    from: Vector2<u16>,
    pagination_size: Vector2<u16>,
}

impl Grid {
    pub fn new(
        _resource_manager: &ResourceManager,
        mut elements: Vec<Vec<Element>>,
        position: UIPosition,
        pagination_size: Vector2<u16>,
        desired_size: Vector2i,
    ) -> Self {
        for vec_elements in &mut elements {
            if vec_elements.len() < pagination_size.y.into() {
                vec_elements.append(&mut vec![
                    Element::Empty;
                    usize::from(pagination_size.y) - vec_elements.len()
                ]);
                vec_elements.shrink_to_fit();
            }
        }

        if elements.len() < pagination_size.x.into() {
            let mut vec_of_empty_elements = vec![Element::Empty; pagination_size.y.into()];
            vec_of_empty_elements.shrink_to_fit();
            elements.append(&mut vec![
                vec_of_empty_elements;
                usize::from(pagination_size.x) - elements.len()
            ]);
            elements.shrink_to_fit();
        }

        let mut ge = Self {
            grid: UIGrid::new(pagination_size.x, pagination_size.y, desired_size),
            position,
            elements,
            global_bounds: IntRect::new(0, 0, desired_size.x, desired_size.y),
            from: Vector2::new(0, 0),
            pagination_size,
        };
        ge.update_size();

        ge
    }

    pub fn grid(&self) -> UIGrid {
        self.grid
    }

    pub fn update_cell_size(&mut self) {
        for ele in self.elements.iter_mut().flatten() {
            ele.update_size();
        }
    }

    fn update_cell_position(&mut self) {
        let cpy_grid = self.grid;
        for (pos, ele) in self.expose_paginated_elements_mut_with_coords() {
            let pos: Vector2<u16> = pos.try_into_other().unwrap_or_default();
            ele.update_position(
                cpy_grid
                    .global_cell_bounds(pos.x, pos.y)
                    .unwrap_or_default()
                    .into_other(),
            );
        }
    }

    fn paginate(&mut self) {
        self.update_cell_position();
    }

    pub fn paginate_left(&mut self, amount: i32) {
        let is_first_page = i32::from(self.from.x) - amount >= 0;
        let is_last_page = i32::from(self.from.x) - amount + i32::from(self.pagination_size.x)
            >= i32::from(self.grid.m);

        if is_first_page {
            self.from.x = 0;
        } else if is_last_page {
            self.from.x = self.grid.m - self.pagination_size.x;
        } else {
            self.from.x = u16::try_from(i32::from(self.from.y) + amount).unwrap_or(u16::MAX);
        }

        self.paginate();
    }

    pub fn paginate_right(&mut self, amount: i32) {
        self.paginate_left(-amount);
    }

    pub fn paginate_up(&mut self, amount: i32) {
        let is_first_page = i32::from(self.from.y) - amount >= 0;
        let is_last_page = i32::from(self.from.y) - amount + i32::from(self.pagination_size.y)
            >= i32::from(self.grid.n);

        if is_first_page {
            self.from.y = 0;
        } else if is_last_page {
            self.from.y = self.grid.n - self.pagination_size.y;
        } else {
            self.from.y = u16::try_from(i32::from(self.from.y) + amount).unwrap_or(u16::MAX);
        }

        self.paginate();
    }

    pub fn paginate_down(&mut self, amount: i32) {
        self.paginate_up(-amount);
    }

    pub fn expose_paginated_elements(&self) -> impl Iterator<Item = &Element> {
        let start_x = self.from.x as usize;
        let start_y = self.from.y as usize;
        let end_x = start_x + self.pagination_size.x as usize;
        let end_y = start_y + self.pagination_size.y as usize;

        self.elements[start_x..end_x]
            .iter()
            .flat_map(move |slice| &slice[start_y..end_y])
    }

    pub fn expose_paginated_elements_mut(&mut self) -> impl Iterator<Item = &mut Element> {
        let start_x = self.from.x as usize;
        let start_y = self.from.y as usize;
        let end_x = start_x + self.pagination_size.x as usize;
        let end_y = start_y + self.pagination_size.y as usize;

        self.elements[start_x..end_x]
            .iter_mut()
            .flat_map(move |slice| &mut slice[start_y..end_y])
    }

    pub fn expose_paginated_elements_mut_with_coords(
        &mut self,
    ) -> impl Iterator<Item = (Vector2<usize>, &mut Element)> {
        let start_x = self.from.x as usize;
        let start_y = self.from.y as usize;
        let end_x = start_x + self.pagination_size.x as usize;
        let end_y = start_y + self.pagination_size.y as usize;

        self.elements[start_x..end_x]
            .iter_mut()
            .enumerate()
            .flat_map(move |(x, slice)| {
                iter::repeat(x).zip(slice[start_y..end_y].iter_mut().enumerate())
            })
            .map(|(x, (y, ele))| (Vector2::new(x, y), ele))
    }

    pub fn mut_children(&mut self) -> impl Iterator<Item = &mut Element> {
        self.elements.iter_mut().flatten()
    }
}

impl ElementTrait for Grid {
    cast_element!();
    fn update(&mut self, resource_manager: &ResourceManager) -> (Vec<Event>, bool) {
        let mut rerender = false;
        let mut events = Vec::new();
        for ele in self.expose_paginated_elements_mut() {
            let mut event = ele.update(resource_manager);
            rerender |= event.1;
            events.append(&mut event.0);
        }

        (events, rerender)
    }

    fn update_size(&mut self) {
        self.update_cell_size();

        let cell_size: Vector2i = self.grid.cell_size();
        self.global_bounds.width = i32::from(self.pagination_size.x) * cell_size.x;
        self.global_bounds.height = i32::from(self.pagination_size.y) * cell_size.y;

        self.grid.global_bounds.width = i32::from(self.grid.m) * cell_size.x;
        self.grid.global_bounds.height = i32::from(self.grid.n) * cell_size.y;

        self.update_cell_size();
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.global_bounds = self
            .position
            .center_with_size(relative_rect, self.global_bounds.size());
        self.grid.global_bounds = self.global_bounds;

        self.update_cell_position();
    }

    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> (Vec<Event>, bool) {
        let mut rerender = false;
        let mut events = Vec::new();
        for ele in self.expose_paginated_elements_mut() {
            let mut event = ele.event_handler(ui_settings, event);
            rerender |= event.1;
            events.append(&mut event.0);
        }
        (events, rerender)
    }

    fn render(&mut self, window: &mut RenderTexture) {
        for ele in self.expose_paginated_elements_mut() {
            ele.render(window);
        }
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.position = ui_position;
        self.update_position(relative_rect);
    }
}
