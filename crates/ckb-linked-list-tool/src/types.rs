//! Types.

/// A closure to parse a part of data from the full data.
pub type DataParseFunc<Full, Part, E> = fn(Full) -> Result<Part, E>;

/// An item of the linked list that contains current data and the next data.
pub struct ListItem<Field> {
    /// Current data.
    pub curr: Field,
    /// Next data.
    pub next: Field,
}

/// A summary of one or more than one continuous linked list items.
pub struct ListItemsSummary<Field> {
    /// Current data of the first item.
    pub start: Field,
    /// Next data of the last item.
    pub end: Field,
}

impl<Field> ListItem<Field> {
    /// Creates a new list item.
    pub fn new(curr: Field, next: Field) -> Self {
        Self { curr, next }
    }
}

impl<Field> ListItemsSummary<Field> {
    /// Creates a new summay of one or more than one continuous linked list items.
    pub fn new(start: Field, end: Field) -> Self {
        Self { start, end }
    }
}

impl<Field: PartialEq> ListItemsSummary<Field>
where
    Field: PartialEq,
{
    /// If current summary contains all items in the list.
    pub fn is_complete(&self) -> bool {
        self.start == self.end
    }
}

impl<Field: PartialOrd> ListItemsSummary<Field>
where
    Field: PartialOrd,
{
    /// Contains the last item which next data points back to the first item of
    /// the linked list.
    pub fn has_last(&self) -> bool {
        self.start >= self.end
    }
}

impl<Field: PartialEq> PartialEq for ListItemsSummary<Field> {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}
