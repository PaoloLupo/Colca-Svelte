const COLCA_SUFIX_FILE: &str = "clc";

const TMP_PROJECT_NAME: &str = "project_tmp";
const TMP_PROJECT_AUTHOR: &str = "author_tmp";
const TMP_PROJECT_COMPANY: &str = "company_tmp";

pub type SteelDiameter = f64;
pub type SteelWeight = f64;
pub type CircularSteelArea = f64;
pub type ColHeight = f64;
pub type ColWidth = f64;
pub type ColArea = f64;
pub type ColRadius = f64;
pub type LoadFactor = f64;
pub type FactorTransverseReinforcement = f64;
pub type PhiFactor = f64;
pub type AnalysisType = String;

pub const PI:f64 = std::f64::consts::PI;

// CONSTANTS
pub const INITIAL_PERCENTAGE_STEEL: f64 = 0.02; // 2%

pub const ACI_DEAD_LOAD_FACTOR: LoadFactor = 1.2;
pub const ACI_LIVE_LOAD_FACTOR: LoadFactor = 1.6;
pub const NTP_LIVE_LOAD_FACTOR: LoadFactor = 1.7;
pub const NTP_DEAD_LOAD_FACTOR: LoadFactor = 1.4;

pub const FACTOR_TIES_TRANSVERSE_REINFORCEMENT: FactorTransverseReinforcement = 0.80;
pub const FACTOR_SPIRALS_TRANSVERSE_REINFORCEMENT: FactorTransverseReinforcement = 0.85;
pub const PHI_SPIRALS_TRANSVERSE_REINFORCEMENT: PhiFactor = 0.75;
pub const PHI_TIES_TRANSVERSE_REINFORCEMENT: PhiFactor = 0.65;

// MPa
pub const C21_COMPRESSIVE_STRENGTH_SI: f64 = 21.0;
pub const C28_COMPRESSIVE_STRENGTH_SI: f64 = 28.0;
// MPa
pub const GRADE60_YIELD_STRENGTH_SI: f64 = 420.0; // MPa