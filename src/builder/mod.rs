use crate::grid::{Dir, Grid};
use rand::Rng;

#[allow(unused)]
pub fn binary_tree(grid: &mut Grid) {
    let mut rng = rand::thread_rng();

    for gridcell in &grid.cells {
        let mut neighbors = Vec::with_capacity(2);

        let loc = gridcell.borrow().loc;

        if let Some(n) = grid.neighbor_loc(loc, Dir::North) {
            neighbors.push(Dir::North);
        }

        if let Some(n) = grid.neighbor_loc(loc, Dir::East) {
            neighbors.push(Dir::East);
        }

        let len = neighbors.len();

        if len > 0 {
            let i = rng.gen_range(0..len);
            grid.link(loc, neighbors[i]);
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
                let loc = run[i].borrow().loc;
                grid.link(loc, Dir::North);
                run.clear();
            } else {
                let loc = gridcell.borrow().loc;
                grid.link(loc, Dir::East);
            }
        }
    }
}
