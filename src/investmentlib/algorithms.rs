use crate::investmentlib::files::ler_arquivo_json;
use crate::investmentlib::structsinvestiments::{Operacao, Investimento, TipoInvestimento};
use crate::investmentlib::consumeapi::{pegar_valor_btc, pegar_valor_selic};
use chrono::{Local, Datelike};


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

pub fn total_de_redimentos_mensal() -> f32 {
    let investimentos = ler_arquivo_json();
    let total_investido = investimentos.iter()
        .fold(0.0, |acc, investimento| acc + investimento.valor);
    total_investido
}

pub fn total_rendimentos() {
    let investimentos = ler_arquivo_json();
    let mut total_selic = 0.0;
    let mut total_rendimento_selic = 0.0;
    let mut total_bitcoin = 0.0;  
    let mut total_rendimento_bitcoin = 0.0;  
    for investimento in investimentos {
        if investimento.tipo_investimento == TipoInvestimento::Selic {
            total_selic = investimento.valor + total_selic;
        } else if investimento.tipo_investimento == TipoInvestimento::Bitcoin {
            total_bitcoin = investimento.valor + total_bitcoin;
        }
    }
}

fn calculo_rendimento(investimento: Investimento) -> (u32, f32) {
    let rendimento_anual = pegar_valor_selic();
    let r_anual = rendimento_anual / 100.0;
    let r_mensal = ((1.0 + r_anual).powf(1.0/12.0)) - 1.0;
    let month = Local::now().month();
    if investimento.mes == month {
        return (0, r_mensal);
    }
    let meses = month - investimento.mes;
    return (meses, r_mensal);
}
