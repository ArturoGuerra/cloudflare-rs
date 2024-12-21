pub mod batch_dns_records;
pub mod create_dns_record;
pub mod delete_dns_record;
pub mod dns_record_details;
pub mod export_dns_records;
pub mod import_dns_records;
pub mod list_dns_records;
pub mod overwrite_dns_record;
pub mod scan_dns_records;
pub mod update_dns_record;

mod data_structures;

pub use data_structures::*;
