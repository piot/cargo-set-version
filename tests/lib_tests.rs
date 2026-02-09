use cargo_set_version::{ensure_version_increase, parse_new_version};
use semver::Version;

#[test]
fn parse_new_version_rejects_invalid_input() {
    let err = parse_new_version("bad!!").unwrap_err();
    assert!(
        err.to_string()
            .contains("'bad!!' is not a valid semver version")
    );
}

#[test]
fn bigger_version_allows_update() {
    let new_version = Version::new(2, 0, 0);
    let current = Version::new(1, 5, 0);
    ensure_version_increase(&new_version, &current, "crate").unwrap();
}

#[test]
fn smaller_version_rejected() {
    let new_version = Version::new(1, 0, 0);
    let current = Version::new(1, 5, 0);
    let err = ensure_version_increase(&new_version, &current, "crate").unwrap_err();
    assert!(
        err.to_string()
            .contains("new version '1.0.0' must be greater than current version '1.5.0'")
    );
}

#[test]
fn equal_version_rejected() {
    let new_version = Version::new(1, 5, 0);
    let current = Version::new(1, 5, 0);
    let err = ensure_version_increase(&new_version, &current, "crate").unwrap_err();
    assert!(
        err.to_string()
            .contains("new version '1.5.0' must be greater than current version '1.5.0'")
    );
}
