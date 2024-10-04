use crate::models::invoice_details::InvoiceDetails;
use crate::models::customer::Customer;
use crate::models::car::Car;

#[derive(Debug)]
pub struct Invoice {
    pub id: i32,
    pub date: String,
    pub details: Vec<InvoiceDetails>,
    pub customer: Customer,
    pub car: Car
}

