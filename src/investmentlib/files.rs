use std::fs::File;
use std::io::prelude::*;
use crate::investmentlib::structsinvestiments::{Investimento, Operacao, TipoInvestimento};
use crate::investmentlib::algorithms::{total_de_redimentos_mensal, calculo_de_renda_passiva, calculo_pericia, calculo_reserva_de_emergencia};

pub fn ler_arquivo_json() -> Vec<Investimento> {
    let mut file = File::open("investments.json").expect("Arquivo não encontrado");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Não foi possível ler o arquivo");
    let data: Vec<Investimento> = serde_json::from_str(&contents)
        .expect("Não foi possível desserializar o JSON");
    data
}

pub fn escrever_arquivo_json(transactions: Vec<Investimento>) {
    println!("Escrevendo arquivo JSON");
    let file_name = "investments.json";
    let mut file = File::create(file_name).expect("Erro ao criar arquivo");
    let transaction_str = serde_json::to_string(&transactions).unwrap();
    file.write_all(transaction_str.as_bytes()).expect("Erro ao escrever no arquivo");
}


pub fn ler_input(mensagem: &str) -> String {
    println!("{}", mensagem);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Erro ao ler input");
    input
}

pub fn remover(id: i32) {
    let mut file: Vec<Investimento> = ler_arquivo_json();
    let index = file.iter().position(|x| x.id == id).unwrap();
    file.remove(index);
    escrever_arquivo_json(file);
    println!("Operação removida");
}

pub fn listar() {
    let file = ler_arquivo_json();
    println!("Investimento: {:?} ", file.len());
    for item in file.into_iter() {
        item.listar();
    }
}

pub fn detalhar(id: i32) {
    let file = ler_arquivo_json();
    let investimento = file.iter()
        .find(|x| x.id == id).unwrap();
    investimento.detalhe();
}

pub fn editar(id: i32) {
    let mut file = ler_arquivo_json();
    let investimento = file.iter_mut()
        .find(|x| x.id == id).unwrap();
    investimento.detalhe();
    println!("-------------------------");
    let opcao = ler_input("Você deseja editar?(y/N) ").trim().to_string();
    match  opcao.as_str() {
        "y" | "Y" => {
            remover(id);
            adicionar();
            println!("Operação editada");
        }
        _ => println!("Operação cancelada"),
    }
}

pub fn adicionar() { 
    let mut file: Vec<Investimento> = ler_arquivo_json(); 
    let id = if file.len() != 0 { file[file.len()-1].id + 1 } else { 1 }; 
    let descricao = ler_input("Digite a descrição: ").trim().to_string();
    let valor = ler_input("Digite o valor: ").trim().parse::<f32>().unwrap();
    // mostrar_opcoes_operacao();
    let opcao_operacao = ler_input("Digite a operação: ").trim().parse::<i32>().unwrap();
    let operacao = match opcao_operacao { 
        1 => Operacao::Aplicao,
        2 => Operacao::Resgate,
        _ => Operacao::Aplicao
    };
    // mostrar_opcoes_tipos_investimentos();
    let opcao_tipo_investimento = ler_input("Digite o tipo de investimento: ").trim().parse::<i32>().unwrap();
    let tipo_investimento = match opcao_tipo_investimento {
        1 => TipoInvestimento::Selic,
        2 => TipoInvestimento::Bitcoin,
        _ => TipoInvestimento::Bitcoin
    };
    let mes = ler_input("Digite o mês: ").trim().parse::<u32>().unwrap();
    let ano = ler_input("Digite o ano: ").trim().parse::<u32>().unwrap();
    let new_transaction = Investimento::new(
        id,
        descricao, 
        valor,
        operacao,
        tipo_investimento,
        mes,
        ano,
    );
    file.push(new_transaction);
    escrever_arquivo_json(file);
    println!("Investimento Adicionado");
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

pub fn calcular_valor_de_uma_pericia() {
    let horas_esperadas = ler_input("Digite as horas esperadas: ").trim().parse::<i32>().unwrap();
    let honrarios = calculo_pericia(horas_esperadas);
    println!("O valor da perícia é de: {}", honrarios);
}
