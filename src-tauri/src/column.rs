// COLUMN PROPERTIES

use crate::col_geometric_props::GeometricPropsColumn;
use crate::load_factoring::FactoredLoad;
use crate::material_props::{ConcreteMaterialProps, SteelMaterialProps};
use crate::ref_steel::{InitialPercentageRefSteel, RefSteel, Stirrups};
use crate::types::*;

#[derive(Debug)]
pub struct InitColumn {
    pub name: String,
    pub analysis_type: Vec<AnalysisType>,
    pub load: FactoredLoad,
    pub geometric_props: GeometricPropsColumn,
    pub concrete_material_props: ConcreteMaterialProps,
    pub steel_material_props: SteelMaterialProps,
    pub initial_percentage_ref_steel: InitialPercentageRefSteel,
    pub ref_steel_selected: Vec<RefSteel>,
    pub stirrups_type: Stirrups,
}

pub fn add_column(column: InitColumn) -> Vec<InitColumn> {
    let mut columns = Vec::new();
    columns.push(column);
    columns
}
