use jiff::Timestamp;
use litellm_python_compat::Value;

use super::record::{Field, fields};
use crate::{DecodeError, FromValue, IntoValue};

/// One window of a team's `budget_limits`, as `BudgetLimitEntry` dumps it. The model leaves
/// `window_start` out of its dump until the window has one, so this is no
/// [`record!`](super::record::record), which writes every field.
#[derive(Clone, Debug, PartialEq)]
pub struct CachedBudgetLimit {
    pub budget_duration: String,
    pub max_budget: f64,
    pub reset_at: Option<Timestamp>,
    pub window_start: Option<Timestamp>,
}

impl FromValue for CachedBudgetLimit {
    fn from_value(value: &Value) -> Result<Self, DecodeError> {
        let Value::Dict(entries) = value else {
            return Err(DecodeError::Shape("CachedBudgetLimit"));
        };
        let named = fields(entries);
        let read = |name: &'static str| named.get(name).copied().unwrap_or(&Value::None);
        Ok(Self {
            budget_duration: Field::read(read("budget_duration"))
                .ok_or(DecodeError::Shape("budget_duration"))?,
            max_budget: Field::read(read("max_budget")).ok_or(DecodeError::Shape("max_budget"))?,
            reset_at: Field::read(read("reset_at")).ok_or(DecodeError::Shape("reset_at"))?,
            window_start: Field::read(read("window_start"))
                .ok_or(DecodeError::Shape("window_start"))?,
        })
    }
}

impl IntoValue for CachedBudgetLimit {
    fn into_value(self) -> Value {
        let window_start = self
            .window_start
            .map(|at| ("window_start".to_owned(), Field::write(at)));
        Value::Dict(
            [
                (
                    "budget_duration".to_owned(),
                    Field::write(self.budget_duration),
                ),
                ("max_budget".to_owned(), Field::write(self.max_budget)),
                ("reset_at".to_owned(), Field::write(self.reset_at)),
            ]
            .into_iter()
            .chain(window_start)
            .map(|(name, value)| (Value::Str(name), value))
            .collect(),
        )
    }
}

impl Field for CachedBudgetLimit {
    fn read(value: &Value) -> Option<Self> {
        Self::from_value(value).ok()
    }

    fn write(self) -> Value {
        self.into_value()
    }
}
