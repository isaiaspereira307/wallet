use std::fs::File;
use std::io::prelude::*;
use crate::objects::objects::{Transaction, Operation, TypeInvestment, Type};
use crate::algorithms::algorithms::{total_de_redimentos_mensal, calculo_de_renda_passiva, calculo_pericia, calculo_reserva_de_emergencia};

pub fn read_file() -> Vec<Transaction> {
    let mut file = File::open("data.json").expect("Arquivo não encontrado");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Não foi possível ler o arquivo");
    let data: Vec<Transaction> = serde_json::from_str(&contents)
        .expect("Não foi possível desserializar o JSON");
    data
}

pub fn write_file(transactions: Vec<Transaction>) {
    println!("Escrevendo arquivo JSON");
    let mut file = File::create("data.json").expect("Erro ao criar arquivo");
    let transaction_str = serde_json::to_string(&transactions).unwrap();
    file.write_all(transaction_str.as_bytes()).expect("Erro ao escrever no arquivo");
}


pub fn read_input(mensagem: &str) -> String {
    println!("{}", mensagem);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Erro ao ler input");
    input
}

pub fn remove(id: i32) {
    let mut file = read_file();
    let index = file.iter().position(|x| x.id == id).unwrap();
    file.remove(index);
    write_file(file);
    println!("Operação removida");
}

pub fn to_list() {
    let file = read_file();
    println!("Transactions: {:?} ", file.len());
    for item in file.into_iter() {
        item.list()
    }
}

pub fn to_detail(id: i32) {
    read_file().into_iter()
        .find(|transaction| transaction.id == id).unwrap()
        .details()
}

pub fn edit(id: i32) {
    let mut file = read_file();
    let investimento = file.iter_mut()
        .find(|x| x.id == id).unwrap();
    investimento.details();
    println!("-------------------------");
    let opcao = read_input("Você deseja editar?(y/N) ").trim().to_string();
    match  opcao.as_str() {
        "y" | "Y" => {
            remove(id);
            adicionar();
            println!("Operação editada");
        }
        _ => println!("Operação cancelada"),
    }
}

pub fn adicionar() { 
    let mut file = read_file(); 
    let id = if file.len() != 0 { file[file.len()-1].id + 1 } else { 1 }; 
    let description = read_input("Digite a descrição: ").trim().to_string();
    let amount = read_input("Digite o valor: ").trim().parse::<f64>().unwrap();
    println!("-------------------------");
    println!("1 - Redemption");
    println!("2 - Deposit");
    let choice_operation = read_input("Digite a operação: ").trim().parse::<i32>().unwrap();
    let operation = match choice_operation { 
        1 => Operation::Deposit,
        2 => Operation::Redemption,
        _ => Operation::Deposit
    };
    println!("-------------------------");
    println!("1 - Valor Fixo");
    println!("2 - Valor Variável");
    println!("Digite uma opção de tipo: ");
    let choice_type_transaction = read_input("Digite o tipo de transação: ").trim().parse::<i32>().unwrap();
    let type_transaction = match choice_type_transaction {
        1 => Type::FixedValue,
        2 => Type::VariableValue,
        _ => Type::VariableValue
    };
    // mostrar_opcoes_tipos_investimentos();
    let choice_type_investment = read_input("Digite o tipo de investimento: ").trim().parse::<i32>().unwrap();
    let type_investment = match choice_type_investment {
        1 => TypeInvestment::RendaFixa,
        2 => TypeInvestment::Bitcoin,
        _ => TypeInvestment::Bitcoin
    };
    let month = read_input("Digite o mês: ").trim().parse::<u32>().unwrap();
    let year = read_input("Digite o ano: ").trim().parse::<u32>().unwrap();
    let new_transaction = Transaction {
        id: id,
        description: description,
        amount: amount,
        operation: operation,
        type_transaction: type_transaction,
        type_investment: Some(type_investment),
        month: Some(month),
        year: Some(year),
    };
    file.push(new_transaction);
    write_file(file);
    println!("Investimento Adicionado");
}


pub fn calcular_valor_de_uma_pericia() {
    let horas_esperadas = read_input("Digite as horas esperadas: ").trim().parse::<i32>().unwrap();
    let honrarios = calculo_pericia(horas_esperadas);
    println!("O valor da perícia é de: {}", honrarios);
}