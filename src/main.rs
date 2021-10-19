mod builder;
mod distance;
mod export_svg;
mod grid;

use svg;

fn main() {
    let mut grid = grid::Grid::new(20, 20);
    // builder::binary_tree(&mut grid);
    builder::sidewinder(&mut grid);

    let distance = distance::Distance::new(grid, (19, 0));

    println!("{}", distance);

    // let doc = export_svg::to_svg(grid, 10);

    // svg::save("image.svg", &doc).unwrap();
}
