pub fn ohms_law_resistance(voltage: f64, current: f64) -> f64 {
    (voltage / current).abs()
}

pub fn ohms_law_voltage(resistance: f64, current: f64) -> f64 {
    resistance * current
}

pub fn ohms_law_current(resistance: f64, voltage: f64) -> f64 {
    voltage / resistance
}

pub fn is_data_valid(v: f64, i: f64, r: f64) -> bool {
    // A negative resistance doesn't make sense
    if r < 0.0 {
        return false;
    }

    let calculated_vir = (
        ohms_law_voltage(r, i),
        ohms_law_current(r, v),
        ohms_law_resistance(v, i),
    );

    calculated_vir == (v, i, r)
}