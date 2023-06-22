use crate::walletlib::files::ler_arquivo_json;
use crate::walletlib::objects::{Operacao, Transacao, Categoria, Tipo};
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
    println!("Receitas Fixas : {}", despesas_fixas);
    let total_receitas = receitas_fixas + receitas_variadas;
    let total_despesas = despesas_fixas + despesas_variadas;
    (total_receitas, total_despesas)
}

pub fn calcula_comunhao_bens(saldo: f32) -> f32 {
    saldo * 0.1
}

pub fn listar_investimentos(transactions: Vec<Transacao>) -> Vec<Transacao> {
    transactions.into_iter()
        .filter(|transacao| transacao.categoria == Categoria::Investimento)
        .collect::<Vec<Transacao>>()
}

pub fn calculo_reserva_de_emergencia(valor_guardado_mensal: f32, objetivo: f32, rentabilidade_mensal: f32) -> (i32, f32) {
    let _rentabilidade_anual = ((1.0 + rentabilidade_mensal)/12.0)-1.0;
    let mut valor_guardado = 0.0;
    let meses = (0..).take_while(|_| {
        valor_guardado <= objetivo && {
            valor_guardado = (valor_guardado + valor_guardado_mensal) * (1.0 + rentabilidade_mensal);
            true
        }
    }).count() as i32;
    (meses, valor_guardado)
}

pub fn calculos_fii(transaction: &Transacao) -> f32 {
    let dividendos = transaction.dividendos.unwrap();
    let numero_cotas = transaction.numero_cotas.unwrap();
    let valor_cota = transaction.valor_cota.unwrap();
    let valor_investido = numero_cotas * valor_cota;
    let _valor_dividendos = valor_investido * dividendos;
    let magic_number = (valor_cota / dividendos) - numero_cotas;
    let _valor_bola_de_neve = valor_cota * magic_number;
    let valor_desejado = 3_000.0;
    let valor_necessario = (valor_desejado / dividendos) * magic_number;
    valor_necessario
}

pub fn calculo_de_renda_passiva(receita_atual: f32, renda_passiva_desejada: f32, economia_atual_em_percentual: f32, rendimento_da_renda_passiva: f32) -> (i32, f32){
    let economia_atual_em_real = receita_atual * economia_atual_em_percentual;
    let valor_necessario = (renda_passiva_desejada * 12.0) / rendimento_da_renda_passiva;
    let mut anos = 0;
    let mut valor_atual = 0.0;
    while valor_atual <= valor_necessario {
        valor_atual = valor_atual + (economia_atual_em_real * 12.0) + (valor_atual * rendimento_da_renda_passiva);
        anos += 1;
    }
    (anos, valor_atual)
}

pub fn calculo_pericia(horas_esperadas: i32) -> i32 {
    let valor_hora = 450;
    let valor_minimo = 4_500;
    if horas_esperadas > 10 {
        let valor_esperado = valor_hora * horas_esperadas;
        return valor_esperado;
    } else {
        valor_minimo
    }
}

pub fn total_de_redimentos_mensal(transactions: Vec<Transacao>) -> f32 {
    let investimentos = listar_investimentos(transactions);
    let total_investido = investimentos.iter()
        .fold(0.0, |acc, transacao| acc + transacao.valor);
    total_investido
}