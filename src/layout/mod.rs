use cassowary::{
    strength::{MEDIUM, REQUIRED, STRONG, WEAK},
    AddConstraintError, Expression, Solver, Variable,
    WeightedRelation::{EQ, GE, LE},
};
use lru::LruCache;
use std::{cell::RefCell, collections::HashMap, fmt, num::NonZeroUsize, rc::Rc, sync::OnceLock};
use strum::{Display, EnumString};

#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    Horizontal,
    #[default]
    Vertical,
}

pub mod gemo;
pub use gemo::Geometry;

/// Constraints to apply
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Constraint {
    /// Apply a percentage to a given amount
    ///
    /// Converts the given percentage to a f32, and then converts it back, trimming off the decimal
    /// point (effectively rounding down)
    /// ```
    /// # use zellij_widgets::prelude::*;
    /// assert_eq!(0, Constraint::Percentage(50).apply(0));
    /// assert_eq!(2, Constraint::Percentage(50).apply(4));
    /// assert_eq!(5, Constraint::Percentage(50).apply(10));
    /// assert_eq!(5, Constraint::Percentage(50).apply(11));
    /// ```
    Percentage(u16),
    /// Apply a ratio
    ///
    /// Converts the given numbers to a f32, and then converts it back, trimming off the decimal
    /// point (effectively rounding down)
    /// ```
    /// # use zellij_widgets::prelude::*;
    /// assert_eq!(0, Constraint::Ratio(4, 3).apply(0));
    /// assert_eq!(4, Constraint::Ratio(4, 3).apply(4));
    /// assert_eq!(10, Constraint::Ratio(4, 3).apply(10));
    /// assert_eq!(100, Constraint::Ratio(4, 3).apply(100));
    ///
    /// assert_eq!(0, Constraint::Ratio(3, 4).apply(0));
    /// assert_eq!(3, Constraint::Ratio(3, 4).apply(4));
    /// assert_eq!(7, Constraint::Ratio(3, 4).apply(10));
    /// assert_eq!(75, Constraint::Ratio(3, 4).apply(100));
    /// ```
    Ratio(u32, u32),
    /// Apply no more than the given amount (currently roughly equal to [Constraint::Max], but less
    /// consistent)
    /// ```
    /// # use zellij_widgets::prelude::*;
    /// assert_eq!(0, Constraint::Length(4).apply(0));
    /// assert_eq!(4, Constraint::Length(4).apply(4));
    /// assert_eq!(4, Constraint::Length(4).apply(10));
    /// ```
    Length(u16),
    /// Apply at most the given amount
    ///
    /// also see [std::cmp::min]
    /// ```
    /// # use zellij_widgets::prelude::*;
    /// assert_eq!(0, Constraint::Max(4).apply(0));
    /// assert_eq!(4, Constraint::Max(4).apply(4));
    /// assert_eq!(4, Constraint::Max(4).apply(10));
    /// ```
    Max(u16),
    /// Apply at least the given amount
    ///
    /// also see [std::cmp::max]
    /// ```
    /// # use zellij_widgets::prelude::*;
    /// assert_eq!(4, Constraint::Min(4).apply(0));
    /// assert_eq!(4, Constraint::Min(4).apply(4));
    /// assert_eq!(10, Constraint::Min(4).apply(10));
    /// ```
    Min(u16),
}

impl Default for Constraint {
    fn default() -> Self {
        Constraint::Percentage(100)
    }
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constraint::Percentage(p) => write!(f, "Percentage({})", p),
            Constraint::Ratio(n, d) => write!(f, "Ratio({}, {})", n, d),
            Constraint::Length(l) => write!(f, "Length({})", l),
            Constraint::Max(m) => write!(f, "Max({})", m),
            Constraint::Min(m) => write!(f, "Min({})", m),
        }
    }
}

impl Constraint {
    pub fn apply(&self, length: u16) -> u16 {
        match *self {
            Constraint::Percentage(p) => {
                let p = p as f32 / 100.0;
                let length = length as f32;
                (p * length).min(length) as u16
            }
            Constraint::Ratio(numerator, denominator) => {
                // avoid division by zero by using 1 when denominator is 0
                // this results in 0/0 -> 0 and x/0 -> x for x != 0
                let percentage = numerator as f32 / denominator.max(1) as f32;
                let length = length as f32;
                (percentage * length).min(length) as u16
            }
            Constraint::Length(l) => length.min(l),
            Constraint::Max(m) => length.min(m),
            Constraint::Min(m) => length.max(m),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Margin {
    pub horizontal: u16,
    pub vertical: u16,
}

impl Margin {
    pub const fn new(horizontal: u16, vertical: u16) -> Margin {
        Margin {
            horizontal,
            vertical,
        }
    }
}

impl fmt::Display for Margin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.horizontal, self.vertical)
    }
}

#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
}

type Cache = LruCache<(Geometry, Layout), Rc<[Geometry]>>;
thread_local! {
    static LAYOUT_CACHE: OnceLock<RefCell<Cache>> = OnceLock::new();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Layout {
    direction: Direction,
    margin: Margin,
    constraints: Vec<Constraint>,
}

impl Default for Layout {
    fn default() -> Layout {
        Layout::new(Direction::Vertical, [])
    }
}

impl Layout {
    pub const DEFAULT_CACHE_SIZE: usize = 16;
    /// Creates a new layout with default values.
    ///
    /// - margin: 0, 0
    /// - segment_size: SegmentSize::LastTakesRemainder
    pub fn new<C: AsRef<[Constraint]>>(direction: Direction, constraints: C) -> Layout {
        Layout {
            direction,
            margin: Margin::new(0, 0),
            constraints: constraints.as_ref().to_vec(),
        }
    }

    /// Initialize an empty cache with a custom size. The cache is keyed on the layout and area, so
    /// that subsequent calls with the same parameters are faster. The cache is a LruCache, and
    /// grows until `cache_size` is reached.
    ///
    /// Returns true if the cell's value was set by this call.
    /// Returns false if the cell's value was not set by this call, this means that another thread
    /// has set this value or that the cache size is already initialized.
    ///
    /// Note that a custom cache size will be set only if this function:
    /// * is called before [Layout::split()] otherwise, the cache size is
    ///   [`Self::DEFAULT_CACHE_SIZE`].
    /// * is called for the first time, subsequent calls do not modify the cache size.
    pub fn init_cache(cache_size: usize) -> bool {
        LAYOUT_CACHE
            .with(|c| {
                c.set(RefCell::new(LruCache::new(
                    NonZeroUsize::new(cache_size).unwrap(),
                )))
            })
            .is_ok()
    }

    /// Builder method to set the constraints of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zellij_widgets::prelude::*;
    /// let layout = Layout::default()
    ///     .constraints([
    ///         Constraint::Percentage(20),
    ///         Constraint::Ratio(1, 5),
    ///         Constraint::Length(2),
    ///         Constraint::Min(2),
    ///         Constraint::Max(2),
    ///     ])
    ///     .split(Geometry::new(10, 10));
    ///    assert_eq!(
    ///     layout[..],
    ///     [
    ///         Geometry {
    ///             x: 0,
    ///             y: 0,
    ///             cols: 10,
    ///             rows: 2,
    ///         },
    ///         Geometry {
    ///             x: 0,
    ///             y: 2,
    ///             cols: 10,
    ///             rows: 2,
    ///         },
    ///         Geometry {
    ///             x: 0,
    ///             y: 4,
    ///             cols: 10,
    ///             rows: 2,
    ///         },
    ///         Geometry {
    ///             x: 0,
    ///             y: 6,
    ///             cols: 10,
    ///             rows: 2,
    ///         },
    ///         Geometry {
    ///             x: 0,
    ///             y: 8,
    ///             cols: 10,
    ///             rows: 2,
    ///         },
    ///     ]
    /// );
    ///
    /// ```
    pub fn constraints<C: AsRef<[Constraint]>>(mut self, constraints: C) -> Layout {
        self.constraints = constraints.as_ref().to_vec();
        self
    }

    /// Builder method to set the margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zellij_widgets::prelude::*;
    /// let layout = Layout::default()
    ///     .constraints([Constraint::Min(0)])
    ///     .margin(2)
    ///     .split(Geometry::new(10, 10));
    /// assert_eq!(layout[..], [Geometry{x:2, y:2, cols:6, rows:0}]);
    /// ```
    pub const fn margin(mut self, margin: u16) -> Layout {
        self.margin = Margin {
            horizontal: margin,
            vertical: margin,
        };
        self
    }

    /// Builder method to set the horizontal margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zellij_widgets::prelude::*;
    /// let layout = Layout::default()
    ///     .constraints([Constraint::Min(0)])
    ///     .horizontal_margin(2)
    ///     .split(Geometry::new(10, 10));
    /// assert_eq!(layout[..], [Geometry{x:2, y:0, cols:6, rows:0}]);
    /// ```
    pub const fn horizontal_margin(mut self, horizontal: u16) -> Layout {
        self.margin.horizontal = horizontal;
        self
    }

    /// Builder method to set the vertical margin of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zellij_widgets::prelude::*;
    /// let layout = Layout::default()
    ///     .constraints([Constraint::Min(0)])
    ///     .vertical_margin(2)
    ///     .split(Geometry::new(10, 10));
    /// assert_eq!(layout[..], [Geometry{ x:0, y:2, cols:10, rows:0}]);
    /// ```
    pub const fn vertical_margin(mut self, vertical: u16) -> Layout {
        self.margin.vertical = vertical;
        self
    }

    /// Builder method to set the direction of the layout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zellij_widgets::prelude::*;
    /// let layout = Layout::default()
    ///     .direction(Direction::Horizontal)
    ///     .constraints([Constraint::Length(5), Constraint::Min(0)])
    ///     .split(Geometry::new(10, 10));
    /// assert_eq!(layout[..], [Geometry{ x:0, y:0, cols:5, rows:10}, Geometry{x:5, y:0, cols:0, rows:10}]);
    ///
    /// let layout = Layout::default()
    ///     .direction(Direction::Vertical)
    ///     .constraints([Constraint::Length(5), Constraint::Min(0)])
    ///     .split(Geometry::new(10, 10));
    /// assert_eq!(layout[..], [Geometry{x:0, y:0, cols:10, rows:5}, Geometry{x:0, y:5, cols:10, rows:0}]);
    /// ```
    pub const fn direction(mut self, direction: Direction) -> Layout {
        self.direction = direction;
        self
    }

    /// Wrapper function around the cassowary-rs solver to be able to split a given area into
    /// smaller ones based on the preferred widths or heights and the direction.
    ///
    /// This method stores the result of the computation in a thread-local cache keyed on the layout
    /// and area, so that subsequent calls with the same parameters are faster. The cache is a
    /// LruCache, and grows until [`Self::DEFAULT_CACHE_SIZE`] is reached by default, if the cache
    /// is initialized with the [Layout::init_cache()] grows until the initialized cache size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use zellij_widgets::prelude::*;
    ///
    /// let layout = Layout::default()
    ///     .direction(Direction::Horizontal)
    ///     .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)])
    ///     .split(Geometry::new(9, 2));
    /// assert_eq!(layout[..], [Geometry{x:0, y:0, cols:3, rows:2}, Geometry{x:3, y:0, cols:6, rows:2}]);
    /// ```
    pub fn split(&self, area: Geometry) -> Rc<[Geometry]> {
        LAYOUT_CACHE.with(|c| {
            c.get_or_init(|| {
                RefCell::new(LruCache::new(
                    NonZeroUsize::new(Self::DEFAULT_CACHE_SIZE).unwrap(),
                ))
            })
            .borrow_mut()
            .get_or_insert((area, self.clone()), || split(area, self))
            .clone()
        })
    }
}

/// A container used by the solver inside split
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Element {
    start: Variable,
    end: Variable,
}

impl Element {
    fn new() -> Element {
        Element {
            start: Variable::new(),
            end: Variable::new(),
        }
    }

    fn size(&self) -> Expression {
        self.end - self.start
    }
}

fn split(area: Geometry, layout: &Layout) -> Rc<[Geometry]> {
    try_split(area, layout).expect("failed to split")
}

fn try_split(area: Geometry, layout: &Layout) -> Result<Rc<[Geometry]>, AddConstraintError> {
    let mut solver = Solver::new();
    let inner = area.inner(&layout.margin);

    let (area_start, area_end) = match layout.direction {
        Direction::Horizontal => (f64::from(inner.x), f64::from(inner.right())),
        Direction::Vertical => (f64::from(inner.y), f64::from(inner.bottom())),
    };
    let area_size = area_end - area_start;

    // create an element for each constraint that needs to be applied. Each element defines the
    // variables that will be used to compute the layout.
    let elements = layout
        .constraints
        .iter()
        .map(|_| Element::new())
        .collect::<Vec<Element>>();

    // ensure that all the elements are inside the area
    for element in &elements {
        solver.add_constraints(&[
            element.start | GE(REQUIRED) | area_start,
            element.end | LE(REQUIRED) | area_end,
            element.start | LE(REQUIRED) | element.end,
        ])?;
    }
    // ensure there are no gaps between the elements
    for pair in elements.windows(2) {
        solver.add_constraint(pair[0].end | EQ(REQUIRED) | pair[1].start)?;
    }
    // ensure the first element touches the left/top edge of the area
    if let Some(first) = elements.first() {
        solver.add_constraint(first.start | EQ(REQUIRED) | area_start)?;
    }
    // apply the constraints
    for (&constraint, &element) in layout.constraints.iter().zip(elements.iter()) {
        match constraint {
            Constraint::Percentage(p) => {
                let percent = f64::from(p) / 100.00;
                solver.add_constraint(element.size() | EQ(STRONG) | (area_size * percent))?;
            }
            Constraint::Ratio(n, d) => {
                // avoid division by zero by using 1 when denominator is 0
                let ratio = f64::from(n) / f64::from(d.max(1));
                solver.add_constraint(element.size() | EQ(STRONG) | (area_size * ratio))?;
            }
            Constraint::Length(l) => {
                solver.add_constraint(element.size() | EQ(STRONG) | f64::from(l))?
            }
            Constraint::Max(m) => {
                solver.add_constraints(&[
                    element.size() | LE(STRONG) | f64::from(m),
                    element.size() | EQ(MEDIUM) | f64::from(m),
                ])?;
            }
            Constraint::Min(m) => {
                solver.add_constraints(&[
                    element.size() | GE(STRONG) | f64::from(m),
                    element.size() | EQ(MEDIUM) | f64::from(m),
                ])?;
            }
        }
    }

    let changes: HashMap<Variable, f64> = solver.fetch_changes().iter().copied().collect();

    // please leave this comment here as it's useful for debugging unit tests when we make any
    // changes to layout code - we should replace this with tracing in the future.
    // let ends = format!(
    //     "{:?}",
    //     elements
    //         .iter()
    //         .map(|e| changes.get(&e.end).unwrap_or(&0.0))
    //         .collect::<Vec<&f64>>()
    // );
    // dbg!(ends);

    // convert to Geometry
    let results = elements
        .iter()
        .map(|element| {
            let start = changes.get(&element.start).unwrap_or(&0.0).round() as u16;
            let end = changes.get(&element.end).unwrap_or(&0.0).round() as u16;
            let size = end - start;
            match layout.direction {
                Direction::Horizontal => Geometry {
                    x: start,
                    y: inner.y,
                    cols: size,
                    rows: inner.rows,
                },
                Direction::Vertical => Geometry {
                    x: inner.x,
                    y: start,
                    cols: inner.cols,
                    rows: size,
                },
            }
        })
        .collect::<Rc<[Geometry]>>();
    Ok(results)
}
