//! Run commands

use std::{collections::HashMap, sync::Arc};

use ethers::{
    contract::Contract,
    core::abi::Abi,
    prelude::{Address, Http, Provider, Uint8},
};

use crate::{format_vec, parser};

pub struct EvalCommand {
    client: Arc<Provider<Http>>,
}

impl EvalCommand {
    pub fn new(node_url: String) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            client: Arc::new(Provider::<Http>::try_from(node_url)?),
        })
    }

    pub async fn eval(&self, command: &parser::Command<'_>) -> Result<String, &str> {
        match command {
            parser::Command::Token { address: addr } => {
                self.eval_token(addr).await.or(Err("Failed to fetch Token"))
            }
            parser::Command::Abi { address: addr } => self
                .eval_abi(addr)
                .await
                .or(Err("Unable to parse `Abi` command")),
            _ => Err("Unknonwn command or not yet supported"),
        }
    }

    async fn eval_abi(
        &self,
        address: &parser::Address<'_>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let contract = self.fetch_contract(address).await?;
        let abi = contract.abi();

        let functions: Vec<String> = abi
            .functions
            .iter()
            .flat_map(|(_, function)| function.iter().map(|fun| fun.signature()))
            .collect();

        let events: Vec<String> = abi
            .events
            .iter()
            .flat_map(|(_, event)| event.iter().map(|eve| eve.name.clone()))
            .collect();

        Ok(format!(
            "\nFunctions:\n{}\n\nEvents:\n{}",
            format_vec!(functions, 2),
            format_vec!(events, 2)
        ))
    }

    async fn eval_token(
        &self,
        address: &parser::Address<'_>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let contract = self.fetch_contract(address).await?;

        let name = contract.method::<_, String>("name", ())?.call().await?;
        let symbol = contract.method::<_, String>("symbol", ())?.call().await?;
        let decimals: u8 = contract
            .method::<_, Uint8>("decimals", ())?
            .call()
            .await?
            .into();

        Ok(format!(
            "Name: {}\tSymbol: {}\n  Address: {}\n  Decimals: {}\n",
            name, symbol, address.address, decimals
        ))
    }

    async fn fetch_contract(
        &self,
        address: &parser::Address<'_>,
    ) -> Result<Contract<Provider<Http>>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.bscscan.com/api?module=contract&action=getabi&address={}",
            address.address
        );
        println!(
            "Fetching abi of {} from {}...",
            &address.address[0..8],
            &url[0..24]
        );

        let abi = reqwest::get(url)
            .await?
            .json::<HashMap<String, String>>()
            .await?;

        let abi: Abi = serde_json::from_str(abi.get("result").unwrap())?;
        Ok(Contract::new(
            address.address.parse::<Address>()?,
            abi,
            self.client.clone(),
        ))
    }
}
