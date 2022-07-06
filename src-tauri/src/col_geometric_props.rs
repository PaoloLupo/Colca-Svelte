use crate::types::*;

// GEOMETRY COLUMN
enum GeometricColumnShape {
    Circular,
    Rectangular,
    // TshapedColumn,
    // LshapedColumn,
}

pub struct GeometricPropsColumn {
    shape: GeometricColumnShape,
    height: ColHeight,
    width: ColWidth,
    radius: ColRadius,
    area: ColArea,
}

impl GeometricPropsColumn {
    fn calculate_area(&self) -> ColArea {
        match self.shape {
            GeometricColumnShape::Circular => {
                let area = (self.radius * self.radius) * std::f64::consts::PI;
                area
            }
            GeometricColumnShape::Rectangular => {
                let area = self.height * self.width;
                area
            }
            // GeometricColumnShape::TshapedColumn => {
            //     let area = self.height * self.width;
            //     area
            // },
            // GeometricColumnShape::LshapedColumn => {
            //     let area = self.height * self.width;
            //     area
            // },
        }
    }

    pub fn new_circular(radius: ColRadius) -> GeometricPropsColumn {
        GeometricPropsColumn {
            shape: GeometricColumnShape::Circular,
            height: 0.0,
            width: 0.0,
            radius,
            area: GeometricColumnShape::Circular.calculate_area(),
        }
    }

    pub fn new_rectangular(height: ColHeight, width: ColWidth) -> GeometricPropsColumn {
        GeometricPropsColumn {
            shape: GeometricColumnShape::Rectangular,
            height,
            width,
            radius: 0.0,
            area: GeometricColumnShape::Rectangular.calculate_area(),
        }
    }
}

