use crate::grid::{Grid, GridCell};
use rand::Rng;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub fn binary_tree(grid: &mut Grid) {
    let mut rng = rand::thread_rng();

    for gridcell in &grid.cells {
        let mut neighbors = Vec::with_capacity(2);

        let mut cell = gridcell.borrow_mut();

        if let Some(ref c) = cell.north {
            neighbors.push(c);
        }

        if let Some(ref c) = cell.east {
            neighbors.push(c);
        }

        let len = neighbors.len();

        if len > 0 {
            let i = rng.gen_range(0..len);
            let other = Weak::upgrade(&neighbors[i]).unwrap();

            cell.link(&other);
            other.borrow_mut().link(gridcell);
        }
    }
}

pub fn sidewinder(grid: &mut Grid) {
    let mut rng = rand::thread_rng();

    for row in grid.rows() {
        let mut run = Vec::new();

        for gridcell in row {
            run.push(gridcell);

            let should_close_out = {
                let cell = gridcell.borrow();
                let at_boundary_east = cell.east.is_none();
                let at_boundary_north = cell.north.is_none();
                at_boundary_east || (!at_boundary_north && rng.gen())
            };

            if should_close_out {
                let i: usize = rng.gen_range(0..run.len());
                let mut cell = run[i].borrow_mut();
                if let Some(ref north) = cell.north {
                    let north = Weak::upgrade(north).unwrap();
                    cell.link(&north);
                    north.borrow_mut().link(run[i]);
                }
                run.clear();
            } else {
                let mut cell = gridcell.borrow_mut();
                if let Some(ref east) = cell.east {
                    let east = Weak::upgrade(east).unwrap();
                    cell.link(&east);
                    east.borrow_mut().link(gridcell);
                }
            }
        }
    }
}
