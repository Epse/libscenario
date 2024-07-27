use crate::util::es_heading;
use super::FlightPlan;

#[derive(Debug)]
#[cfg_attr(features = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScheduledFlightPlan {
    pub flight_plan: FlightPlan,
    pub position: String, // ES-style coordinates
    pub altitude: u32,
    pub start: u64,
    pub initial_pseudo_pilot: String,
    pub route: Option<String>, // Fallback to flight_plan route
    pub req_alt: String, // e.g. EBBR:2000
    pub fault_description: Option<String>,
    pub heading: Option<u16>,
}

impl ScheduledFlightPlan {
    pub fn to_euroscope(&self) -> String {
        let mut line = self.fault_description.clone().unwrap_or(String::new());

        line.push_str(&self.definition_line());
        line.push_str(&self.flight_plan.flight_plan_line());
        if let Some(route) = &self.route {
            line.push_str(&format!("$ROUTE:{}\n", route));
        } else {
            line.push_str(&format!("$ROUTE:{}\n", self.flight_plan.route));
        }

        line.push_str(&format!("START:{}\n", self.start));
        line.push_str(&format!("REQALT:{}\n", self.req_alt));
        line.push_str(&format!("INITIALPSEUDOPILOT:{}\n", self.initial_pseudo_pilot));
        line.push_str(&self.flight_plan.simdata_line());

        line
    }

    fn definition_line(&self) -> String {
        format!(
            "@N:{callsign}:{squawk}:1:{position}:{altitude}:0:{heading}:0\n",
            callsign=self.flight_plan.callsign,
            squawk=self.flight_plan.squawk,
            position=self.position,
            altitude=self.altitude,
            heading=es_heading(self.heading.unwrap_or(0))
        ).to_string()
    }
}
