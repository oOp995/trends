use std::fmt::{Debug};

/// Extension trait for converting two ordered values into a [`Trend`].
///
/// # Examples
///
/// ```
/// use trends::TrendExt;
///
/// let trend = 10.to_trend(&20);
///
/// assert!(trend.is_rising());
/// assert_eq!(trend.direction(), 1);
/// ```
pub trait TrendExt: Sized {
    /// Returns the trend between `self` and `end`.
    ///
    /// - [`Trend::Rising`] if `end > self`
    /// - [`Trend::Falling`] if `end < self`
    /// - [`Trend::Stable`] if `end == self`
    fn to_trend<'a>(&'a self, end: &'a Self) -> Trend<'a, Self>;
}

impl<T> TrendExt for T
where
    T: Ord,
{
    /// Converts two ordered values into a [`Trend`].
    ///
    /// # Examples
    ///
    /// ```
    /// use trends::TrendExt;
    ///
    /// assert!(1.to_trend(&2).is_rising());
    /// assert!(2.to_trend(&1).is_falling());
    /// assert!(1.to_trend(&1).is_stable());
    /// ```
    fn to_trend<'a>(&'a self, end: &'a Self) -> Trend<'a, Self> {
        use std::cmp::Ordering;

        match self.cmp(end) {
            Ordering::Greater => Trend::Falling { start: self, end },
            Ordering::Less => Trend::Rising { start: self, end },
            Ordering::Equal => Trend::Stable { start: self, end },
        }
    }
}
/// Represents the directional relationship between two values.
///
/// A trend compares a starting value with an ending value and classifies it as:
///
/// - [`Trend::Rising`] — the value increased
/// - [`Trend::Falling`] — the value decreased
/// - [`Trend::Stable`] — the value did not change
///
/// # Examples
///
/// ```
/// use trends::{Trend, TrendExt};
///
/// let trend = 5.to_trend(&8);
///
/// assert_eq!(trend, Trend::Rising { start: &5, end: &8 });
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Trend<'a, T> {
    /// Indicates the value increased from `start` to `end`.
    Rising { start: &'a T, end: &'a T },

    /// Indicates the value decreased from `start` to `end`.
    Falling { start: &'a T, end: &'a T },

    /// Indicates the value remained unchanged from `start` to `end`.
    Stable { start: &'a T, end: &'a T },
}

impl<'a, T> Trend<'a, T> {
    /// Returns `true` if the trend is [`Trend::Rising`].
    pub fn is_rising(&self) -> bool {
        matches!(self, Self::Rising { .. })
    }

    /// Returns `true` if the trend is [`Trend::Falling`].
    pub fn is_falling(&self) -> bool {
        matches!(self, Self::Falling { .. })
    }

    /// Returns `true` if the trend is [`Trend::Stable`].
    pub fn is_stable(&self) -> bool {
        matches!(self, Self::Stable { .. })
    }

    /// Returns a reference to the starting value.
    ///
    /// # Examples
    ///
    /// ```
    /// use trends::TrendExt;
    ///
    /// let trend = 3.to_trend(&7);
    ///
    /// assert_eq!(trend.start(), &3);
    /// ```
    pub fn start(&self) -> &'a T {
        match self {
            Self::Rising { start, .. }
            | Self::Falling { start, .. }
            | Self::Stable { start, .. } => *start,
        }
    }

    /// Returns a reference to the ending value.
    ///
    /// # Examples
    ///
    /// ```
    /// use trends::TrendExt;
    ///
    /// let trend = 3.to_trend(&7);
    ///
    /// assert_eq!(trend.end(), &7);
    /// ```
    pub fn end(&self) -> &'a T {
        match self {
            Self::Rising { end, .. } | Self::Falling { end, .. } | Self::Stable { end, .. } => *end,
        }
    }

    /// Returns a cloned copy of the starting value.
    ///
    /// Requires `T: Clone`.
    pub fn clone_start(&self) -> T
    where
        T: Clone,
    {
        match self {
            Self::Rising { start, .. }
            | Self::Falling { start, .. }
            | Self::Stable { start, .. } => (*start).clone(),
        }
    }

    /// Returns a cloned copy of the ending value.
    ///
    /// Requires `T: Clone`.
    pub fn clone_end(&self) -> T
    where
        T: Clone,
    {
        match self {
            Self::Rising { end, .. } | Self::Falling { end, .. } | Self::Stable { end, .. } => {
                (*end).clone()
            }
        }
    }

    /// Returns the trend direction as a signed integer `sign of change`.
    ///
    /// Returns:
    ///
    /// - `1` for [`Trend::Rising`]
    /// - `0` for [`Trend::Stable`]
    /// - `-1` for [`Trend::Falling`]
    ///
    /// Useful when numeric direction is preferred over pattern matching.
    pub fn direction(&self) -> i8 {
        if self.is_stable() {
            0
        } else if self.is_rising() {
            1
        } else {
            -1
        }
    }
    /// Returns a reference to the 'min' point in the trend
    pub fn min_point(&self) -> &'a T {
        match self.direction() {
            1 | 0 => self.start(),
            -1 => self.end(),
            _ => {
                //unreachable block
                //read documentation of `returns::return_shit!()`
                //it will cause panic if reached in release mode
                //but it is impossible to reach here
                //read documentation of method `Trend::direction()`
                returns::return_shit!()
            }
        }
    }

    /// Returns a reference to the 'max' point in the trend
    pub fn max_point(&self) -> &'a T {
        match self.direction() {
            1 | 0 => self.end(),
            -1 => self.start(),
            _ => {
                //unreachable block
                //read documentation of `returns::return_shit!()`
                //it will cause panic if reached in release mode
                //but it is impossible to reach here
                //read documentation of method `Trend::direction()`
                returns::return_shit!()
            }
        }
    }

    /// Returns a cloned Copy `as Clone` to the 'min' point in the trend
    ///
    /// Requires `T:Clone`
    pub fn min_point_cloned(&self) -> T
    where
        T: Clone,
    {
        match self.direction() {
            1 | 0 => self.start().clone(),
            -1 => self.end().clone(),
            _ => {
                //unreachable block
                //read documentation of `returns::return_shit!()`
                //it will cause panic if reached in release mode
                //but it is impossible to reach here
                //read documentation of method `Trend::direction()`
                returns::return_shit!()
            }
        }
    }

    /// Returns a cloned Copy `as Clone` to the 'max' point in the trend
    ///
    /// Requires `T:Clone`
    pub fn max_point_cloned(&self) -> T
    where
        T: Clone,
    {
        match self.direction() {
            1 | 0 => self.end().clone(),
            -1 => self.start().clone(),
            _ => {
                //unreachable block
                //read documentation of `returns::return_shit!()`
                //it will cause panic if reached in release mode
                //but it is impossible to reach here
                //read documentation of method `Trend::direction()`
                returns::return_shit!()
            }
        }
    }

    //TODO:Fix for release version
    #[allow(unused)]
    fn sort_group(trends: &mut Vec<Self>) {
        //sort each trend in the group
        for i in 0..trends.len() {
            trends[i].sort_unit();
        }

        //now units are sorted
        //we need to sort the units as groups
    }

    //TODO:Fix for release version
    #[allow(unused)]
    fn sort_unit(&mut self) {
        let min = self.min_point();
        let max = self.max_point();
        (*self) = Trend::Rising {
            start: min,
            end: max,
        }
    }
    
}
