#[cfg(test)]
mod tests;

pub mod basic_laws;

pub struct ElementBuilder {
    pub name: Option<String>,
    pub voltage: Option<f64>,
    pub current: Option<f64>,
    pub resistance: Option<f64>,
}

pub struct Element {
    pub name: Option<String>,
    pub voltage: f64,
    pub current: f64,
    pub resistance: f64
}

/// Errors encountered when building an [`Element`] from an [`ElementBuilder`].
#[derive(Debug, PartialEq)]
pub enum ElementBuildError {
    /// There must be at least two of voltage, current, or resistance filled.
    NotEnoughData,
    /// This occurs when all three of the data fields are filled, but the
    /// values are incompatible with the equations.
    InvalidData
}

impl ElementBuilder {
    pub fn new() -> ElementBuilder {
        ElementBuilder {
            name: None,
            voltage: None,
            current: None,
            resistance: None,
        }
    }

    pub fn name<S: Into<String>>(self, name: S) -> ElementBuilder {
        ElementBuilder {
            name: Some(name.into()),
            voltage: self.voltage,
            current: self.current,
            resistance: self.resistance,
        }
    }

    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = Some(name.into());
    }

    pub fn voltage(self, voltage: f64) -> ElementBuilder {
        ElementBuilder {
            name: self.name,
            voltage: Some(voltage),
            current: self.current,
            resistance: self.resistance,
        }
    }

    pub fn set_voltage(&mut self, voltage: f64) {
        self.voltage = Some(voltage);
    }

    pub fn current(self, current: f64) -> ElementBuilder {
        ElementBuilder {
            name: self.name,
            voltage: self.voltage,
            current: Some(current),
            resistance: self.resistance,
        }
    }

    pub fn set_current(&mut self, current: f64) {
        self.current = Some(current);
    }

    pub fn resistance(self, resistance: f64) -> ElementBuilder {
        ElementBuilder {
            name: self.name,
            voltage: self.voltage,
            current: self.current,
            resistance: Some(resistance),
        }
    }

    pub fn set_resistance(&mut self, resistance: f64) {
        self.resistance = Some(resistance);
    }

    pub fn build(self) -> Result<Element, ElementBuildError> {
        let (v, i, r) = self.calculate_values()?;

        Ok(Element {
            name: self.name,
            voltage: v,
            current: i,
            resistance: r
        })
    }

    fn calculate_values(&self) -> Result<(f64, f64, f64), ElementBuildError> {
        match (self.voltage, self.current, self.resistance) {
            (Some(v), Some(i), Some(r)) => {
                if basic_laws::is_data_valid(v, i, r) {
                    Ok((v, i, r))
                } else {
                    Err(ElementBuildError::InvalidData)
                }
            },
            (Some(v), Some(i), None) => Ok((v, i, basic_laws::ohms_law_resistance(v, i))),
            (Some(v), None, Some(r)) => Ok((v, basic_laws::ohms_law_current(v, r), r)),
            (None, Some(i), Some(r)) => Ok((basic_laws::ohms_law_voltage(i, r), i, r)),
            _ => Err(ElementBuildError::NotEnoughData),
        }
    }
}