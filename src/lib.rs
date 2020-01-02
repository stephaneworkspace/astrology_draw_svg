//use svg::node::element::path::Command;
//use svg::node::element::path::Command::EllipticalArc;
use svg::node::element::path::Data;
use svg::node::element::path::{Number, Parameters}; //, Position};
use svg::node::element::{Circle, Path};
use svg::Document;

#[derive(Debug)]
struct WorkingStorage {
    max_size: Number,
}
// Const size in %
// tuple (visible/value)
const CIRCLE_SIZE: [(bool, Number); 7] = [
    (true, 35.0),  // 0
    (true, 62.0),  // 1
    (true, 67.0),  // 2
    (false, 75.0), // 3
    (false, 80.0), // 4
    (false, 87.0), // 5
    (false, 94.0), // 6
];

trait CalcDraw {
    fn get_radius_total(&self) -> Number;
    fn get_radius_circle(&self, occurs: usize) -> (Number, bool);
}

impl WorkingStorage {
    fn new(max_size: Number) -> WorkingStorage {
        WorkingStorage { max_size: max_size }
    }
}

impl CalcDraw for WorkingStorage {
    fn get_radius_total(&self) -> Number {
        self.max_size / 2.0
    }
    fn get_radius_circle(&self, occurs: usize) -> (Number, bool) {
        if occurs > CIRCLE_SIZE.len() {
            panic!("Out of range in circle occurs: {}", occurs);
        }
        (
            (self.get_radius_total() * CIRCLE_SIZE[occurs].1) / 100.0,
            CIRCLE_SIZE[occurs].0,
        )
    }
}

pub fn chart(max_size: Number, path_export: &str) -> String {
    let calc_draw = WorkingStorage::new(max_size);
    // Center circle
    // let center: (f64, f64) =
    //    (calc_draw.get_radius_total(), calc_draw.get_radius_total());
    let center: (i32, i32) = (
        calc_draw.get_radius_total() as i32,
        calc_draw.get_radius_total() as i32,
    );

    // Circle visible to draw
    let mut circle: Vec<Data> = Vec::new();
    let mut ell: Vec<Number> = Vec::new();
    ell.push(30.0);
    ell.push(20.0);
    //let ell_arc = EllipticalArc(Position::Absolute, Parameters::from(ell));
    circle.push(
        Data::new()
            .move_to(center)
            .elliptical_arc_by(Parameters::from(ell))
            /*.elliptical_arc_to(EllipticalArc(
                Position::Absolute,
                Parameters::from(ell),
            ))*/
            .close(),
    );

    let data1 = Circle::new()
        .set("fill", "none")
        .set("cx", 50.0)
        .set("cy", 50.0)
        .set("r", 50.0)
        .set("stroke", "black")
        .set("stroke-width", 3);

    let data2 = Circle::new()
        .set("fill", "none")
        .set("cx", 150.0)
        .set("cy", 150.0)
        .set("r", 350.0)
        .set("stroke", "red")
        .set("stroke-width", 2);

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", circle[0].clone());

    let document = Document::new()
        .set("viewBox", (0, 0, max_size as i32, max_size as i32))
        .add(data1)
        .add(data2);

    /*
    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data1);

    let document = Document::new()
        .set("viewBox", (0, 0, max_size as i32, max_size as i32))
        .add(path)
    */
    svg::save(format!("{}{}", path_export, "image.svg"), &document).unwrap();
    document.to_string()
}

pub fn write() -> String {
    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);

    let document = Document::new().set("viewBox", (0, 0, 70, 70)).add(path);

    //svg::save("image.svg", &document).unwrap();
    document.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
