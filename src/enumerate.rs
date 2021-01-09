pub trait IntoGridEnumerate {
    type InnerIter;

    fn into_grid_enumerate(self, cols: usize) -> GridEnumerate<Self::InnerIter>;
}

// Blanket impl
impl<I> IntoGridEnumerate for I
     where I: Iterator
{
    type InnerIter = I;

    fn into_grid_enumerate(self, cols: usize) -> GridEnumerate<I> {
        GridEnumerate::new(self, cols)
    }
}

/// An iterator similar to `std::iter::Enumerate`,
/// except it yields grid positions as (row, column) (assuming row-major order),
/// instead of current counts as integers.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct GridEnumerate<I> {
    inner_iter:  I,
    columns:     usize,
    current_row: usize,
    current_col: usize
}

impl<I> Iterator for GridEnumerate<I>
    where I: Iterator
{
    type Item = ((usize, usize), <I as Iterator>::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let inner_item = self.inner_iter.next()?;

        let row = self.current_row;
        let col = self.current_col;

        self.current_col = (col + 1) % self.columns;

        if self.current_col == 0 {
            self.current_row += 1
        };

        Some(((row, col), inner_item))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner_iter.size_hint()
    }
}

impl<I> ExactSizeIterator for GridEnumerate<I>
    where I: ExactSizeIterator
{}

impl<I> GridEnumerate<I> {
    pub fn new(inner_iter: I, columns: usize) -> Self {
        Self{
            inner_iter,
            columns,
            current_row: 0,
            current_col: 0
        }
    }
}
