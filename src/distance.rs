use std::{collections::HashMap, fmt::Display};

use crate::grid::{Grid, Loc};

pub struct Distance {
    grid: Grid,
    root: Loc,
    cells: HashMap<Loc, u32>,
}

impl Distance {
    pub fn new(grid: Grid, root: Loc) -> Self {
        let mut cells = HashMap::new();
        cells.insert(root, 0);

        let mut frontier = vec![root];
        let mut new_frontier = vec![];

        while !frontier.is_empty() {
            while let Some(loc) = frontier.pop() {
                let cell = grid.cell(loc).borrow();
                let dist = *cells.get(&loc).unwrap();
                for linked in cell.links.iter() {
                    let linked = linked.upgrade().unwrap();
                    let linked_loc = (*linked).borrow().loc;
                    if !cells.contains_key(&linked_loc) {
                        cells.insert(linked_loc, dist + 1);
                        new_frontier.push(linked_loc);
                    }
                }
            }

            frontier.append(&mut new_frontier);
        }

        Self { grid, root, cells }
    }

    fn get(&self, loc: Loc) -> u32 {
        *self.cells.get(&loc).unwrap()
    }

    // fn set(&mut self, loc: Loc, dist: u32) {
    //     self.cells.insert(loc, dist);
    // }

    // fn cells(&self) -> std::collections::hash_map::Keys<Loc, u32> {
    //     self.cells.keys()
    // }
}

impl Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        output.push('+');
        for _ in 0..self.grid.width {
            output.push_str("---+");
        }
        output.push('\n');

        let mut top = String::new();
        let mut bottom = String::new();

        for row in 0..self.grid.height {
            top.push('|');
            bottom.push('+');

            for col in 0..self.grid.width {
                let cell = self.grid.cells[row * self.grid.width + col].borrow();

                let dist = format!("{:^3}", self.get((row, col)));
                top.push_str(dist.as_str());

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
