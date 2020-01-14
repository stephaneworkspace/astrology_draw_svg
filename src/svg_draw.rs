extern crate strum;
use crate::sweconst::Bodies;
//use strum::AsStaticRef;
use svg::node::element::path::Data;
use svg::node::element::path::Number;
//use svg::node::element::{Circle, Path, Symbol, Use};
use svg::node::element::{Circle, Path};
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
        if bodie == Bodies::Sun {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_to((7.0, 25.0))
                .elliptical_arc_by((18.0, 18.0, 0, 1, 1, 0, 0.1))
                .move_to((3.0, 0.0))
                .elliptical_arc_by((15.0, 15.0, 0, 1, 0, 0, -0.1))
                .move_to((11.0, 0.0))
                .elliptical_arc_by((4.0, 4.0, 0, 1, 0, 0, -0.1))
                .close();
            path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data);
        } else if bodie == Bodies::Moon {
            size = (50.0, 50.0);
            let data = Data::new()
                .move_to((12.5, 3.5))
                .elliptical_arc_by((22.5, 22.5, 0, 0, 1, 0, 43))
                .elliptical_arc_by((22.5, 22.5, 0, 1, 0, 0, -43))
                .close();
            path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data);
        } else {
            size = (0.0, 0.0);
            path = Path::new();
        }
        let document = Document::new()
            .set("viewBox", (0, 0, size.0, size.1))
            .add(path);
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
