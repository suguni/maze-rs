mod builder;
mod distance;
mod export_svg;
mod grid;

use svg;

fn main() {
    let mut grid = grid::Grid::new(20, 20);
    // builder::binary_tree(&mut grid);
    builder::sidewinder(&mut grid);

    let distance = distance::Distance::from(&grid, (19, 0));

    render(&grid, &distance);

    // let doc = export_svg::to_svg(grid, 10);

    // svg::save("image.svg", &doc).unwrap();
}

fn render(grid: &grid::Grid, distance: &distance::Distance) {
    use radix_fmt::radix;

    let mut output = String::new();

    output.push('+');
    for _ in 0..grid.width {
        output.push_str("---+");
    }
    output.push('\n');

    let mut top = String::new();
    let mut bottom = String::new();

    for row in 0..grid.height {
        top.push('|');
        bottom.push('+');

        for col in 0..grid.width {
            let cell = grid.cells[row * grid.width + col].borrow();

            let dist = format!("{:^3}", radix(distance.get((row, col)), 36).to_string());
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

    println!("{}", output)
}
