use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Investimento {
    pub id: i32,
    pub descricao: String,
    pub valor: f32,
    pub operacao: Operacao,
    pub tipo_investimento: TipoInvestimento,
    pub mes: u32,
    pub ano: u32,
}

impl Investimento {
    pub fn new(id: i32, descricao: String, valor: f32, operacao: Operacao, tipo_investimento: TipoInvestimento, mes: u32, ano: u32) -> Investimento {
        Investimento {
            id: id,
            descricao: descricao,
            valor: valor,
            operacao: operacao,
            tipo_investimento: tipo_investimento,
            mes: mes,
            ano: ano,
        }
    }
    
    pub fn listar(&self) {
        println!("-------------------------");
        println!("Id: {:?}", &self.id);
        println!("Descrição: {:?}", &self.descricao);
        println!("Valor: {:?}", &self.valor);
        println!("Operação: {:?}", &self.operacao);
        println!("Mês: {:?}", &self.mes);
        println!("-------------------------");
    }

    pub fn detalhe(&self) {
        println!("-------------------------");
        println!("Id: {:?}", self.id);
        println!("Descrição: {:?}", self.descricao);
        println!("Valor: {:?}", self.valor);
        println!("Operação: {:?}", self.operacao);
        println!("Mês: {:?}", self.mes);
        println!("Ano: {:?}", self.ano);
        println!("Tipo de investimento: {:?}", self.tipo_investimento);
        println!("-------------------------");
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum Operacao {
    #[default] Aplicao,
    Resgate,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum TipoInvestimento {
    #[default] Selic,
    Bitcoin
}