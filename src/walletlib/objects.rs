use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Transacao {
    pub id: i32,
    pub descricao: String,
    pub valor: f32,
    pub operacao: Operacao,
    pub tipo: Tipo,
    pub categoria: Categoria,
    pub mes: Option<u32>,
    pub ano: Option<i32>,
}

impl Transacao {
    pub fn new(id: i32, descricao: String, valor: f32, operacao: Operacao, tipo: Tipo, categoria: Categoria, mes: Option<u32>, ano: Option<i32>) -> Transacao {
        Transacao {
            id: id,
            descricao: descricao,
            valor: valor,
            operacao: operacao,
            tipo: tipo,
            categoria: categoria,
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
        println!("Tipo: {:?}", &self.tipo);
        println!("Mês: {:?}", &self.mes);
        println!("Categoria: {:?}", &self.categoria);
        println!("-------------------------");
    }

    pub fn detalhe(&self) {
        println!("-------------------------");
        println!("Id: {:?}", self.id);
        println!("Descrição: {:?}", self.descricao);
        println!("Valor: {:?}", self.valor);
        println!("Operação: {:?}", self.operacao);
        println!("Tipo: {:?}", self.tipo);
        if self.mes.is_none() == false {
            println!("Mês: {:?}", self.mes);
        }
        if self.ano.is_none() == false {
            println!("Ano: {:?}", self.ano.unwrap());
        }
        println!("Categoria: {:?}", self.categoria);
        println!("-------------------------");
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum Operacao {
    #[default] Despesa,
    Receita,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum Tipo {
    #[default] ValorFixo,
    ValorVariavel,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum Categoria {
    Salario,
    RendaExtra,
    Alimentacao,
    Transporte,
    Lazer,
    Educacao,
    Saude,
    #[default] Outros,
}