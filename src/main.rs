mod builder;
mod grid;

fn main() {
    let mut grid = grid::Grid::new(4, 4);
    builder::binary_tree(&mut grid);
    println!("{}", grid);
}
