use super::FlightRules;
use rand::Rng;

#[derive(Debug, Default)]
#[cfg_attr(features = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlightPlan {
    pub callsign: String,
    pub aircraft: String,
    pub adep: String,
    pub ades: String,
    pub alternate: String,
    pub route: String,
    pub rules: FlightRules,
    pub tas: u16,
    pub rfl: u32,
    pub flight_time: String,
    pub squawk: String, // Just easier to handle...
    pub remarks: Option<String>
}

impl FlightPlan {
    pub fn make_squawk(mut self) -> Self {
        if self.rules == FlightRules::VFR {
            self.squawk = String::from("7000");
            return self;
        }

        // We pick between 0o7776 and 0o0, then add one, to avoid selecting 0000
        let max_octal: u32 = 0o7776;
        let mut rng = rand::thread_rng();
        let numeric_squawk = rng.gen_range(0..max_octal);
        self.squawk = format!("{:0}", numeric_squawk);
        self
    }

    pub fn weight_category(&self) -> char {
        self.aircraft.chars().rev().nth(0).unwrap_or('M')
    }

    pub fn flight_plan_line(&self) -> String {
        // We zero out the EST and ACT dep time, flight time, fuel time, because who cares
        format!(
            "$FP{callsign}:*A:{rules}:{aircraft}:{tas}:{adep}:0000:0000:{rfl}:{ades}:00:00:00:00:{alternate}:{remarks}:{route}\n",
            callsign=self.callsign,
            rules=self.rules.to_char(),
            aircraft=self.aircraft,
            tas=self.tas, // TODO check format, should be 3 digits in kt
            adep=self.adep,
            rfl=self.rfl, // TODO check format pls
            ades=self.ades,
            alternate=self.alternate,
            remarks=self.remarks.clone().unwrap_or(String::from("/V/")),
            route=self.route,
        ).to_string()
    }

    pub fn simdata_line(&self) -> String {
        format!(
            "SIMDATA:{callsign}:*:*:{taxi_speed}:1:0\n",
            callsign=self.callsign,
            taxi_speed=if self.weight_category() == 'L' { 15 } else { 20 },
        ).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weight_categories() {
        let aircraft = ["A20N/M", "C25C/L", "E190/M", "B77W/H"];
        let answers = ['M', 'L', 'M', 'H'];
        let mut flight_plan = FlightPlan::default();

        for i in 0..aircraft.len() {
            flight_plan.aircraft = String::from(aircraft[i]);
            assert_eq!(flight_plan.weight_category(), answers[i]);
        }
    }
}
