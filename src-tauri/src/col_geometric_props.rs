use crate::types::*;

// GEOMETRY COLUMN
#[derive(Debug)]
enum GeometricColumnShape {
    Circular,
    Rectangular,
    // TshapedColumn,
    // LshapedColumn,
}

#[derive(Debug)]
pub struct GeometricPropsColumn {
    shape: GeometricColumnShape,
    dimensions: Vec<f64>,
    area: ColArea,
}

impl GeometricPropsColumn {
    pub fn new(dimensions: &Vec<f64>) -> GeometricPropsColumn {
        match dimensions.len() {
            1 => GeometricPropsColumn {
                shape: GeometricColumnShape::Circular,
                dimensions: dimensions.clone(),
                area: &dimensions[0] * &dimensions[0] * PI,
            },
            2 => GeometricPropsColumn {
                shape: GeometricColumnShape::Rectangular,
                dimensions: dimensions.clone(),
                area: &dimensions[0] * &dimensions[1],
            },
            _ => panic!("Invalid number of dimensions"),
        }
    }
}
