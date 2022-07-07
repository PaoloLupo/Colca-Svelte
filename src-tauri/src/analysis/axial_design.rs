use crate::column::InitColumn;

pub fn axial_calculate_column_area(column: &InitColumn) -> f64 {
    let load_factored = column.load.factored_load;
    let phi_factor = column.stirrups_type.phi_factor;
    let pn_factor = column.stirrups_type.pn_factor;
    let fc = column.concrete_material_props.compressive_strength;
    let fy = column.steel_material_props.yield_strength;
    let q = column.initial_percentage_ref_steel.percentage;
    load_factored / (phi_factor * pn_factor * (0.85 * fc * (1.0 - q) + fy * q))
}

