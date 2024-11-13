use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RuleLifetime {
    Matches(u64),
}
