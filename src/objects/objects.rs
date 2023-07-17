use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum Operation {
    #[default]
    Redemption,
    Deposit,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum Type {
    #[default]
    FixedValue,
    VariableValue,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum TypeInvestment {
    #[default]
    RendaFixa,
    Bitcoin,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Transaction {
    pub id: i32,
    pub description: String,
    pub amount: f64,
    pub operation: Operation,
    pub type_transaction: Type,
    pub type_investment: Option<TypeInvestment>,
    pub month: Option<u32>,
    pub year: Option<u32>,
}

// Implementing Wallet trait for Transaction
impl Transaction {
    pub fn list(&self) {
        println!("-------------------------");
        println!("Id: {:?}", &self.id);
        println!("Transaction details: {:?}", self.description);
        println!("Amount: {:?}", self.amount);
        println!("Operation: {:?}", self.operation);
        println!("Type: {:?}", self.type_transaction);
        println!("-------------------------");
    }

    pub fn details(&self) {
        println!("-------------------------");
        println!("Id: {:?}", self.id);
        println!("Transaction details: {:?}", self.description);
        println!("Amount: {:?}", self.amount);
        println!("Operation: {:?}", self.operation);
        println!("Type: {:?}", self.type_transaction);
        if self.type_investment.is_none() == false {
            println!("Type Investment: {:?}", self.type_investment);
        }
        if self.month.is_none() == false {
            println!("Month: {:?}", self.month);
        }
        if self.year.is_none() == false {
            println!("Year: {:?}", self.year.unwrap());
        }
        println!("-------------------------");
    }
}
