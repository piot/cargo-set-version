use anyhow::{Result, anyhow};
use semver::Version;

pub fn parse_new_version(input: &str) -> Result<Version> {
    Version::parse(input)
        .map_err(|err| anyhow!("'{}' is not a valid semver version: {}", input, err))
}

pub fn ensure_version_increase(new: &Version, current: &Version, package_name: &str) -> Result<()> {
    if new <= current {
        Err(anyhow!(
            "new version '{}' must be greater than current version '{}' for package '{}'",
            new,
            current,
            package_name,
        ))
    } else {
        Ok(())
    }
}
