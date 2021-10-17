use crate::grid::Grid;
use rand::Rng;
use std::rc::Weak;

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
