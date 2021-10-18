mod builder;
mod export_svg;
mod grid;

use svg;

fn main() {
    let mut grid = grid::Grid::new(20, 20);
    // builder::binary_tree(&mut grid);
    builder::sidewinder(&mut grid);

    println!("{}", grid);

    let doc = export_svg::to_svg(grid, 10);

    svg::save("image.svg", &doc).unwrap();
}
