use crate::SELevel;
use log::LevelFilter;

#[test]
fn selevel_off() {
    let level = SELevel::new().off(true);
    assert_eq!(LevelFilter::from(level), LevelFilter::Off);
}

#[test]
fn selevel_error() {
    let level = SELevel::new().quiet(true);
    assert_eq!(LevelFilter::from(level), LevelFilter::Error);
}

#[test]
fn selevel_warn() {
    let level = SELevel::new();
    assert_eq!(LevelFilter::from(level), LevelFilter::Warn);
}

#[test]
fn selevel_info() {
    let level = SELevel::new().verbose(true);
    assert_eq!(LevelFilter::from(level), LevelFilter::Info);
}

#[test]
fn selevel_debug() {
    let level = SELevel::new().debug(true);
    assert_eq!(LevelFilter::from(level), LevelFilter::Debug);
}

#[test]
fn selevel_trace() {
    let level = SELevel::new().verbose(true).debug(true);
    assert_eq!(LevelFilter::from(level), LevelFilter::Trace);
}
