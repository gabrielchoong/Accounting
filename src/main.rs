mod auth;
mod models;

use crate::models::customer::Customer;
use crate::models::car::Car;
use crate::models::invoice_details::InvoiceDetails;
use crate::models::invoice::Invoice;

fn main() { 
    let company_name = "MB Auto";

    welcome_screen(company_name);
    // auth::login_screen();

    let customer = Customer {
        id: 1,
        name: String::from("John Doe"),
        phone: String::from("123-456-7890"),
    };

    let car = Car {
        id: 1,
        model: String::from("Toyota Corolla"),
        registration_number: String::from("XYZ 1234"),
    };

    let detail1 = InvoiceDetails::new(
        String::from("Oil Change"),
        1,
        29.99
    );

    let detail2 = InvoiceDetails::new(
        String::from("Tire Rotation"),
        1,
        15.00
    );
    
    let invoice = Invoice {
        id: 1,
        date: String::from("2024-10-04"),
        details: vec![detail1, detail2],
        customer,
        car,
    };

    println!("{:?}", invoice);
}

fn welcome_screen(user: &str) {
    println!("Welcome to {user} Accounting");
}

