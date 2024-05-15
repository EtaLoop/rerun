use super::Range2D;

impl From<emath::Rect> for Range2D {
    #[inline]
    fn from(rect: emath::Rect) -> Self {
        Self {
            x_range: rect.x_range().into(),
            y_range: rect.y_range().into(),
        }
    }
}

impl From<Range2D> for emath::Rect {
    #[inline]
    fn from(range2d: Range2D) -> Self {
        emath::Rect::from_x_y_ranges(range2d.x_range, range2d.y_range)
    }
}

impl std::fmt::Display for Range2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {}]×[{}, {}]",
            self.x_range.0[0], self.x_range.0[1], self.y_range.0[0], self.y_range.0[1],
        )
    }
}