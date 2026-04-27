use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensePlan {
    pub id: String,
    pub name: String,
    pub description: String,
    pub suggested_duration: i64,
    pub price_sol: f64,
}

impl LicensePlan {
    pub fn get_all() -> Vec<Self> {
        vec![
            Self {
                id: "basic-plan".to_string(),
                name: "Basic Plan".to_string(),
                description: "Basic features for small projects".to_string(),
                suggested_duration: 30,
                price_sol: 0.1,
            },
            Self {
                id: "premium-plan".to_string(),
                name: "Premium Plan".to_string(),
                description: "Advanced features for growing businesses".to_string(),
                suggested_duration: 90,
                price_sol: 0.5,
            },
            Self {
                id: "enterprise-plan".to_string(),
                name: "Enterprise Plan".to_string(),
                description: "Full features for large organizations".to_string(),
                suggested_duration: 365,
                price_sol: 2.0,
            },
            Self {
                id: "developer-plan".to_string(),
                name: "Developer Plan".to_string(),
                description: "For individual developers".to_string(),
                suggested_duration: 30,
                price_sol: 0.05,
            },
            Self {
                id: "startup-plan".to_string(),
                name: "Startup Plan".to_string(),
                description: "For early-stage startups".to_string(),
                suggested_duration: 180,
                price_sol: 1.0,
            },
        ]
    }

    pub fn get_ids() -> Vec<String> {
        Self::get_all().iter().map(|p| p.id.clone()).collect()
    }

    pub fn get_names() -> Vec<String> {
        Self::get_all().iter().map(|p| p.name.clone()).collect()
    }

    pub fn find_by_name(name: &str) -> Option<Self> {
        Self::get_all().into_iter().find(|p| p.name == name)
    }
}
