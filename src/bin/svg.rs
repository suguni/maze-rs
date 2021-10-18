use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::node::element::{Circle, Path, Polyline, Rectangle, SVG};
use svg::parser::Event;
use svg::Document;

fn main() {
    // let data = Data::new()
    //     .move_to((10, 10))
    //     .line_by((0, 50))
    //     .line_by((50, 0))
    //     .line_by((0, -50))
    //     .close();

    // let path = Path::new()
    //     .set("fill", "none")
    //     .set("stroke", "black")
    //     .set("stroke-width", 3)
    //     .set("d", data);

    // let body = Rectangle::new()
    //     .set("x", 10)
    //     .set("y", 35)
    //     .set("width", 40)
    //     .set("height", 40)
    //     .set("style", "stroke:black; fill:none;");

    // let roof = Polyline::new()
    //     .set("points", "10 35, 30 7.68, 50 35")
    //     .set("style", "stroke:black; fill:none");

    // let door = Polyline::new()
    //     .set("points", "30 75, 30 55, 40 55, 40 75")
    //     .set("style", "stroke:black; fill:none");

    let circle = Circle::new()
        .set("cx", 25)
        .set("cy", 25)
        .set("r", 25)
        .set("style", "stroke:black; fill:none");

    let rect = Rectangle::new()
        .set("x", 100)
        .set("y", 5)
        .set("width", 30)
        .set("height", 80)
        .set("style", "stroke:blue; fill:none");

    let c2 = Circle::new()
        .set("cx", 25)
        .set("cy", 25)
        .set("r", 25)
        .set("style", "stroke:black; fill:none");

    let svg = SVG::new()
        .set("x", 100)
        .set("y", 5)
        .set("width", 30)
        .set("height", 80)
        .set("viewBox", "0 0 50 50")
        .set("preserveAspectRatio", "xMaxYMax meet")
        .add(c2);

    let document = Document::new()
        .set("width", "200")
        .set("height", "200")
        .set("viewBox", (0, 0, 200, 200))
        .add(circle)
        .add(rect)
        .add(svg);

    svg::save("image.svg", &document).unwrap();

    /*
        let path = "image.svg";
        let mut content = String::new();
        for event in svg::open(path, &mut content).unwrap() {
            match event {
                Event::Tag(Path, _, attributes) => {
                    let data = attributes.get("d").unwrap();
                    let data = Data::parse(data).unwrap();
                    for command in data.iter() {
                        match command {
                            &Command::Move(..) => println!("Move!"),
                            &Command::Line(..) => println!("Line!"),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    */
}
