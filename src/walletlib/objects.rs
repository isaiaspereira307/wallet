use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Transacao {
    pub id: i32,
    pub descricao: String,
    pub valor: f32,
    pub operacao: Operacao,
    pub tipo: Tipo,
    pub categoria: Categoria,
    pub objetivo: Option<String>,
    pub tipo_investimento: Option<TipoInvestimento>,
    pub dia: Option<i32>,
    pub mes: Option<u32>,
    pub ano: Option<i32>,
    pub valor_cdb: Option<f32>,
    pub rendimento: Option<f32>,
    pub taxa: Option<f32>,
    pub valor_cota: Option<f32>,
    pub dividendos: Option<f32>,
    pub numero_cotas: Option<f32>,
    pub valor_bitcoin: Option<f32>,
}

impl Transacao {
    pub fn new(id: i32, descricao: String, valor: f32, operacao: Operacao, tipo: Tipo, categoria: Categoria, objetivo: Option<String>, tipo_investimento: Option<TipoInvestimento>, dia: Option<i32>, mes: Option<u32>, ano: Option<i32>, valor_cdb: Option<f32>, rendimento: Option<f32>, taxa: Option<f32>, valor_cota: Option<f32>, dividendos: Option<f32>, numero_cotas: Option<f32>, valor_bitcoin: Option<f32>) -> Transacao {
        Transacao {
            id: id,
            descricao: descricao,
            valor: valor,
            operacao: operacao,
            tipo: tipo,
            categoria: categoria,
            objetivo: objetivo,
            tipo_investimento: tipo_investimento,
            dia: dia,
            mes: mes,
            ano: ano,
            valor_cdb: valor_cdb,
            rendimento: rendimento,
            taxa: taxa,
            valor_cota: valor_cota,
            dividendos: dividendos,
            numero_cotas: numero_cotas,
            valor_bitcoin: valor_bitcoin,
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
        if self.dia.is_none() == false {
            println!("Dia: {:?}", self.dia);
        }
        if self.mes.is_none() == false {
            println!("Mês: {:?}", self.mes);
        }
        if self.ano.is_none() == false {
            println!("Ano: {:?}", self.ano.unwrap());
        }
        println!("Categoria: {:?}", self.categoria);
        if self.objetivo.is_none() == false {
            println!("Objetivo: {:?}", self.objetivo.as_ref().unwrap());
        }
        if self.tipo_investimento.is_none() == false {
            println!("Tipo de investimento: {:?}", self.tipo_investimento.as_ref().unwrap());
        }
        if self.valor_cdb.is_none() == false {
            println!("Valor do CDB: {:?}", self.valor_cdb.unwrap());
        }
        if self.rendimento.is_none() == false {
            println!("Rendimento: {:?}", self.rendimento.unwrap());
        }
        if self.taxa.is_none() == false {
            println!("Taxa: {:?}", self.taxa.unwrap());
        }
        if self.valor_cota.is_none() == false {
            println!("Valor da cota: {:?}", self.valor_cota.unwrap());
        }
        if self.dividendos.is_none() == false {
            println!("Dividendos: {:?}", self.dividendos.unwrap());
        }
        if self.numero_cotas.is_none() == false {
            println!("Número de cotas: {:?}", self.numero_cotas.unwrap());
        }
        if self.valor_bitcoin.is_none() == false {
            println!("Valor do bitcoin: {:?}", self.valor_bitcoin.unwrap());
        }
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
    Investimento,
    #[default] Outros,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TipoInvestimento {
    Cdb,
    Fii,
    Bitcoin,
    Nenhum
}