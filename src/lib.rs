use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

pub fn chart(screen_size: f64) -> String {
    let data1 = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let data2 = Data::new()
        .move_to((30, 30))
        .line_by((0, 50))
        .line_by((20, 0))
        .line_by((0, -20))
        .close();

    let path1 = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data1);

    let path2 = Path::new()
        .set("fill", "none")
        .set("stroke", "red")
        .set("stroke-width", 1)
        .set("d", data2);

    let document = Document::new()
        .set("viewBox", (0, 0, screen_size as i32, screen_size as i32))
        .add(path1)
        .add(path2);

    //svg::save("image.svg", &document).unwrap();
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
