extern crate geometry;


use geometry::{Geometry, Attribute, AttributeValue};


static VERTICES: [f32; 6] = [
    0.0,  0.5,
    -0.5, -0.5,
    0.5, -0.5
];

static INDICES: [u32; 3] = [
    0, 1, 2
];


#[test]
fn test_geometry() {
    let mut geo = Geometry::new();

    geo.add_attribute(Attribute::new_f32("vertices", &VERTICES, 2, false));
    geo.add_attribute(Attribute::new_u32("indices", &INDICES, 1, false));

    let vertices = geo.get_attribute("vertices").unwrap();
    match vertices.value {
        AttributeValue::F32(v) => assert_eq!(v, VERTICES),
        _ => panic!("unexpected type"),
    }

    let indices = geo.get_attribute("indices").unwrap();
    match indices.value {
        AttributeValue::U32(i) => assert_eq!(i, INDICES),
        _ => panic!("unexpected type"),
    }
}
