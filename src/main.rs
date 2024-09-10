mod auth;
mod models;

use models::invoice::Invoice;
use models::customer::Customer;
use models::car::Car;

fn main() { 
    let company_name = "MB Auto";

    welcome_screen(company_name);
    auth::login_screen();

    let invoice = Invoice {
        id: 1,
        description: "Payment for consulting services".to_string(),
        amount: 500.0,
        date: "2024-09-10".to_string(),
    };

    let customer = Customer {
        id: 1,
        name: "Gabriel Choong".to_string(),
        phone: "012-4989791".to_string(),
    };

    let car = Car {
        id: 1,
        model: "Honda".to_string(),
        registration_number: "WRM 756".to_string(),
    };

    println!("{:?}", invoice);
    println!("{:?}", customer);
    println!("{:?}", car);
}

fn welcome_screen(user: &str) {
    println!("Welcome to {user} Accounting");
}

