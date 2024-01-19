//! Datatypes to support source location information.

use std::fmt;

/// A location somewhere in the sourcecode.
#[derive(Clone, Copy, Default, Eq)]
pub struct Location {
    row: usize,
    column: usize,
    previos_column: usize,
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && 
        self.column == other.column
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Location {{ row: {}, column: {} }}", self.row, self.column)
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {} column {}", self.row, self.column)
    }
}

impl Location {
    pub fn visualize<'a>(
        &self,
        line: &'a str,
        desc: impl fmt::Display + 'a,
    ) -> impl fmt::Display + 'a {
        struct Visualize<'a, D: fmt::Display> {
            loc: Location,
            line: &'a str,
            desc: D,
        }
        impl<D: fmt::Display> fmt::Display for Visualize<'_, D> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "{}\n{}{arrow:>pad$}",
                    self.desc,
                    self.line,
                    pad = self.loc.column,
                    arrow = "^",
                )
            }
        }
        Visualize {
            loc: *self,
            line,
            desc,
        }
    }
}

impl Location {
    pub fn new(row: usize, column: usize) -> Self {
        Location { 
            row, 
            column, 
            previos_column: column 
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn reset(&mut self) {
        self.row = 1;
        self.column = 1;
    }

    pub fn go_right(&mut self) {
        self.previos_column = self.column;
        self.column += 1;
    }

    pub fn go_left(&mut self) {
        self.column -= 1;
    }

    pub fn newline(&mut self) {
        self.previos_column = self.column;
        self.row += 1;
        self.column = 0;
    }


    pub fn move_back_newline(&mut self) {
        if self.row > 1 {
            self.row -= 1;
            self.column = self.previos_column;
        }
    }
}