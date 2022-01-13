use super::*;

#[test]
fn test_element_builder_with_all_valid_data() -> Result<(), ElementBuildError> {
    let element = ElementBuilder::new()
        .name("Test")
        .voltage(1.5)
        .current(1.0)
        .resistance(1.5)
        .build()?;

    assert_eq!(element.name, Some("Test".to_string()));
    assert_eq!(element.voltage, 1.5);
    assert_eq!(element.current, 1.0);
    assert_eq!(element.resistance, 1.5);

    Ok(())
}

#[test]
fn test_element_builder_with_valid_data_no_name() -> Result<(), ElementBuildError> {
    let element = ElementBuilder::new()
        .voltage(1.5)
        .current(1.0)
        .resistance(1.5)
        .build()?;

    assert_eq!(element.name, None);
    assert_eq!(element.voltage, 1.5);
    assert_eq!(element.current, 1.0);
    assert_eq!(element.resistance, 1.5);

    Ok(())
}

#[test]
fn test_element_builder_resistance_calculation() -> Result<(), ElementBuildError> {
    let element = ElementBuilder::new()
        .voltage(1.5)
        .current(1.0)
        .build()?;

    assert_eq!(element.resistance, 1.5);

    Ok(())
}

#[test]
fn test_element_builder_resistance_calculation_with_negative_current() -> Result<(), ElementBuildError> {
    let element = ElementBuilder::new()
        .voltage(1.5)
        .current(-1.0)
        .build()?;

    assert_eq!(element.resistance, 1.5);

    Ok(())
}