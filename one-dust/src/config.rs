pub struct App {
    pub dry_run: bool,
    pub account_type: AccountType,
    pub dns_timeout: usize,
    pub connection_timeout: usize,
    pub data_timeout: usize,
    pub operation_timeout: usize,
    pub ip_protocol_version: usize,
}

pub enum AccountType {
    Personal,
    Business,
}
