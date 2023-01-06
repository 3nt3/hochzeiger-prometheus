use crate::hochzeiger::models::{SlopeStatus,Lift,Slope};

pub fn output_slopes(slopes: Vec<Slope>) {
    println!("=== SLOPES ===");
    slopes.iter().for_each(|slope| {
        let emoji = match &slope.popup.status {
            SlopeStatus::Open => "🦈",
            SlopeStatus::Closed => "😭",
            SlopeStatus::Unknown => "???"
        };
        let text = match &slope.popup.status {
            SlopeStatus::Open => "open",
            SlopeStatus::Closed => "closed",
            SlopeStatus::Unknown => "man weiß es nicht digga"
        };
        println!("{} {} is {}", emoji, slope.popup.title, text);
    });
}

pub fn output_lifts(lifts: Vec<Lift>) {
    println!("=== LIFTS ===");
    lifts.iter().for_each(|lift| {
        let emoji = match &lift.popup.status {
            SlopeStatus::Open => "🦈",
            SlopeStatus::Closed => "😭",
            SlopeStatus::Unknown => "???"
        };
        let text = match &lift.popup.status {
            SlopeStatus::Open => "open",
            SlopeStatus::Closed => "closed",
            SlopeStatus::Unknown => "man weiß es nicht digga"
        };
        println!("{} {} is {}", emoji, lift.popup.title, text);
    });
}
