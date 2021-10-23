use std::collections::HashMap;

use crate::grid::{Grid, Loc};

pub struct Distance {
    cells: HashMap<Loc, u32>,
}

impl Distance {
    pub fn from(grid: &Grid, root: Loc) -> Self {
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

        Self { cells }
    }

    pub fn get(&self, loc: Loc) -> u32 {
        *self.cells.get(&loc).unwrap()
    }
}
