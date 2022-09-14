use near_sdk::{borsh::{self, BorshDeserialize, BorshSerialize}};

#[allow(unused_imports)]
use near_sdk::{env, PromiseIndex, near_bindgen};
use near_sdk::serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

pub type AccountId = String;

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Company {
    id: u32,
    owner: String,
    name: String,
    location: String,
    assets: Vec<Asset>,
    inventories: Vec<Inventory>,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Asset{
    name: String,
    serial: String,
    company: String,
    status: String
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Inventory {
    date_taken: String,
    serial: String,
    status: String,
}

#[near_bindgen]
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct Contract{
    companies: Vec<Company>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self{
        let companies: Vec<Company> = Vec::new();
        Contract {
            companies
        }
    }

    

    pub fn new_company(&mut self, owner: String, name: String, location: String){
        let id = self.companies.len() as u32;
        let company = Company {
            id: id,
            owner: owner.to_string(),
            name: name.to_string(),
            location: location.to_string(),
            assets: vec![],
            inventories: vec![],
        };

        self.companies.push(company);   
        env::log_str("company added successfully");
    }

    pub fn new_asset(&mut self, name: String, serial: String, company_name: String){
        let companies = &mut self.companies;
        companies.into_iter().for_each(|company|{
          if company.name == company_name{
            let asset = Asset {
                name: name.to_string(),
                serial: serial.to_string(),
                company: company_name.to_string(),
                status: "good".to_string(),
            };
            company.assets.push(asset);
          }else{
            env::log_str("company not found");
          }  
        })
    }

    pub fn edit_asset(&mut self, name: String, serial: String, company_name: String){
        let companies = &mut self.companies;
        let mut company_count: u32 = 0;
        companies.into_iter().for_each(|company|{
          if company.name == company_name{
            company_count += 1;
            let assets = &mut company.assets;
            let mut asset_count = 0;
            assets.into_iter().for_each(|asset|{
                if asset.serial == serial {
                    asset_count += 1;
                    asset.name = name.to_string();
                    asset.company = company_name.to_string();
                    asset.serial = serial.to_string();
                }
            });
            if asset_count < 1 {
                env::log_str("no asset found with that serial");
            }
          }
        });
        if company_count < 1 {
            env::log_str("no company found with that name");
        }
    }

    pub fn count_companies(&self) -> usize{
        let companies = &self.companies;
        companies.len()
    }

    pub fn count_assets(&self, company_name: String) -> usize{
        let companies = &self.companies;
        let mut company_count = 0;
        let mut asset_count = 0;
        companies.into_iter().for_each(|company|{
            if company.name == company_name{
                company_count += 1;
                let assets = &company.assets;
                asset_count = assets.len();
            }
        });

        return asset_count;

    }

    pub fn take_inventories(&mut self, company_name: String, serial: String, status: String){
        let companies = &mut self.companies;
        let mut company_count: u32 = 0;
        companies.into_iter().for_each(|company|{
          if company.name == company_name{
            company_count += 1;
            

            let assets = &mut company.assets;
            let mut asset_count: u32 = 0;
            assets.into_iter().for_each(|asset|{
                if asset.serial == serial{
                    asset_count += 1;
                    let now: DateTime<Utc> = Utc::now();
                    let inventory = Inventory {
                        date_taken: now.format("%a %b %e %T %Y").to_string(),
                        serial: serial.to_string(),
                        status: status.to_string(),
                    };
                    company.inventories.push(inventory);
                    asset.status = status.to_string();
                    env::log_str("asset inventory successfull")
                }
            });
            if asset_count < 1 {
                env::log_str("no asset found with that serial number");
            }
          }
        });

        if company_count < 1 {
            env::log_str("no company found with that name");
        }
    }

    pub fn count_inventories(&self, company_name: String)->usize{
         let companies = &self.companies;
        let mut company_count = 0;
        let mut inventory_count = 0;
        companies.into_iter().for_each(|company|{
            if company.name == company_name{
                company_count += 1;
                let inventories = &company.inventories;
                inventory_count = inventories.len();
            }
        });

        return inventory_count;
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId};

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.signer_account_id(predecessor);
        builder
    }

    #[test]
    #[allow(non_snake_case)]
    fn regCompany(){
        let clyde = AccountId::new_unchecked("clyde.testnet".to_string());
        let context = get_context(felix.clone());
        testing_env!(context.build());

        let mut contract = Contract::new();
        contract.new_company("clyde.testnet".to_string(), "Kylastroke".to_string(), "kisumu".to_string());
        assert_eq!(contract.count_companies(), 1);
    }

    // test for asset registration
    #[test]
    fn reg_asset(){
        let clyde = AccountId::new_unchecked("clyde.testnet".to_string());
        let context = get_context(felix.clone());
        testing_env!(context.build());

        let mut contract = Contract::new();
        contract.new_company("clyde.testnet".to_string(), "Kylastroke".to_string(), "kisumu".to_string());
        contract.new_asset("laptop".to_string(), "1234".to_string(), "Kylastroke".to_string());
        assert_eq!(contract.count_assets("Kylastroke"), 1);
    }

    #[test]
    fn test_inventories(){
        let clyde = AccountId::new_unchecked("clyde.testnet".to_string());
        let context = get_context(felix.clone());
        testing_env!(context.build());

        let mut contract = Contract::new();
        contract.new_company("clyde.testnet".to_string(), "Kylastroke".to_string(), "kisumu".to_string());
        contract.new_asset("laptop".to_string(), "1234".to_string(), "Kylastroke".to_string());
        contract.take_inventories("Kylastroke".to_string(), "1234".to_string(), "good".to_string());
        assert_eq!(contract.count_inventories("Kylastroke"), 1);
    }
}

