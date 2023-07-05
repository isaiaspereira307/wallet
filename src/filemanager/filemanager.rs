use std::fs::File;
use std::io::prelude::*;
use crate::investmentlib::structsinvestiments::{Investimento, Operacao, TipoInvestimento};
use crate::investmentlib::algorithms::{total_de_redimentos_mensal, calculo_de_renda_passiva, calculo_pericia, calculo_reserva_de_emergencia};

pub fn ler_arquivo<T: serde::de::DeserializeOwned>(filename: String) -> Vec<T> {
    let mut file = File::open(filename).expect("Arquivo não encontrado");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Não foi possível ler o arquivo");
    let data: Vec<T> = serde_json::from_str(&contents)
        .expect("Não foi possível desserializar o JSON");
    data
}

pub fn escrever_arquivo_investimentos(transactions: Vec<Investimento>) {
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
    let mut file: Vec<Investimento> = ler_arquivo::<Investimento>("investments.json".to_string());
    let index = file.iter().position(|x| x.id == id).unwrap();
    file.remove(index);
    escrever_arquivo_investimentos(file);
    println!("Operação removida");
}

pub fn listar() {
    let file = ler_arquivo::<Investimento>("investments.json".to_string());
    println!("Investimento: {:?} ", file.len());
    for item in file.into_iter() {
        item.listar();
    }
}

pub fn detalhar(id: i32) {
    let file = ler_arquivo::<Investimento>("investments.json".to_string());
    let investimento = file.iter()
        .find(|x| x.id == id).unwrap();
    investimento.detalhe();
}

pub fn editar(id: i32) {
    let mut file = ler_arquivo::<Investimento>("investments.json".to_string());
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
    let mut file: Vec<Investimento> = ler_arquivo::<Investimento>("investments.json".to_string()); 
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
    escrever_arquivo_investimentos(file);
    println!("Investimento Adicionado");
}


pub fn calcular_valor_de_uma_pericia() {
    let horas_esperadas = ler_input("Digite as horas esperadas: ").trim().parse::<i32>().unwrap();
    let honrarios = calculo_pericia(horas_esperadas);
    println!("O valor da perícia é de: {}", honrarios);
}


use crate::walletlib::objects::{Transacao, Operacao, Tipo, Categoria};
use crate::walletlib::banners::{mostrar_opcoes_categoria, mostrar_opcoes_operacao, mostrar_opcoes_tipo};


pub fn escrever_arquivo_json(transactions: Vec<Transacao>) {
    println!("Escrevendo arquivo JSON");
    let file_name = "data.json";
    let mut file = File::create(file_name).expect("Erro ao criar arquivo");
    let transaction_str = serde_json::to_string(&transactions).unwrap();
    file.write_all(transaction_str.as_bytes()).expect("Erro ao escrever no arquivo");
}


pub fn remover_transacao(id: i32) {
    let mut file: Vec<Transacao> = ler_arquivo::<Transacao>("transacao.json".to_string());
    let index = file.iter().position(|x| x.id == id).unwrap();
    file.remove(index);
    escrever_arquivo_json(file);
    println!("Operação removida");
}

pub fn listar() {
    let file = ler_arquivo_json();
    println!("Transações: {:?} ", file.len());
    for item in file.into_iter() {
        item.listar();
    }
}

pub fn detalhar(id: i32) {
    let file = ler_arquivo_json();
    let transaction = file.iter()
        .find(|x| x.id == id).unwrap();
    transaction.detalhe();
}

pub fn editar(id: i32) {
    let mut file = ler_arquivo_json();
    let transaction = file.iter_mut()
        .find(|x| x.id == id).unwrap();
    transaction.detalhe();
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
    let mut file: Vec<Transacao> = ler_arquivo_json(); 
    println!("Tamanho do arquivo: {:?}", file.len()); 
    let id = if file.len() != 0 { file[file.len()-1].id + 1 } else { 1 }; 
    let descricao = ler_input("Digite a descrição: ").trim().to_string();
    let valor = ler_input("Digite o valor: ").trim().parse::<f32>().unwrap();
    mostrar_opcoes_operacao();
    let opcao_operacao = ler_input("Digite a operação: ").trim().parse::<i32>().unwrap();
    let operacao = match opcao_operacao { 
        1 => Operacao::Despesa,
        2 => Operacao::Receita,
        _ => Operacao::Despesa
    };
    mostrar_opcoes_tipo();
    let opcao_tipo = ler_input("Digite o tipo: ").trim().parse::<i32>().unwrap();
    let tipo = match opcao_tipo { 
        1 => Tipo::ValorFixo,
        2 => Tipo::ValorVariavel,
        _ => Tipo::ValorVariavel
    };
    mostrar_opcoes_categoria();
    let opcao_categoria = ler_input("Digite a categoria: ").trim().parse::<i32>().unwrap();
    let categoria = match opcao_categoria {
        1 => Categoria::Salario,
        2 => Categoria::RendaExtra,
        3 => Categoria::Alimentacao,
        4 => Categoria::Transporte,
        5 => Categoria::Lazer,
        6 => Categoria::Educacao,
        7 => Categoria::Saude,
        _ => Categoria::Outros, 
    }; 
    if tipo == Tipo::ValorVariavel {
        let mes = ler_input("Digite o mês: ").trim().parse::<u32>().unwrap();
        let ano = ler_input("Digite o ano: ").trim().parse::<i32>().unwrap();
        let new_transaction = Transacao::new(
            id, 
            descricao, 
            valor,
            operacao, 
            tipo, 
            categoria,
            Some(mes),
            Some(ano),
        );
        file.push(new_transaction);
        escrever_arquivo_json(file);
        println!("Transação Adicionada");
    } else {
        let new_transaction = Transacao::new(
            id, 
            descricao,
            valor,
            operacao, 
            tipo, 
            categoria, 
            None,
            None,
        );
        file.push(new_transaction);
        escrever_arquivo_json(file);
        println!("Transação Adicionada");  
    }
}