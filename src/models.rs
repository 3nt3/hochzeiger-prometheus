use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiData {
    pub lifts: Vec<Lift>,
    pub slopes: Vec<Slope>,
    pub routes: Vec<Route>,
}

#[derive(Deserialize, Debug)]
pub struct Lift {
    popup: LiftPopup
}

#[derive(Deserialize, Debug)]
pub struct LiftPopup {
    title: String
}

#[derive(Deserialize, Debug)]
pub struct Slope {}
#[derive(Deserialize, Debug)]
pub struct Route {}
