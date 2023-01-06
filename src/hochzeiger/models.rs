use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiData {
    pub lifts: Vec<Lift>,
    pub slopes: Vec<Slope>,
    pub routes: Vec<Route>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lift {
    pub popup: LiftPopup,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LiftPopup {
    pub title: String,
    pub status: SlopeStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Slope {
    pub popup: SlopePopup,
    pub number: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SlopePopup {
    pub title: String,
    pub subtitle: String,
    pub status: SlopeStatus,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SlopeStatus {
    Open,
    Closed,
    Unknown,
}

impl Serialize for SlopeStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
                serializer.serialize_bool(match &self {
                    SlopeStatus::Open => true,
                    _ => false
                })

    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Route {
    pub popup: RoutePopup,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoutePopup {
    pub title: String,
    pub status: SlopeStatus,
}
