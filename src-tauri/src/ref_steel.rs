use std::collections::HashMap;
use crate::types::*;

/// `ReSteelKind`
/// The kind of steel used in the project.
#[derive(Clone, Copy, Debug)]
pub enum RefSteelKind {
    SixMill,
    EightMill,
    ThreeEights,
    TwelveMill,
    OneHalf,
    FiveEights,
    ThreeQuarters,
    One,
    OneAndThreeEights,
}

impl RefSteelKind {
    pub fn from_str(s: &str) -> Option<RefSteelKind> {
        match s {
            "6mm" => Some(RefSteelKind::SixMill),
            "8mm" => Some(RefSteelKind::EightMill),
            "3/8" => Some(RefSteelKind::ThreeEights),
            "12mm" => Some(RefSteelKind::TwelveMill),
            "1/2" => Some(RefSteelKind::OneHalf),
            "5/8" => Some(RefSteelKind::FiveEights),
            "3/4" => Some(RefSteelKind::ThreeQuarters),
            "1" => Some(RefSteelKind::One),
            "1-3/8" => Some(RefSteelKind::OneAndThreeEights),
            _ => None,
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct RefSteel {
    kind: RefSteelKind,
    diameter: SteelDiameter,
    area: CircularSteelArea,
    weight: SteelWeight,
}

fn calculate_steel_area(re_steel_kind: RefSteelKind) -> CircularSteelArea {
    match re_steel_kind {
        RefSteelKind::SixMill => 28.0,
        RefSteelKind::EightMill => 50.0,
        RefSteelKind::ThreeEights => 71.0,
        RefSteelKind::TwelveMill => 113.0,
        RefSteelKind::OneHalf => 129.0,
        RefSteelKind::FiveEights => 199.0,
        RefSteelKind::ThreeQuarters => 284.0,
        RefSteelKind::One => 510.0,
        RefSteelKind::OneAndThreeEights => 1006.0,
    }
}

fn calculate_steel_diameter(re_steel_kind: RefSteelKind) -> SteelDiameter {
    match re_steel_kind {
        RefSteelKind::SixMill => 6.0,
        RefSteelKind::EightMill => 8.0,
        RefSteelKind::ThreeEights => 9.525,
        RefSteelKind::TwelveMill => 12.0,
        RefSteelKind::OneHalf => 12.7,
        RefSteelKind::FiveEights => 15.875,
        RefSteelKind::ThreeQuarters => 19.05,
        RefSteelKind::One => 25.4,
        RefSteelKind::OneAndThreeEights => 34.925,
    }
}

impl RefSteel {
    pub fn new(ref_steel_kind: RefSteelKind) -> RefSteel {
        RefSteel {
            kind: ref_steel_kind,
            diameter: calculate_steel_diameter(ref_steel_kind),
            area: calculate_steel_area(ref_steel_kind),
            // TDO
            weight: 0.0,
        }
    }
}


fn calculate_total_area_refsteel(vec_re_steel: &Vec<RefSteel>) -> CircularSteelArea {
    let mut total_area: CircularSteelArea = 0.0;
    for re_steel in vec_re_steel {
        total_area += re_steel.area;
    }
    total_area
}

fn calculate_number_ref_steel(total_ref_steel_area: CircularSteelArea, selected_proy_ref_steel: Vec<RefSteel>) -> HashMap<u8, RefSteelKind> {
    let mut number_ref_steel: HashMap<u8, RefSteelKind> = HashMap::new();
    let mut total_area: CircularSteelArea = 0.0;
    for ref_steel in selected_proy_ref_steel {
        total_area += ref_steel.area;
    }
    let mut i: u8 = 0;
    for re_steel in selected_proy_ref_steel {
        let mut area_percentage: f64 = re_steel.area / total_area;
        let mut number_steel: u8 = (area_percentage * total_ref_steel_area as f64) as u8;
        number_ref_steel.insert(i, re_steel.kind);
        i += 1;
    }
    number_ref_steel
}

/// STIRRUPS

pub enum StirrupsKind {
    Ties,
    Spirals,
}

impl StirrupsKind {
    pub fn from_str(s: &str) -> Option<StirrupsKind> {
        match s {
            "rectangulares" => Some(StirrupsKind::Ties),
            "espirales" => Some(StirrupsKind::Spirals),
            _ => None,
        }
    }
}

pub struct Stirrups {
    kind: StirrupsKind,
    phi_factor: PhiFactor,
    pn_factor: FactorTransverseReinforcement,
}

impl Stirrups {
    pub fn new(stirrups_kind: StirrupsKind) -> Stirrups {
        match stirrups_kind {
            StirrupsKind::Ties => Stirrups {
                kind: StirrupsKind::Ties,
                phi_factor: PHI_TIES_TRANSVERSE_REINFORCEMENT,
                pn_factor: FACTOR_SPIRALS_TRANSVERSE_REINFORCEMENT,
            },
            StirrupsKind::Spirals => Stirrups {
                kind: StirrupsKind::Spirals,
                phi_factor: PHI_SPIRALS_TRANSVERSE_REINFORCEMENT,
                pn_factor: FACTOR_SPIRALS_TRANSVERSE_REINFORCEMENT,
            },
        }
    }
}


pub struct InitialPercentageRefSteel {
    pub(crate) percentage: f64,
}

impl InitialPercentageRefSteel {
    pub fn new(percentage: f64) -> InitialPercentageRefSteel {
        InitialPercentageRefSteel {
            percentage,
        }
    }
}







