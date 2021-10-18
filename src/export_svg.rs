use svg::node::element::{Line, SVG};
use svg::Document;

use crate::grid::Grid;

fn line<U>(x1: U, y1: U, x2: U, y2: U, style: &str) -> Line
where
    U: Into<svg::node::Value>,
{
    Line::new()
        .set("x1", x1)
        .set("y1", y1)
        .set("x2", x2)
        .set("y2", y2)
        .set("style", style)
}

pub fn to_svg(grid: Grid, cell_size: usize) -> Document {
    let img_width = cell_size * grid.width + 2;
    let img_height = cell_size * grid.height + 2;

    let mut svg = SVG::new().set("viewBox", (0, 0, img_width, img_height));

    let wall = "stroke-width:1; stroke:black";

    for cell in &grid.cells {
        let cell = cell.borrow();

        let (row, col) = cell.loc;
        let x1 = col * cell_size + 1;
        let y1 = row * cell_size + 1;
        let x2 = (col + 1) * cell_size + 1;
        let y2 = (row + 1) * cell_size + 1;

        if cell.north.is_none() {
            svg = svg.add(line(x1, y1, x2, y1, wall));
        }

        if cell.west.is_none() {
            svg = svg.add(line(x1, y1, x1, y2, wall));
        }

        if cell.east.is_none() || !cell.linked(cell.east.as_ref().unwrap()) {
            svg = svg.add(line(x2, y1, x2, y2, wall));
        }

        if cell.south.is_none() || !cell.linked(cell.south.as_ref().unwrap()) {
            svg = svg.add(line(x1, y2, x2, y2, wall));
        }
    }

    svg
    // Document::new()
    //     .set("width", "95%")
    //     .set("height", "95%")
    //     .add(svg)
}
