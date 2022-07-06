use crate::material_props::ConcreteMaterialKind::C28;
use crate::types::*;

// STEEL MATERIAL
pub(crate) enum SteelMaterialKind {
    Grade60,
}

impl SteelMaterialKind {
    pub(crate) fn from_str(s: &str) -> Option<SteelMaterialKind> {
        match s {
            "Grado 60" => Some(SteelMaterialKind::Grade60),
            _ => None,
        }
    }
}

pub struct SteelMaterialProps {
    kind: SteelMaterialKind,
    yield_strength: f64,
}


impl SteelMaterialProps {
    pub fn new(steel_material_kind: SteelMaterialKind) -> SteelMaterialProps {
        match steel_material_kind {
            SteelMaterialKind::Grade60 => SteelMaterialProps {
                kind: SteelMaterialKind::Grade60,
                yield_strength: GRADE60_YIELD_STRENGTH_SI,
            },
        }
    }
}


// CONCRETE MATERIAL
pub(crate) enum ConcreteMaterialKind {
    C21,
    C28,
}

impl ConcreteMaterialKind {
    pub(crate) fn from_str(s: &str) -> Option<ConcreteMaterialKind> {
        match s {
            "21 MPa" => Some(ConcreteMaterialKind::C21),
            "28 MPa" => Some(C28),
            _ => None,
        }
    }
}

pub struct ConcreteMaterialProps {
    kind: ConcreteMaterialKind,
    compressive_strength: f64,
}

impl ConcreteMaterialProps {
    pub fn new(concrete_material_kind: ConcreteMaterialKind) -> ConcreteMaterialProps {
        match concrete_material_kind {
            ConcreteMaterialKind::C21 => ConcreteMaterialProps {
                kind: ConcreteMaterialKind::C21,
                compressive_strength: C21_COMPRESSIVE_STRENGTH_SI,
            },
            C28 => ConcreteMaterialProps {
                kind: C28,
                compressive_strength: C28_COMPRESSIVE_STRENGTH_SI,
            },
        }
    }
}




