use crate::code_design::CodeDesign;
use crate::col_geometric_props::GeometricPropsColumn;
use crate::column::InitColumn;
use crate::load_factoring::FactoredLoad;
use crate::material_props::{
    ConcreteMaterialKind, ConcreteMaterialProps, SteelMaterialKind, SteelMaterialProps,
};
use crate::ref_steel::{InitialPercentageRefSteel, RefSteel, RefSteelKind, Stirrups, StirrupsKind};
use crate::types::AnalysisType;

pub fn calculate_load_factored(
    normative_factoring: String,
    live_load: f64,
    dead_load: f64,
) -> FactoredLoad {
    let normative_factoring = CodeDesign::from_str(&normative_factoring).unwrap();
    FactoredLoad::new(normative_factoring, live_load, dead_load)
}

fn select_column_shape(dimensions: &Vec<f64>) -> GeometricPropsColumn {
    GeometricPropsColumn::new(dimensions)
}

fn selected_concrete_material(concrete_material: String) -> ConcreteMaterialProps {
    let concrete_material_kind = ConcreteMaterialKind::from_str(&concrete_material).unwrap();
    ConcreteMaterialProps::new(concrete_material_kind)
}

fn selected_steel_material(steel_material: String) -> SteelMaterialProps {
    let steel_material_kind = SteelMaterialKind::from_str(&steel_material).unwrap();
    SteelMaterialProps::new(steel_material_kind)
}

fn select_initial_percentage_ref_steel(percentage: f64) -> InitialPercentageRefSteel {
    InitialPercentageRefSteel::new(percentage)
}

fn select_proy_ref_steel(array_re_steel: Vec<String>) -> Vec<RefSteel> {
    let mut vec_ref_steel: Vec<RefSteel> = Vec::new();
    for ref_steel in array_re_steel {
        let ref_steel_kind = RefSteelKind::from_str(&ref_steel).unwrap();
        vec_ref_steel.push(RefSteel::new(ref_steel_kind));
    }
    vec_ref_steel
}

fn selected_stirrups_type(stirrups: String) -> Stirrups {
    let stirrups_kind = StirrupsKind::from_str(&stirrups).unwrap();
    Stirrups::new(stirrups_kind)
}

pub fn new_init_column(
    name: String,
    analysis_type: Vec<AnalysisType>,
    normative: String,
    live_load: f64,
    dead_load: f64,
    dimensions: Vec<f64>,
    concrete_material: String,
    steel_material: String,
    selected_ref_steel: Vec<String>,
    percentage: f64,
    stirrups: String,
) -> InitColumn {
    let load = calculate_load_factored(normative, live_load, dead_load);
    let geometric_props = select_column_shape(&dimensions);
    let concrete_material_props = selected_concrete_material(concrete_material);
    let steel_material_props = selected_steel_material(steel_material);
    let initial_percentage_ref_steel = select_initial_percentage_ref_steel(percentage);
    let ref_steel_selected = select_proy_ref_steel(selected_ref_steel);
    let stirrups_type = selected_stirrups_type(stirrups);
    InitColumn {
        name,
        analysis_type,
        load,
        geometric_props,
        concrete_material_props,
        steel_material_props,
        initial_percentage_ref_steel,
        ref_steel_selected,
        stirrups_type,
    }
}
