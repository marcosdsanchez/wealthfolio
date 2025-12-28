#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
    use rust_decimal::Decimal;
    use std::sync::Arc;

    use crate::activities::activities_traits::ActivityRepositoryTrait;
    use crate::errors::Result;
    use crate::fx::fx_traits::FxServiceTrait;
    use crate::limits::limits_model::{ContributionLimit, NewContributionLimit};
    use crate::limits::limits_service::ContributionLimitService;
    use crate::limits::limits_traits::{
        ContributionLimitRepositoryTrait, ContributionLimitServiceTrait,
    };

    struct MockLimitRepository {
        limit: ContributionLimit,
    }

    #[async_trait]
    impl ContributionLimitRepositoryTrait for MockLimitRepository {
        fn get_contribution_limits(&self) -> Result<Vec<ContributionLimit>> {
            unimplemented!()
        }
        fn get_contribution_limit(&self, _id: &str) -> Result<ContributionLimit> {
            Ok(self.limit.clone())
        }
        async fn create_contribution_limit(
            &self,
            _n: NewContributionLimit,
        ) -> Result<ContributionLimit> {
            unimplemented!()
        }
        async fn update_contribution_limit(
            &self,
            _id: &str,
            _n: NewContributionLimit,
        ) -> Result<ContributionLimit> {
            unimplemented!()
        }
        async fn delete_contribution_limit(&self, _id: &str) -> Result<()> {
            unimplemented!()
        }
    }

    struct MockActivityRepository {
        activities: Vec<(String, Decimal, Decimal, String, Option<Decimal>)>,
        last_query_range: Arc<std::sync::Mutex<Option<(NaiveDateTime, NaiveDateTime)>>>,
    }

    #[async_trait]
    impl ActivityRepositoryTrait for MockActivityRepository {
        fn get_deposit_activities(
            &self,
            _account_ids: &[String],
            start_date: NaiveDateTime,
            end_date: NaiveDateTime,
        ) -> Result<Vec<(String, Decimal, Decimal, String, Option<Decimal>)>> {
            let mut range = self.last_query_range.lock().unwrap();
            *range = Some((start_date, end_date));
            Ok(self.activities.clone())
        }
        // ... rest unimplemented
        fn get_activity(&self, _id: &str) -> Result<crate::activities::Activity> {
            unimplemented!()
        }
        fn get_activities(&self) -> Result<Vec<crate::activities::Activity>> {
            unimplemented!()
        }
        fn get_activities_by_account_id(
            &self,
            _id: &str,
        ) -> Result<Vec<crate::activities::Activity>> {
            unimplemented!()
        }
        fn get_activities_by_account_ids(
            &self,
            _ids: &[String],
        ) -> Result<Vec<crate::activities::Activity>> {
            unimplemented!()
        }
        fn get_trading_activities(&self) -> Result<Vec<crate::activities::Activity>> {
            unimplemented!()
        }
        fn get_income_activities(&self) -> Result<Vec<crate::activities::Activity>> {
            unimplemented!()
        }
        fn search_activities(
            &self,
            _p: i64,
            _s: i64,
            _af: Option<Vec<String>>,
            _tf: Option<Vec<String>>,
            _ak: Option<String>,
            _so: Option<crate::activities::Sort>,
        ) -> Result<crate::activities::ActivitySearchResponse> {
            unimplemented!()
        }
        async fn create_activity(
            &self,
            _n: crate::activities::NewActivity,
        ) -> Result<crate::activities::Activity> {
            unimplemented!()
        }
        async fn update_activity(
            &self,
            _u: crate::activities::ActivityUpdate,
        ) -> Result<crate::activities::Activity> {
            unimplemented!()
        }
        async fn delete_activity(&self, _id: String) -> Result<crate::activities::Activity> {
            unimplemented!()
        }
        async fn bulk_mutate_activities(
            &self,
            _c: Vec<crate::activities::NewActivity>,
            _u: Vec<crate::activities::ActivityUpdate>,
            _d: Vec<String>,
        ) -> Result<crate::activities::ActivityBulkMutationResult> {
            unimplemented!()
        }
        async fn create_activities(
            &self,
            _a: Vec<crate::activities::NewActivity>,
        ) -> Result<usize> {
            unimplemented!()
        }
        fn get_first_activity_date(
            &self,
            _ids: Option<&[String]>,
        ) -> Result<Option<DateTime<Utc>>> {
            unimplemented!()
        }
        fn get_first_activity_date_overall(&self) -> Result<DateTime<Utc>> {
            unimplemented!()
        }
        fn get_import_mapping(
            &self,
            _id: &str,
        ) -> Result<Option<crate::activities::ImportMapping>> {
            unimplemented!()
        }
        async fn save_import_mapping(&self, _m: &crate::activities::ImportMapping) -> Result<()> {
            unimplemented!()
        }
        fn calculate_average_cost(&self, _acc: &str, _ass: &str) -> Result<Decimal> {
            unimplemented!()
        }
        fn get_income_activities_data(
            &self,
        ) -> Result<Vec<crate::activities::activities_model::IncomeData>> {
            unimplemented!()
        }
    }

    struct MockFxService;
    #[async_trait]
    impl FxServiceTrait for MockFxService {
        fn initialize(&self) -> Result<()> {
            Ok(())
        }
        async fn add_exchange_rate(
            &self,
            _n: crate::fx::fx_model::NewExchangeRate,
        ) -> Result<crate::fx::fx_model::ExchangeRate> {
            unimplemented!()
        }
        fn get_historical_rates(
            &self,
            _f: &str,
            _t: &str,
            _d: i64,
        ) -> Result<Vec<crate::fx::fx_model::ExchangeRate>> {
            unimplemented!()
        }
        async fn update_exchange_rate(
            &self,
            _f: &str,
            _t: &str,
            _r: Decimal,
        ) -> Result<crate::fx::fx_model::ExchangeRate> {
            unimplemented!()
        }
        fn get_latest_exchange_rate(&self, _f: &str, _t: &str) -> Result<Decimal> {
            unimplemented!()
        }
        fn get_exchange_rate_for_date(&self, _f: &str, _t: &str, _d: NaiveDate) -> Result<Decimal> {
            Ok(Decimal::ONE)
        }
        fn convert_currency(&self, amount: Decimal, _f: &str, _t: &str) -> Result<Decimal> {
            Ok(amount)
        }
        fn convert_currency_for_date(
            &self,
            amount: Decimal,
            _f: &str,
            _t: &str,
            _d: NaiveDate,
        ) -> Result<Decimal> {
            Ok(amount)
        }
        fn get_latest_exchange_rates(&self) -> Result<Vec<crate::fx::fx_model::ExchangeRate>> {
            unimplemented!()
        }
        async fn delete_exchange_rate(&self, _id: &str) -> Result<()> {
            unimplemented!()
        }
        async fn register_currency_pair(&self, _f: &str, _t: &str) -> Result<()> {
            Ok(())
        }
        async fn register_currency_pair_manual(&self, _f: &str, _t: &str) -> Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_calculate_deposits_timezone_boundaries() {
        let limit = ContributionLimit {
            id: "limit1".to_string(),
            group_name: "2024 Limit".to_string(),
            contribution_year: 2024,
            limit_amount: 30000.0,
            account_ids: Some("acc1".to_string()),
            start_date: None,
            end_date: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        let last_query_range = Arc::new(std::sync::Mutex::new(None));
        let act_repo = MockActivityRepository {
            activities: vec![],
            last_query_range: last_query_range.clone(),
        };

        let svc = ContributionLimitService::new(
            Arc::new(MockFxService),
            Arc::new(MockLimitRepository { limit }),
            Arc::new(act_repo),
        );

        // 1. UTC-3 (Argentina): Local 2024-01-01 00:00 -> UTC 2024-01-01 03:00
        // Local 2024-12-31 23:59:59 -> UTC 2025-01-01 02:59:59
        svc.calculate_deposits_for_contribution_limit("limit1", "USD", -180)
            .unwrap();
        let range = last_query_range.lock().unwrap().take().unwrap();
        assert_eq!(
            range.0,
            NaiveDate::from_ymd_opt(2024, 1, 1)
                .unwrap()
                .and_hms_opt(3, 0, 0)
                .unwrap()
        );
        assert_eq!(
            range.1,
            NaiveDate::from_ymd_opt(2025, 1, 1)
                .unwrap()
                .and_hms_opt(2, 59, 59)
                .unwrap()
        );

        // 2. UTC+2 (Europe/Paris): Local 2024-01-01 00:00 -> UTC 2023-12-31 22:00
        // Local 2024-12-31 23:59:59 -> UTC 2024-12-31 21:59:59
        svc.calculate_deposits_for_contribution_limit("limit1", "USD", 120)
            .unwrap();
        let range = last_query_range.lock().unwrap().take().unwrap();
        assert_eq!(
            range.0,
            NaiveDate::from_ymd_opt(2023, 12, 31)
                .unwrap()
                .and_hms_opt(22, 0, 0)
                .unwrap()
        );
        assert_eq!(
            range.1,
            NaiveDate::from_ymd_opt(2024, 12, 31)
                .unwrap()
                .and_hms_opt(21, 59, 59)
                .unwrap()
        );
    }
}
