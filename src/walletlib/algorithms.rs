use crate::walletlib::files::ler_arquivo_json;
use crate::walletlib::objects::{Operacao, Tipo};
use chrono::{Local, Datelike};


pub fn calcular_receita_despesa() -> (f32, f32) {
    let transactions = ler_arquivo_json();
    let month = Local::now().month();
    let receitas_fixas  = transactions.iter()
        .filter(|transacao| transacao.operacao == Operacao::Receita && transacao.tipo == Tipo::ValorFixo)
        .fold(0.0, |acc, transacao| acc + transacao.valor);
    let receitas_variadas  = transactions.iter()
        .filter(|transacao| transacao.operacao == Operacao::Receita && transacao.tipo == Tipo::ValorVariavel && transacao.mes.unwrap() == month)
        .fold(0.0, |acc, transacao| acc + transacao.valor);
    let despesas_fixas = transactions.iter()
        .filter(|transacao| transacao.operacao == Operacao::Despesa && transacao.tipo == Tipo::ValorFixo)
        .fold(0.0, |acc, transacao| acc + transacao.valor);
    let despesas_variadas = transactions.iter()
        .filter(|transacao| transacao.operacao == Operacao::Despesa && transacao.tipo == Tipo::ValorVariavel && transacao.mes.unwrap() == month)
        .fold(0.0, |acc, transacao| acc + transacao.valor);
    let total_receitas = receitas_fixas + receitas_variadas;
    let total_despesas = despesas_fixas + despesas_variadas;
    (total_receitas, total_despesas)
}

pub fn calcula_comunhao_bens(saldo: f32) -> f32 {
    saldo * 0.1
}