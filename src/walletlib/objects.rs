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
    pub mes: i32,
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
    pub fn new(id: i32, descricao: String, valor: f32, operacao: Operacao, tipo: Tipo, categoria: Categoria, objetivo: Option<String>, tipo_investimento: Option<TipoInvestimento>, dia: Option<i32>, mes: i32, ano: Option<i32>, valor_cdb: Option<f32>, rendimento: Option<f32>, taxa: Option<f32>, valor_cota: Option<f32>, dividendos: Option<f32>, numero_cotas: Option<f32>, valor_bitcoin: Option<f32>) -> Transacao {
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
        if let Some(_dia) = self.dia {
            println!("Dia: {:?}", self.dia);
        }
        println!("Mês: {:?}", self.mes);
        if let Some(_ano) = self.ano {
            println!("Ano: {:?}", self.ano);
        }
        println!("Categoria: {:?}", &self.categoria);
        if let Some(_objetivo) = &self.objetivo {
            println!("Objetivo: {:?}", &self.objetivo);
        }
        if let Some(_tipo_investimento) = &self.tipo_investimento {
            println!("Tipo de investimento: {:?}", self.tipo_investimento);
        }
        if let Some(_valor_cdb) = self.valor_cdb {
            println!("Valor do CDB: {:?}", self.valor_cdb);
        }
        if let Some(_rendimento) = self.rendimento {
            println!("Rendimento: {:?}", self.rendimento);
        }
        if let Some(_taxa) = self.taxa {
            println!("Taxa: {:?}", &self.taxa);
        }
        if let Some(_valor_cota) = self.valor_cota {
            println!("Valor da cota: {:?}", &self.valor_cota);
        }
        if let Some(_dividendos) = self.dividendos {
            println!("Dividendos: {:?}", &self.dividendos);
        }
        if let Some(_numero_cotas) = self.numero_cotas {
            println!("Número de cotas: {:?}", &self.numero_cotas);
        }
        if let Some(_valor_bitcoin) = self.valor_bitcoin {
            println!("Valor do bitcoin: {:?}", &self.valor_bitcoin);
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