use crate::hochzeiger;
use crate::hochzeiger::models::SlopeStatus;
use crate::util::{output_lifts, output_slopes};
use rocket::get;
use rocket::http::Status;

#[get("/metrics")]
pub async fn get_metrics() -> (Status, String) {
    let api_data = hochzeiger::api::get_info().await;
    if let Err(why) = api_data {
        return (
            Status::InternalServerError,
            format!("error querying hochzeiger api: {why}"),
        );
    }

    output_lifts(api_data.as_ref().unwrap().lifts.clone());
    output_slopes(api_data.as_ref().unwrap().slopes.clone());

    // let mut lifts: HashMap<String, SlopeStatus> = HashMap::new();
    let lifts: Vec<(String, SlopeStatus)> = api_data
        .as_ref()
        .unwrap()
        .lifts
        .iter()
        .map(|lift| (lift.popup.title.clone(), lift.popup.status.clone()))
        .collect();
    let slopes: Vec<(String, SlopeStatus)> = api_data
        .as_ref()
        .unwrap()
        .slopes
        .iter()
        .map(|slope| (slope.popup.title.clone(), slope.popup.status.clone()))
        .collect();

    let lifts_encoded = lifts
        .iter()
        .map(|(name, status)| {
            format!(
                "lift{{name=\"{}\"}} {}",
                name,
                if let SlopeStatus::Open = status {
                    1
                } else {
                    0
                },
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let slopes_encoded = slopes
        .iter()
        .map(|(name, status)| {
            format!(
                "slope{{name=\"{}\"}} {}",
                name,
                if let SlopeStatus::Open = status {
                    1
                } else {
                    0
                },
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    (Status::Ok, format!("{}\n{}", lifts_encoded, slopes_encoded))
}
