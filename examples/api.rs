use envconfig::Envconfig;
use mercury_bank_api::apis::configuration::Configuration;
use mercury_bank_api::apis::default_api;
use mercury_bank_api::models::Accounts;

#[derive(Debug, Envconfig)]
struct Config {
    #[envconfig(from = "MERCURY_API_KEY")]
    pub mercury_api_key: String,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // set .env values;
    dotenv::dotenv().ok();

    match Config::init_from_env() {
        Ok(config) => {
            println!("Found config: {:?}", config);
            let mut api_config = Configuration::new();
            api_config.basic_auth = Some((config.mercury_api_key, None));

            println!("api config: {:?}", api_config.basic_auth);

            let Accounts { accounts } = default_api::list_accounts(&api_config)
                .await
                .expect("failed to list accounts");

            println!("Found Accounts: {:?}", accounts);

            for account in accounts.iter() {
                let account_id = account.id.clone().unwrap_or_default();
                let transactions = default_api::list_transactions(
                    &api_config,
                    &account_id,
                    None,                             // limit: Option<i32>,
                    None,                             // offset: Option<i32>,
                    None,                             // status: Option<&str>,
                    Some(String::from("2020-12-31")), // start: Option<String>,
                    Some(String::from("2021-01-02")), // end: Option<String>,
                    None,                             // search: Option<&str>,
                )
                .await
                .expect(&format!(
                    "Failed to find transactions for account: {:?}",
                    account_id
                ));

                println!("Found Transactions: {:#?}", transactions);
            }
        }
        Err(e) => {
            eprintln!("Config error: {:?}", e);
        }
    }

    Ok(())
}
