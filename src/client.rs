use anyhow::anyhow;
use chrono::{DateTime, Local, TimeZone, Utc};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::env;

fn get_api_key() -> String {
    env::var("NNC_API_KEY").expect(
        "couldn't get NotNetCoin API key - make sure to set the `NNC_API_KEY` environment variable",
    )
}

// what a mess
#[derive(Deserialize, Debug)]
struct UserInfo {
    id: String,
    username: String,
    discriminator: String,

    #[serde(rename = "apiKey")]
    api_key: String,
    balance: u32,
}

#[derive(Serialize)]
struct TransactionRequest {
    to: String,
    amount: u32,
}

#[derive(Deserialize, Debug)]
struct TransactionInfo {
    id: String,
    from: String,
    to: String,
    amount: u32,
}

#[derive(Deserialize, Debug)]
struct RequestError {
    error: String,
}

#[derive(Deserialize, Debug)]
struct DailyError {
    error: String,
    reset: i64,
}

#[derive(Deserialize, Debug)]

struct DailyInfo {
    amount: u8,
}

pub struct Client {
    http_client: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> Self {
        Client {
            http_client: reqwest::blocking::Client::new(),
        }
    }

    pub fn check_balance(&mut self) -> anyhow::Result<()> {
        let api_key = get_api_key();

        let res = self
            .http_client
            .get("https://nnc.n2.pm/me")
            .header("Authorization", api_key)
            .send()?;

        if res.status() != StatusCode::OK {
            let err: RequestError = res.json()?;
            return Err(anyhow!("{}", err.error));
        }

        let json: UserInfo = res.json()?;

        println!("You have {} NNC.", json.balance);

        Ok(())
    }

    pub fn send_money(&mut self, to: String, amount: u32) -> anyhow::Result<()> {
        let api_key = get_api_key();

        let req = TransactionRequest { to, amount };

        let res = self
            .http_client
            .post("https://nnc.n2.pm/transactions")
            .header("Authorization", api_key)
            .json(&req)
            .send()?;

        if res.status() != StatusCode::OK {
            let err: RequestError = res.json()?;
            return Err(anyhow!("{}", err.error));
        }

        let json: TransactionInfo = res.json()?;

        println!("Sent {} NNC to {}.", json.amount, json.to);

        Ok(())
    }

    pub fn claim_daily(&mut self) -> anyhow::Result<()> {
        let api_key = get_api_key();

        let res = self
            .http_client
            .get("https://nnc.n2.pm/daily")
            .header("Authorization", api_key)
            .send()?;

        if res.status() == StatusCode::BAD_REQUEST {
            let err: DailyError = res.json()?;
            let dt = Utc.timestamp(err.reset / 1000, 0);
            let what: DateTime<Local> = DateTime::from(dt);
            return Err(anyhow!(
                "Not yet ready. Try again at {}.",
                what.to_rfc2822()
            ));
        }

        if res.status() == 401 {
            let err: RequestError = res.json()?;
            return Err(anyhow!("{}", err.error));
        }

        let json: DailyInfo = res.json()?;
        println!("You got {} NNC.", json.amount);

        Ok(())
    }
}
