use crate::column::InitColumn;

pub fn axial_calculate_column_area(column: InitColumn) -> f64 {
    let load_factored = column.load.factored_load;
    let phi_factor = column.geometric_props.phi_factor;
    let pn_factor = column.geometric_props.pn_factor;
    let fc = column.concrete_material_props.fc;
    let fy = column.steel_material_props.fy;
    let q = column.initial_percentage_ref_steel.percentage;
    load_factored / (phi_factor * pn_factor * (0.85 * fc * (1.0 - q) + fy * q))
}
