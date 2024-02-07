use super::Margin;

/// Geometry acts the similar purpose in zellij host,
/// the rows and cols come from [ZellijPlugin](https://github.com/zellij-org/zellij/blob/main/zellij-tile/src/lib.rs#L42), which represent the content size
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Geometry {
    pub x: u16,
    pub y: u16,
    pub rows: u16,
    pub cols: u16,
}

impl Geometry {
    /// Initialize a Geometry with x=0
    pub fn new(rows: u16, cols: u16) -> Self {
        Self::init(0, 0, rows, cols)
    }

    /// Initialize a Geometry with x and y
    fn init(x: u16, y: u16, rows: u16, cols: u16) -> Self {
        Self { x, y, rows, cols }
    }

    /// The area of the gemo. If the area is larger than the maximum value of u16, it will be
    /// clamped to u16::MAX.
    pub const fn area(self) -> u16 {
        self.cols.saturating_mul(self.rows)
    }

    /// Returns true if the gemo has no area.
    pub const fn is_empty(self) -> bool {
        self.cols == 0 || self.rows == 0
    }

    /// Returns the left coordinate of the gemo.
    pub const fn left(self) -> u16 {
        self.x
    }

    /// Returns the right coordinate of the gemo. This is the first coordinate outside of the gemo.
    ///
    /// If the right coordinate is larger than the maximum value of u16, it will be clamped to
    /// u16::MAX.
    pub const fn right(self) -> u16 {
        self.x.saturating_add(self.cols)
    }

    /// Returns the top coordinate of the gemo.
    pub const fn top(self) -> u16 {
        self.y
    }

    /// Returns the bottom coordinate of the gemo. This is the first coordinate outside of the gemo.
    ///
    /// If the bottom coordinate is larger than the maximum value of u16, it will be clamped to
    /// u16::MAX.
    pub const fn bottom(self) -> u16 {
        self.y.saturating_add(self.rows)
    }

    /// Returns a new gemo inside the current one, with the given margin on each side.
    ///
    /// If the margin is larger than the gemo, the returned gemo will have no area.
    pub fn inner(self, margin: &Margin) -> Geometry {
        let doubled_margin_horizontal = margin.horizontal.saturating_mul(2);
        let doubled_margin_vertical = margin.vertical.saturating_mul(2);

        if self.cols < doubled_margin_horizontal || self.rows < doubled_margin_vertical {
            Geometry::default()
        } else {
            Geometry {
                x: self.x.saturating_add(margin.horizontal),
                y: self.y.saturating_add(margin.vertical),
                cols: self.cols.saturating_sub(doubled_margin_horizontal),
                rows: self.rows.saturating_sub(doubled_margin_vertical),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let geometry = Geometry::new(10, 20);
        assert_eq!(geometry.x, 0);
        assert_eq!(geometry.y, 0);
        assert_eq!(geometry.rows, 10);
        assert_eq!(geometry.cols, 20);
    }

    #[test]
    fn test_area() {
        let geometry = Geometry::new(10, 20);
        assert_eq!(geometry.area(), 200);
    }

    #[test]
    fn test_is_empty() {
        let empty_geometry = Geometry::new(0, 10);
        assert!(empty_geometry.is_empty());

        let non_empty_geometry = Geometry::new(10, 20);
        assert!(!non_empty_geometry.is_empty());
    }

    #[test]
    fn test_left() {
        let geometry = Geometry::new(10, 20);
        assert_eq!(geometry.left(), 0);
    }

    #[test]
    fn test_right() {
        let geometry = Geometry::new(10, 20);
        assert_eq!(geometry.right(), 20);
    }

    #[test]
    fn test_top() {
        let geometry = Geometry::new(10, 20);
        assert_eq!(geometry.top(), 0);
    }

    #[test]
    fn test_bottom() {
        let geometry = Geometry::new(10, 20);
        assert_eq!(geometry.bottom(), 10);
    }

    #[test]
    fn test_inner() {
        let geometry = Geometry::new(10, 20);
        let margin = Margin {
            horizontal: 2,
            vertical: 2,
        };
        let inner_geometry = geometry.inner(&margin);
        assert_eq!(inner_geometry.x, 2);
        assert_eq!(inner_geometry.y, 2);
        assert_eq!(inner_geometry.rows, 6);
        assert_eq!(inner_geometry.cols, 16);
    }
}
