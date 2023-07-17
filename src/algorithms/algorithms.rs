use crate::filemanager::filemanager::read_file;
use crate::objects::objects::{Transaction, Operation, Type, TypeInvestment};
use crate::investmentlib::consumeapi::{pegar_valor_btc, pegar_valor_selic};
use chrono::{Local, Datelike};


pub fn calcular_receita_despesa() -> (f64, f64) {
    let transactions =read_file();
    let month = Local::now().month();
    let receitas_fixas  = transactions.iter()
        .filter(|transacao| transacao.operation == Operation::Deposit && transacao.type_transaction == Type::FixedValue)
        .fold(0.0, |acc, transacao| acc + transacao.amount);
    let receitas_variadas  = transactions.iter()
        .filter(|transacao| transacao.operation == Operation::Deposit && transacao.type_transaction == Type::VariableValue && transacao.month.unwrap() == month)
        .fold(0.0, |acc, transacao| acc + transacao.amount);
    let despesas_fixas = transactions.iter()
        .filter(|transacao| transacao.operation == Operation::Redemption && transacao.type_transaction == Type::FixedValue)
        .fold(0.0, |acc, transacao| acc + transacao.amount);
    let despesas_variadas = transactions.iter()
        .filter(|transacao| transacao.operation == Operation::Redemption && transacao.type_transaction == Type::VariableValue && transacao.month.unwrap() == month)
        .fold(0.0, |acc, transacao| acc + transacao.amount);
    let total_receitas = receitas_fixas + receitas_variadas;
    let total_despesas = despesas_fixas + despesas_variadas;
    (total_receitas, total_despesas)
}

pub fn calcula_comunhao_bens(saldo: f64) -> f64 {
    saldo * 0.1
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
        valor_hora * horas_esperadas
    } else {
        valor_minimo
    }
}

pub fn total_de_redimentos_mensal() -> f64 {
    let investimentos = read_file();
    let total_investido = investimentos.iter()
        .filter(|transaction| transaction.type_investment.is_none() != false)
        .fold(0.0, |acc, investimento| acc + investimento.amount);
    total_investido
}

pub fn total_rendimentos() -> (f64, f64, f64) {
    let investimentos = read_file();
    let mut total_selic = 0.0;
    let mut total_rendimento_selic = 0.0;
    let mut total_bitcoin = 0.0;  
    let mut total_rendimento_bitcoin = 0.0;  
    for investimento in investimentos {
        if investimento.type_investment == Some(TypeInvestment::RendaFixa) {
            total_selic = investimento.amount + total_selic;
        } else if investimento.type_investment == Some(TypeInvestment::Bitcoin) {
            total_bitcoin = investimento.amount + total_bitcoin;
        }
    }
    let valor_btc = pegar_valor_btc();
    (total_selic, total_bitcoin, total_bitcoin * valor_btc)
}

fn calculo_rendimento(investimento: Transaction) -> (u32, f32) {
    let rendimento_anual = pegar_valor_selic();
    let r_anual = rendimento_anual / 100.0;
    let r_mensal = ((1.0 + r_anual).powf(1.0/12.0)) - 1.0;
    let month = Local::now().month();
    if investimento.month == Some(month) {
        return (0, r_mensal);
    } else {
        let months = month - investimento.month.unwrap();
        return (months, r_mensal);
    }
}



// pub fn calcular_reserva_de_emergencia() {
//     let (total_receitas, total_despesas) = calcular_receita_despesa();
//     let percentual_a_guardar = 0.55;
//     let limite_de_despesas = 0.45 * total_receitas;
//     if total_despesas > limite_de_despesas {
//         println!("Você gastou mais do que 45% da sua receita");
//         let diferenca = total_despesas - limite_de_despesas;
//         println!("Você precisa reduzir suas despesas em {} para atingir o limite de 45% da sua receita", diferenca);
//     } else {
//         println!("--------------------------");
//         println!("Você está gastando menos do que 45% da sua receita");
//         let objetivo = 12_000.0;
//         let rentabilidade_mensal = 0.04;
//         let valor_guardado_mensal = total_receitas * percentual_a_guardar;
//         let (meses, valor_guardado) = calculo_reserva_de_emergencia(valor_guardado_mensal, objetivo, rentabilidade_mensal);
//         println!("Para ter uma reserva de emergência que cubra 6 meses com 3 mil cada mês.");
//         println!("Meses para atingir o objetivo: {}", meses);
//         println!("Valor guardado no período: {}", valor_guardado);
//     }    
// }

// pub fn calcular_renda_passiva() {
//     let (total_receitas, total_despesas) = calcular_receita_despesa();
//     let percentual_a_guardar = 0.55;
//     let limite_de_despesas = 0.45 * total_receitas;
//     if total_despesas > limite_de_despesas {
//         println!("Você gastou mais do que 45% da sua receita");
//     } else {
//         println!("Você está gastando menos do que 45% da sua receita");
//         let objetivo = 10_000.0;
//         let rentabilidade_mensal = 0.04;
//         let (meses, valor_guardado) = calculo_de_renda_passiva(total_receitas, objetivo, percentual_a_guardar, rentabilidade_mensal);
//         if meses >= 12 {
//             println!("Anos para atingir o objetivo: {}", meses/12);
//         } else {
//             println!("Meses para atingir o objetivo: {}", meses);
//         }
//         println!("Valor guardado no período: R$ {:.2}", valor_guardado);
//     }
// }
