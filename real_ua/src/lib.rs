use data::USER_AGENTS;

mod data;

pub fn get_ua() -> &'static str {
    return USER_AGENTS[fastrand::usize(..USER_AGENTS.len())];
}
