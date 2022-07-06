use crate::code_design::CodeDesign;
use crate::types::*;

#[derive(Debug)]
pub struct FactoredLoad {
    normative: CodeDesign,
    live_load_factor: LoadFactor,
    dead_load_factor: LoadFactor,
    live_load: f64,
    dead_load: f64,
    pub factored_load: f64,
}

impl FactoredLoad {
    pub fn new(normative_factoring: CodeDesign, live_load: f64, dead_load: f64) -> FactoredLoad {
        match normative_factoring {
            CodeDesign::ACI => FactoredLoad {
                normative: CodeDesign::ACI,
                live_load_factor: ACI_LIVE_LOAD_FACTOR,
                dead_load_factor: ACI_DEAD_LOAD_FACTOR,
                live_load,
                dead_load,
                factored_load: ACI_LIVE_LOAD_FACTOR * live_load + ACI_DEAD_LOAD_FACTOR * dead_load,
            },
            // TODO Check if this is correct
            CodeDesign::NTP => FactoredLoad {
                normative: CodeDesign::NTP,
                live_load_factor: NTP_LIVE_LOAD_FACTOR,
                dead_load_factor: NTP_DEAD_LOAD_FACTOR,
                live_load,
                dead_load,
                factored_load: NTP_LIVE_LOAD_FACTOR * live_load + NTP_DEAD_LOAD_FACTOR * dead_load,
            },
        }
    }
}
