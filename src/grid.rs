use std::cell::RefCell;
use std::fmt::Display;
use std::rc::{Rc, Weak};
use std::slice::Chunks;

type Loc = (usize, usize);

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

pub struct GridCell {
    #[allow(unused)]
    row: usize,
    #[allow(unused)]
    column: usize,
    pub loc: Loc,
    pub north: Option<Weak<RefCell<GridCell>>>,
    pub south: Option<Weak<RefCell<GridCell>>>,
    pub east: Option<Weak<RefCell<GridCell>>>,
    pub west: Option<Weak<RefCell<GridCell>>>,
    links: Vec<Weak<RefCell<GridCell>>>,
}

impl GridCell {
    pub fn new(row: usize, column: usize) -> Self {
        GridCell {
            row,
            column,
            loc: (row, column),
            north: None,
            south: None,
            east: None,
            west: None,
            links: Vec::with_capacity(4),
        }
    }

    pub fn link(&mut self, cell: &Rc<RefCell<GridCell>>) {
        self.links.push(Rc::downgrade(&cell));
    }

    pub fn linked(&self, cell: &Weak<RefCell<GridCell>>) -> bool {
        self.links
            .iter()
            .any(|c| std::ptr::eq(c.as_ptr(), cell.as_ptr()))
    }

    pub fn neighbor(&self, dir: Dir) -> &Option<Weak<RefCell<GridCell>>> {
        match dir {
            Dir::North => &self.north,
            Dir::East => &self.east,
            Dir::South => &self.south,
            Dir::West => &self.west,
        }
    }
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Rc<RefCell<GridCell>>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells: Vec<Rc<RefCell<GridCell>>> = (0..height)
            .flat_map(|r| (0..width).map(move |c| (r, c)))
            .map(|(row, column)| Rc::new(RefCell::new(GridCell::new(row, column))))
            .collect();

        for (i, cell) in cells.iter().enumerate() {
            let mut cell = cell.borrow_mut();
            let r = i / width;
            let c = i % width;

            if r > 0 {
                let idx = (r - 1) * width + c;
                cell.north = Some(Rc::downgrade(&cells[idx]));
            }
            if r + 1 < height {
                let idx = (r + 1) * width + c;
                cell.south = Some(Rc::downgrade(&cells[idx]));
            }
            if c > 0 {
                let idx = r * width + (c - 1);
                cell.west = Some(Rc::downgrade(&cells[idx]));
            }
            if c + 1 < width {
                let idx = r * width + (c + 1);
                cell.east = Some(Rc::downgrade(&cells[idx]));
            }
        }

        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn neighbor_loc(&self, (r, c): Loc, dir: Dir) -> Option<Loc> {
        match dir {
            Dir::North => {
                if r > 0 {
                    Some((r - 1, c))
                } else {
                    None
                }
            }
            Dir::East => {
                if c + 1 < self.width {
                    Some((r, c + 1))
                } else {
                    None
                }
            }
            Dir::South => {
                if r + 1 < self.height {
                    Some((r + 1, c))
                } else {
                    None
                }
            }
            Dir::West => {
                if c > 0 {
                    Some((r, c - 1))
                } else {
                    None
                }
            }
        }
    }

    pub fn rows(&self) -> Chunks<Rc<RefCell<GridCell>>> {
        self.cells.chunks(self.width)
    }

    pub fn link(&self, loc1: Loc, dir: Dir) {
        if let Some(loc2) = self.neighbor_loc(loc1, dir) {
            let c1 = &self.cells[loc1.0 * self.width + loc1.1];
            let c2 = &self.cells[loc2.0 * self.width + loc2.1];
            c1.borrow_mut().link(&c2);
            c2.borrow_mut().link(&c1);
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        output.push('+');
        for _ in 0..self.width {
            output.push_str("---+");
        }
        output.push('\n');

        let mut top = String::new();
        let mut bottom = String::new();

        for row in 0..self.height {
            top.push('|');
            bottom.push('+');

            for col in 0..self.width {
                let cell = self.cells[row * self.width + col].borrow();

                top.push_str("   ");
                if cell.east.is_some() && cell.linked(cell.east.as_ref().unwrap()) {
                    top.push(' ');
                } else {
                    top.push('|');
                }

                if cell.south.is_some() && cell.linked(cell.south.as_ref().unwrap()) {
                    bottom.push_str("   ");
                } else {
                    bottom.push_str("---");
                }
                bottom.push('+');
            }

            output.push_str(&top);
            output.push('\n');

            output.push_str(&bottom);
            output.push('\n');

            top.clear();
            bottom.clear();
        }

        write!(f, "{}", output)
    }
}

/*
impl<'a> IntoIterator for &'a Grid {
    type Item = &'a GridCell;
    type IntoIter = std::slice::Iter<'a, GridCell>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter()
    }
}

impl<'a> IntoIterator for &'a mut Grid {
    type Item = &'a mut GridCell;
    type IntoIter = std::slice::IterMut<'a, GridCell>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter_mut()
    }
}
*/

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    // fn test_iter() {
    //     let grid = Grid::new(2, 2);
    //     assert_eq!(grid.into_iter().count(), 4);

    //     let mut count = 0;
    //     for _cell in &grid {
    //         count += 1;
    //     }
    //     assert_eq!(count, 4);

    //     assert_eq!(grid.cells.len(), 4);
    // }

    // #[test]
    // fn test_print() {
    //     let grid = Grid::new(3, 2);
    //     for cell in &grid {
    //         println!("{:?}", cell);
    //     }
    // }

    // #[test]
    // fn test_mut_iter() {
    //     let mut grid = Grid::new(2, 2);
    //     let mut c = 0;
    //     for cell in &mut grid {
    //         cell.id = c;
    //         c += 1;
    //     }

    //     c = 0;
    //     for cell in &grid {
    //         assert_eq!(cell.id, c);
    //         c += 1;
    //     }
    // }
}
