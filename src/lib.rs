extern crate strum;
use svg::node::element::path::Data;
use svg::node::element::path::Number;
use svg::node::element::{Circle, Path, Symbol, Use};
use svg::Document;
mod sweconst;
use sweconst::Bodies;
//use strum::{AsStaticRef, IntoEnumIterator};
#[macro_use]
extern crate strum_macros;
use strum::AsStaticRef;

struct WorkingStorageSvg {
    center: (Number, Number), // size max of a svg (the space left is u
                              // transparent)
                              // this is nececary for after that center the svg
                              // to center and compute some math for put the
                              // svg in x,y
}

trait BodiesSvg {
    fn get_path(&self, bodie: Bodies) -> Path;
    fn get_variable(&self, bodie: Bodies, sw_link: bool) -> String;
}

impl WorkingStorageSvg {
    fn new(center: (Number, Number)) -> WorkingStorageSvg {
        WorkingStorageSvg { center: center }
    }
}

impl BodiesSvg for WorkingStorageSvg {
    fn get_path(&self, bodie: Bodies) -> Path {
        if bodie == Bodies::Moon {
            let data = Data::new()
                //.move_to((12.5, 3.5))
                .move_to(self.center)
                .elliptical_arc_by((22.5, 22.5, 0, 0, 1, 0, 43))
                .elliptical_arc_by((22.5, 22.5, 0, 1, 0, 0, -43))
                .close();
            Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data)
        } else {
            Path::new()
        }
    }
    fn get_variable(&self, bodie: Bodies, sw_link: bool) -> String {
        if sw_link {
            format!("#{}", bodie.clone().as_static().to_lowercase())
        } else {
            bodie.clone().as_static().to_lowercase()
        }
    }
}

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
    let center: (Number, Number) = (
        calc_draw.get_radius_total() as Number,
        calc_draw.get_radius_total() as Number,
    );

    /*
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
    );*/

    let data1 = Circle::new()
        .set("fill", "none")
        .set("cx", center.0)
        .set("cy", center.1)
        .set("r", calc_draw.get_radius_circle(0).0)
        .set("stroke", "black")
        .set("stroke-width", 3);

    let data2 = Circle::new()
        .set("fill", "none")
        .set("cx", center.0)
        .set("cy", center.1)
        .set("r", calc_draw.get_radius_circle(1).0)
        .set("stroke", "red")
        .set("stroke-width", 2);

    /*let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        //.set("d", circle[0].clone());
        .set("d", moon);
    */

    let ws_svg = WorkingStorageSvg::new(center);

    let moon_symbol = Symbol::new()
        .set("viewBox", (0, 0, max_size as i32, max_size as i32))
        .set("id", ws_svg.get_variable(Bodies::Moon, false))
        .add(ws_svg.get_path(Bodies::Moon));

    let moon_use = Use::new()
        .set("xlink:href", ws_svg.get_variable(Bodies::Moon, true))
        .set("width", 100)
        .set("height", 100)
        .set("x", center.0)
        .set("y", center.1);

    let document = Document::new()
        .set("viewBox", (0, 0, max_size as i32, max_size as i32))
        .add(data1)
        .add(data2)
        .add(moon_symbol)
        .add(moon_use);
    if path_export != "" {
        svg::save(format!("{}{}", path_export, "image.svg"), &document)
            .unwrap();
    }
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
