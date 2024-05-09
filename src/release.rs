use chrono::{Duration, NaiveDate, Utc};

const EPOCH_DATE: NaiveDate = NaiveDate::from_ymd_opt(2015, 12, 10).unwrap();
/// The version release on the epoch date was `1.5.0`, or `5`
const EPOCH_VERSION: i64 = 5;

#[derive(Debug)]
pub struct Release {
    pub(crate) version: semver::Version,
    pub(crate) stable_on: NaiveDate,
    pub(crate) branch_on: NaiveDate,
}

impl Release {
    pub fn new(incr: i64) -> Self {
        let new_releases = (Utc::now().naive_utc().date() - EPOCH_DATE).num_weeks() / 6;
        let release_date = EPOCH_DATE + Duration::weeks((new_releases + incr) * 6);
        let branch_date =
            EPOCH_DATE + Duration::weeks((new_releases + incr - 1) * 6) - Duration::days(6);

        let mut version = semver::Version::new(1, 0, 0);
        version.minor = version.minor.saturating_add_signed(new_releases + EPOCH_VERSION + incr);
        Self {
            version,
            stable_on: release_date,
            branch_on: branch_date,
        }
    }

    pub fn time_stable(&self) -> String {
        let weeks = (self.stable_on - Utc::now().naive_utc().date()).num_weeks();
        let days = ((self.stable_on - Utc::now().naive_utc().date()).num_days() % 7).abs();
        format!("{weeks}w {days}d")
    }

    pub fn time_branch(&self) -> String {
        let weeks = (self.branch_on - Utc::now().naive_utc().date()).num_weeks();
        let days = ((self.branch_on - Utc::now().naive_utc().date()).num_days() % 7).abs();
        format!("{weeks}w {days}d")
    }
}
