use super::RatePlan;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Subscription {
    id: String,
    currency: String,
    current_period_end: String,
    current_period_start: String,
    frequency: SubscriptionFrequency,
    price: isize,
    rate_plan: RatePlan,
    state: SubscriptionState,
}

#[derive(Serialize, Deserialize)]
pub enum SubscriptionFrequency {
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(Serialize, Deserialize)]
pub enum SubscriptionState {
    Trial,
    Provisioned,
    Paid,
    AwaitingPayment,
    Canceled,
    Failed,
    Expired,
}

#[derive(Serialize, Deserialize)]
pub struct SubscriptionComponent {
    default: isize,
    name: String,
    price: isize,
    value: isize,
}

#[derive(Serialize, Deserialize)]
pub struct SubscriptionZone {
    id: String,
    name: String,
}
