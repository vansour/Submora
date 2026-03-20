#[derive(Clone, Debug, PartialEq)]
pub struct UserSummary {
    pub selected_username: String,
}

impl UserSummary {
    pub fn build(selected_username: Option<String>) -> Option<Self> {
        let selected_username = selected_username?;

        Some(Self { selected_username })
    }
}
