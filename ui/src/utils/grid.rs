use sfml::{
    graphics::{IntRect, Rect},
    system::{Vector2, Vector2i},
};
use tracing::error;

#[derive(Debug, Clone, Copy, Default)]
pub struct UIGrid {
    pub m: u16,
    pub n: u16,
    pub global_bounds: IntRect,
}

impl UIGrid {
    pub fn new(m: u16, n: u16, size: Vector2i) -> Self {
        Self {
            m,
            n,
            global_bounds: IntRect::from_vecs(Vector2::new(0, 0), size),
        }
    }

    pub fn cell_size(&self) -> Vector2<i32> {
        Vector2::new(
            self.global_bounds.width.abs() / i32::from(self.m),
            self.global_bounds.height.abs() / i32::from(self.n),
        )
    }

    pub fn cell_pos(&self, m: u16, n: u16) -> Option<Vector2<i32>> {
        if m > self.m || n > self.n {
            None
        } else {
            let cell_size = self.cell_size();
            Some(Vector2::new(
                i32::from(m) * cell_size.x,
                i32::from(n) * cell_size.y,
            ))
        }
    }

    pub fn cell_bounds(&self, m: u16, n: u16) -> Option<Rect<i32>> {
        Some(Rect::from_vecs(self.cell_pos(m, n)?, self.cell_size()))
    }

    pub fn global_cell_position(&self, m: u16, n: u16) -> Option<Vector2i> {
        let cell_pos: Vector2i = self.cell_pos(m, n)?.into_other();
        let grid_global_pos = self
            .global_bounds
            .position()
            .try_into_other()
            .map_err(|e| error!("{}", e))
            .ok()?;

        Some(cell_pos + grid_global_pos)
    }

    pub fn global_cell_bounds(&self, m: u16, n: u16) -> Option<IntRect> {
        Some(Rect::from_vecs(
            self.global_cell_position(m, n)?,
            self.cell_size().into_other(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cell_size() {
        let grid = UIGrid::new(50, 50, Vector2::new(10000, 10000));
        assert_eq!(grid.cell_size(), Vector2::new(200, 200));
    }

    #[test]
    fn cell_pos() {
        let grid = UIGrid::new(50, 50, Vector2::new(10000, 10000));
        assert_eq!(
            grid.cell_pos(25, 25).expect("unit-test"),
            Vector2::new(5000, 5000)
        );
    }

    #[test]
    fn cell_bounds() {
        let grid = UIGrid::new(50, 50, Vector2::new(10000, 10000));
        assert_eq!(
            grid.cell_bounds(25, 25).expect("unit-test"),
            Rect::new(5000, 5000, 200, 200)
        );
    }

    #[test]
    fn global_cell_pos() {
        let grid = UIGrid::new(50, 50, Vector2::new(10000, 10000));
        assert_eq!(
            grid.global_cell_position(25, 25).expect("unit-test"),
            (grid.global_bounds.position() + Vector2::new(5000, 5000))
        );
    }

    #[test]
    fn global_cell_bounds() {
        let grid = UIGrid::new(50, 50, Vector2::new(10000, 10000));
        assert_eq!(
            grid.global_cell_bounds(25, 25).expect("unit-test"),
            IntRect::from_vecs(
                grid.global_bounds.position() + Vector2::new(5000, 5000),
                grid.cell_size().into_other()
            ),
        );
    }
}
