extern crate strum;
use crate::sweconst::Bodies;
use strum::AsStaticRef;
use svg::node::element::path::Data;
use svg::node::element::path::Number;
use svg::node::element::{Circle, Path, Symbol, Use};
use svg::Document;

pub struct WorkingStorageSvg {
    pub center: (Number, Number), // size max of a svg (the space left is u
                                  // transparent)
                                  // this is nececary for after that center the
                                  // svg to center and compute some math for
                                  // put the svg in x,y
}

impl WorkingStorageSvg {
    pub fn new(center: (Number, Number)) -> WorkingStorageSvg {
        WorkingStorageSvg { center: center }
    }
}

#[derive(Debug)]
pub struct WorkingStorage {
    max_size: Number,
}

impl WorkingStorage {
    pub fn new(max_size: Number) -> WorkingStorage {
        WorkingStorage { max_size: max_size }
    }
}
pub trait CalcDraw {
    fn get_radius_total(&self) -> Number;
    fn get_radius_circle(&self, occurs: usize) -> (Number, bool);
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

pub trait BodiesSvg {
    fn get_path(&self, bodie: Bodies) -> Path;
    fn get_variable(&self, bodie: Bodies, sw_link: bool) -> String;
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
