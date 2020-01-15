extern crate strum;
use crate::sweconst::Bodies;
//use strum::AsStaticRef;
use svg::node::element::path::Data;
use svg::node::element::path::Number;
//use svg::node::element::{Circle, Path, Symbol, Use};
use svg::node::element::{Circle, Group, Path};
use svg::Document;

// Working Storage - CONST

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

// Working Storage - Struct
#[derive(Debug, Clone)]
pub struct WorkingStorage {
    pub max_size: Number,
}

pub struct WorkingStorageSvg {
    pub center: (Number, Number), // size max of a svg (the space left is u
                                  // transparent)
                                  // this is nececary for after that center the
                                  // svg to center and compute some math for
                                  // put the svg in x,y
}

#[derive(Debug, Clone)]
pub struct WorkingStorageDraw {
    ws: WorkingStorage,
}

// Interfaces

pub trait Draw {
    fn draw_base(&self) -> Document;
}

pub trait CalcDraw {
    fn get_radius_total(&self) -> Number;
    fn get_radius_circle(&self, occurs: usize) -> (Number, bool);
    fn get_center_equal(&self, max_size: Number) -> (Number, Number);
}

pub trait BodiesSvg {
    fn draw_bodie(&self, bodie: Bodies) -> Document;
}

// Methods - Constructors

impl WorkingStorage {
    pub fn new(max_size: Number) -> WorkingStorage {
        WorkingStorage { max_size: max_size }
    }
}

impl WorkingStorageSvg {
    pub fn new(center: (Number, Number)) -> WorkingStorageSvg {
        WorkingStorageSvg { center: center }
    }
}

impl WorkingStorageDraw {
    pub fn new(ws: WorkingStorage) -> WorkingStorageDraw {
        WorkingStorageDraw { ws: ws }
    }
}

// Methods

impl Draw for WorkingStorageDraw {
    fn draw_base(&self) -> Document {
        let calc_draw = self.ws.clone();
        let center: (Number, Number) = (
            calc_draw.get_radius_total() as Number,
            calc_draw.get_radius_total() as Number,
        );
        let ws_svg = WorkingStorageSvg::new(center);

        let mut circle = Vec::new();
        for (i, ele) in CIRCLE_SIZE.iter().enumerate() {
            if ele.0 {
                circle.push(
                    Circle::new()
                        .set("fill", "none")
                        .set("cx", ws_svg.center.0)
                        .set("cy", ws_svg.center.1)
                        .set("r", calc_draw.get_radius_circle(i).0)
                        .set("stroke", "black")
                        .set("stroke-width", 1),
                );
            }
        }
        let document = Document::new()
            //.set("baseProfile", "full")
            //.set("version", "1.1")
            //.set("xmlns:xlink", "http://www.w3.org/1999/xlink")
            .set(
                "viewBox",
                (0, 0, self.ws.max_size as i32, self.ws.max_size as i32),
            )
            .add(circle[0].clone())
            .add(circle[1].clone())
            .add(circle[2].clone());
        document
    }
}

impl BodiesSvg for WorkingStorageSvg {
    fn draw_bodie(&self, bodie: Bodies) -> Document {
        let size: (Number, Number);
        let path: Path;
        let document: Document;
        if bodie == Bodies::Sun {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_by((7.0, 25.0)) // m
                .elliptical_arc_by((18.0, 18.0, 0, 1, 1, 0, 0.1)) // a
                .close() // z
                .move_by((3.0, 0.0)) // m
                .elliptical_arc_by((15.0, 15.0, 0, 1, 0, 0, -0.1)) // a
                .close() // z
                .move_by((11.0, 0.0)) // m
                .elliptical_arc_by((4.0, 4.0, 0, 1, 0, 0, -0.1)) // a
                .close(); // z
            path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        } else if bodie == Bodies::Moon {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_to((12.5, 3.5)) // M
                .elliptical_arc_by((22.5, 22.5, 0, 0, 1, 0, 43)) // a
                .elliptical_arc_by((22.5, 22.5, 0, 1, 0, 0, -43)) // a
                .close(); // z
            path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        } else if bodie == Bodies::Mercury {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((112.0, 36.5)) // M
                .elliptical_arc_to((11.5, 11.5, 0, 1, 1, 89, 36.5)) // A
                .elliptical_arc_to((11.5, 11.5, 0, 1, 1, 112, 36.5)) // A
                .close(); // z
            let data2 = Data::new()
                .move_to((111.9469, 37.603862)) // M
                .elliptical_arc_to((11.5, 11.5, 0, 0, 1, 89.052015, 37.592533));
            let data3 = Data::new()
                .move_to((373.83706, 512.99267)) // M
                .line_to((373.83706, 524.99267)); // L
            let data4 = Data::new()
                .move_to((368.83706, 519.99267)) // M
                .line_to((378.83706, 519.99267)); // L
            let path1 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data1)
                .set("transform", "matrix(0.96,0,0,0.96,277.357,466.9525)");
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data2)
                .set(
                    "transform",
                    "matrix(0.810715,0,0,0.810715,292.4483,451.9429)",
                );
            let path3 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data3);
            let path4 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data4);
            let group = Group::new()
                .set("transform", "translate(-348.7552,-478.0905)")
                .add(path1)
                .add(path2)
                .add(path3)
                .add(path4);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        } else if bodie == Bodies::Venus {
            size = (75.0, 75.0);
            let data = Data::new()
                .move_to((47.0, 59.0)) // M
                .horizontal_line_to(28.0) // H
                .move_by((9.5, 10.0)) // m
                .vertical_line_to(46.2) // v
                .elliptical_arc_by((18.3, 18.3, 0, 1, 1, 0.1, 0)); // a
            path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 5)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        } else if bodie == Bodies::Mars {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_by((30.0, 21.0)) // m
                .elliptical_arc_by((12.2, 12.2, 0, 1, 0, 2, 2))
                .close()
                .line_by((1, 1, 11, -11)) // l
                .move_by((-9, 0)) // m
                .horizontal_line_by(9) // h
                .vertical_line_by(9); // v
            path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3.3)
                .set("d", data);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        } else if bodie == Bodies::Jupiter {
            size = (50.0, 50.0);
            let data1 = Data::new()
                .move_to((382.83736, 486.87888))
                .line_to((382.83736, 519.93338));
            let data2 = Data::new()
                .move_to((388.2865, 511.54787))
                .line_to((361.949, 511.45787));
            let data3 = Data::new()
                .move_to((364.67357, 498.7446))
                .smooth_cubic_curve_to((
                    363.76538, 361.949, 497.89705, 361.949, 494.50684,
                )) // C
                .smooth_cubic_curve_to((
                    361.949, 491.11663, 365.58176, 487.72643, 369.21452,
                    487.72643,
                )) // C
                .smooth_cubic_curve_to((
                    372.84728, 487.72643, 376.48003, 490.26908, 376.48003,
                    496.20194,
                )) // C
                .smooth_cubic_curve_to((
                    376.48003, 502.1348, 371.93909, 511.45787, 362.85719,
                    511.45787,
                )); //C
            let path1 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data1);
            let path2 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data2);
            let path3 = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data3);
            let group = Group::new()
                .set("transform", "translate(-348.7552,-478.0905)")
                .add(path1)
                .add(path2)
                .add(path3);
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(group);
        } else {
            size = (0.0, 0.0);
            path = Path::new();
            document = Document::new()
                .set("viewBox", (0, 0, size.0, size.1))
                .add(path);
        }
        document
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
    fn get_center_equal(&self, max_size: Number) -> (Number, Number) {
        let result = max_size / 2.0;
        (result, result)
    }
}
