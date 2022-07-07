#[derive(Debug, Clone, serde::Serialize)]
pub enum CodeDesign {
    ACI,
    NTP,
}

impl CodeDesign {
    pub fn from_str(s: &str) -> Option<CodeDesign> {
        match s {
            "ACI" => Some(CodeDesign::ACI),
            "NTP" => Some(CodeDesign::NTP),
            _ => None,
        }
    }
}
