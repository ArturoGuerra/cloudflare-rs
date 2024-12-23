use super::{Policy, Role};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Member {
    id: isize,
    policies: Vec<Policy>,
    roles: Vec<Role>,
    status: MemberStatus,
    user: MemberUserDetails,
}

#[derive(Serialize, Deserialize)]
pub struct MemberUserDetails {
    email: String,
    id: String,
    first_name: String,
    last_name: String,
    two_factor_authentication_enabled: bool,
}

#[derive(Serialize, Deserialize)]
pub enum MemberStatus {
    Accepted,
    Pending,
}
