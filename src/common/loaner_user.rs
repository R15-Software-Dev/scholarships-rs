#[derive(Clone)]
pub struct LoanerUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub has_loan: bool,
}

impl LoanerUser {
    pub fn new() -> Self {
        LoanerUser {
            first_name: "".to_string(),
            last_name: "".to_string(),
            email: "".to_string(),
            has_loan: false,
        }
    }
}
