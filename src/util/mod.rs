/*
 * Converts a standard 0-359 degree heading into the slightly insane format Euroscope uses.
 * The formula used is $$floor((heading * 2.88 + 0.5) * 4)$$ .
 */
pub fn es_heading(heading: u16) -> u16 {
    if heading > 360 {
        panic!("ES Heading only defined for 0<=heading<=360");
    }

    (((heading as f32) * 2.88 + 0.5) * 4.0) as u16
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_es_heading() {
        let values: [u16; 5] = [0, 1, 100, 259, 360];
        let results: [u16; 5] = [2, 13, 1154, 2985, 4149];

        for i in 0..values.len() {
            assert_eq!(results[i], es_heading(values[i]));
        }
    }

    #[test]
    #[should_panic]
    fn ensure_denies_invalid_heading() {
        let _ = es_heading(361);
    }
}
