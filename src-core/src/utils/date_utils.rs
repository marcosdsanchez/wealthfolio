use crate::errors::{Error, Result, ValidationError};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};

/// Minimum timezone offset: UTC-12 (Baker Island)
const MIN_OFFSET_MINUTES: i32 = -720;

/// Maximum timezone offset: UTC+14 (Kiribati)
const MAX_OFFSET_MINUTES: i32 = 840;

/// Validates that timezone offset is within Earth's actual timezone range.
///
/// # Arguments
/// * `offset_minutes` - Offset in minutes (Local = UTC + offset)
///
/// # Returns
/// * `Ok(offset)` if valid
/// * `Err` if outside range [-720, +840]
fn validate_offset(offset_minutes: i32) -> Result<i32> {
    if offset_minutes < MIN_OFFSET_MINUTES || offset_minutes > MAX_OFFSET_MINUTES {
        return Err(Error::Validation(ValidationError::InvalidInput(format!(
            "Invalid timezone offset: {} minutes. Valid range: {} to {} (UTC-12 to UTC+14).",
            offset_minutes, MIN_OFFSET_MINUTES, MAX_OFFSET_MINUTES
        ))));
    }
    Ok(offset_minutes)
}

/// Converts a UTC DateTime to a Local NaiveDateTime using the provided offset in minutes.
///
/// Convention: Local = UTC + offset_minutes
///
/// # Arguments
/// * `utc` - DateTime in UTC
/// * `offset_minutes` - Timezone offset in minutes
///
/// # Returns
/// * `Ok(NaiveDateTime)` - The local datetime
/// * `Err` - If offset is invalid or date arithmetic overflows
///
/// # Example
/// ```
/// use chrono::{Utc, TimeZone};
/// use wealthfolio_core::utils::date_utils::utc_to_local_with_offset;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // UTC-3 (Argentina): 2024-01-01 00:00 UTC -> 2023-12-31 21:00 Local
/// let utc = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
/// let local = utc_to_local_with_offset(utc, -180)?;
/// # Ok(())
/// # }
/// ```
pub fn utc_to_local_with_offset(utc: DateTime<Utc>, offset_minutes: i32) -> Result<NaiveDateTime> {
    let offset = validate_offset(offset_minutes)?;

    utc.checked_add_signed(Duration::minutes(offset as i64))
        .ok_or_else(|| {
            Error::Validation(ValidationError::InvalidInput(format!(
                "Date overflow when applying timezone offset {} minutes to UTC datetime {}",
                offset, utc
            )))
        })
        .map(|dt| dt.naive_utc())
}

/// Converts a Local NaiveDateTime to a UTC DateTime using the provided offset in minutes.
///
/// Convention: UTC = Local - offset_minutes
///
/// # Arguments
/// * `local` - NaiveDateTime in local timezone
/// * `offset_minutes` - Timezone offset in minutes
///
/// # Returns
/// * `Ok(DateTime<Utc>)` - The UTC datetime
/// * `Err` - If offset is invalid or date arithmetic overflows
///
/// # Example
/// ```
/// use chrono::{NaiveDate, Utc};
/// use wealthfolio_core::utils::date_utils::local_to_utc_with_offset;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // UTC-3 (Argentina): 2024-01-01 00:00 Local -> 2024-01-01 03:00 UTC
/// let local = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
/// let utc = local_to_utc_with_offset(local, -180)?;
/// # Ok(())
/// # }
/// ```
pub fn local_to_utc_with_offset(
    local: NaiveDateTime,
    offset_minutes: i32,
) -> Result<DateTime<Utc>> {
    let offset = validate_offset(offset_minutes)?;

    local
        .checked_sub_signed(Duration::minutes(offset as i64))
        .ok_or_else(|| {
            Error::Validation(ValidationError::InvalidInput(format!(
                "Date overflow when applying timezone offset {} minutes to local datetime {}",
                offset, local
            )))
        })
        .map(|naive| DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate; // Removed TimeZone trait import as it triggered unused import warning

    #[test]
    fn test_local_to_utc_valid_negative_offset() {
        let local = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let utc = local_to_utc_with_offset(local, -180).unwrap();
        assert_eq!(
            utc.naive_utc(),
            NaiveDate::from_ymd_opt(2024, 1, 1)
                .unwrap()
                .and_hms_opt(3, 0, 0)
                .unwrap()
        );
    }

    #[test]
    fn test_local_to_utc_valid_positive_offset() {
        let local = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let utc = local_to_utc_with_offset(local, 120).unwrap();
        assert_eq!(
            utc.naive_utc(),
            NaiveDate::from_ymd_opt(2023, 12, 31)
                .unwrap()
                .and_hms_opt(22, 0, 0)
                .unwrap()
        );
    }

    #[test]
    fn test_rejects_offset_too_negative() {
        let local = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert!(local_to_utc_with_offset(local, -999).is_err());
    }

    #[test]
    fn test_rejects_offset_too_positive() {
        let local = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert!(local_to_utc_with_offset(local, 999).is_err());
    }

    #[test]
    fn test_accepts_valid_extreme_offsets() {
        let local = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert!(local_to_utc_with_offset(local, -720).is_ok());
        assert!(local_to_utc_with_offset(local, 840).is_ok());
    }

    #[test]
    fn test_handles_half_hour_offsets() {
        let local = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();
        let utc = local_to_utc_with_offset(local, 330).unwrap();
        assert_eq!(
            utc.naive_utc(),
            NaiveDate::from_ymd_opt(2024, 1, 1)
                .unwrap()
                .and_hms_opt(6, 30, 0)
                .unwrap()
        );
    }
}
