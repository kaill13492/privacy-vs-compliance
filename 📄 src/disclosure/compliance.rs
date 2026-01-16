/// Defines what a regulator is allowed to see
pub enum ComplianceLevel {
    AmountOnly,
    AmountAndTimestamp,
}

pub struct ComplianceRequest {
    pub level: ComplianceLevel,
}

pub struct ComplianceResponse {
    pub disclosed_amount: Option<u64>,
}

pub fn comply(
    request: ComplianceRequest,
    amount: u64,
) -> ComplianceResponse {
    match request.level {
        ComplianceLevel::AmountOnly => ComplianceResponse {
            disclosed_amount: Some(amount),
        },
        _ => ComplianceResponse {
            disclosed_amount: None,
        },
    }
}
