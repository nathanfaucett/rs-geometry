#[macro_use]
extern crate vector;

extern crate geometry;

use geometry::{Geometry, Attribute, AttributeValue};


#[test]
fn test_geometry() {
    let mut geo = Geometry::new();

    let vertices = vector![
        0.0f32,  0.5f32,
        -0.5f32, -0.5f32,
        0.5f32, -0.5f32
    ];
    let indices = vector![
        0u32, 1u32, 2u32
    ];

    geo.add_attribute(Attribute::new_f32("vertices", vertices.clone(), 2, false));
    geo.add_attribute(Attribute::new_u32("indices", indices.clone(), 1, false));

    let vertices_attribute = geo.get_attribute("vertices").unwrap();
    match vertices_attribute.value {
        AttributeValue::F32(ref v) => {
            assert_eq!(**v, *vertices);
        },
        _ => panic!("unexpected type"),
    }

    let indices_attribute = geo.get_attribute("indices").unwrap();
    match indices_attribute.value {
        AttributeValue::U32(ref i) => {
            assert_eq!(**i, *indices);
        },
        _ => panic!("unexpected type"),
    }
}
