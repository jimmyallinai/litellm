use jiff::Timestamp;
use litellm_python_compat::Value;

use super::{
    bag::Bag,
    budget_limit::CachedBudgetLimit,
    member::CachedMember,
    object_permission::CachedObjectPermission,
    record::{Field, fields, record, required},
};
use crate::{DecodeError, FromValue, IntoValue};

record! {
    /// A team's model alias table, as `LiteLLM_ModelTable` dumps it nested in the team.
    pub struct CachedModelTable {
        id: Option<i64>,
        #[redact(with = "[REDACTED]")]
        model_aliases: Option<Value>,
        created_by: String = required!("created_by"),
        updated_by: String = required!("updated_by"),
        #[redact(with = "[REDACTED]")]
        team: Option<Value>,
    }
}

record! {
    /// `LiteLLM_TeamTableCachedObj.model_dump(mode="json")`, the value under
    /// [`AuthObject::Team`](crate::auth_cache::AuthObject::Team).
    pub struct CachedTeam {
        team_alias: Option<String>,
        team_id: String = required!("team_id"),
        organization_id: Option<String>,
        admins: Vec<String>,
        members: Vec<String>,
        members_with_roles: Vec<CachedMember>,
        team_member_permissions: Option<Vec<String>>,
        metadata: Option<Bag>,
        tpm_limit: Option<i64>,
        rpm_limit: Option<i64>,
        tpd_limit: Option<i64>,
        max_budget: Option<f64>,
        soft_budget: Option<f64>,
        budget_duration: Option<String>,
        budget_limits: Option<Vec<CachedBudgetLimit>>,
        models: Vec<String>,
        blocked: bool,
        router_settings: Option<Bag>,
        access_group_ids: Option<Vec<String>>,
        default_team_member_models: Option<Vec<String>>,
        spend: Option<f64>,
        max_parallel_requests: Option<i64>,
        budget_reset_at: Option<Timestamp>,
        model_id: Option<i64>,
        model_spend: Option<Bag> = Some(Bag::default()),
        model_max_budget: Option<Bag> = Some(Bag::default()),
        policies: Option<Vec<String>>,
        allow_team_guardrail_config: Option<bool> = Some(false),
        litellm_model_table: Option<CachedModelTable>,
        object_permission: Option<CachedObjectPermission>,
        object_permission_id: Option<String>,
        updated_at: Option<Timestamp>,
        created_at: Option<Timestamp>,
        last_refreshed_at: Option<f64>,
    }
}

#[cfg(test)]
mod tests {
    use litellm_python_compat::json;
    use rstest::rstest;

    use super::CachedTeam;
    use crate::{FromValue, auth_cache::Bag};

    #[rstest]
    fn a_team_entry_needs_its_id_and_takes_the_models_defaults_for_the_rest() {
        let team = CachedTeam::from_value(&json::loads(r#"{"team_id": "team"}"#).unwrap()).unwrap();

        assert_eq!(
            (
                team.team_id.as_str(),
                team.blocked,
                team.models.len(),
                team.model_spend,
                team.model_max_budget,
                team.allow_team_guardrail_config,
                CachedTeam::from_value(&json::loads("{}").unwrap()).is_err(),
            ),
            (
                "team",
                false,
                0,
                Some(Bag::default()),
                Some(Bag::default()),
                Some(false),
                true,
            )
        );
    }
}
