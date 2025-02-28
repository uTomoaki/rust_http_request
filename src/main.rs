use reqwest::Error;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize)]
struct Customer {
    no: i32,
    customer_no: i32,
}

async fn fetch_data(url: &str) -> Result<Vec<Customer>, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .timeout(Duration::from_secs(10))
        .send()
        .await?;

    let customers = response.json::<Vec<Customer>>().await?;

    Ok(customers)
}

#[tokio::main]
async fn main() {
    let url = "http://localhost:4010/customers";

    match fetch_data(url).await {
        Ok(customers) => {
            for customer in customers {
                println!("No: {}", customer.no);
                println!("Customer No: {}", customer.customer_no);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
