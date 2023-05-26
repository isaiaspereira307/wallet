use std::fs::File;
use std::io::prelude::*;
use crate::walletlib::objects::{Transacao, Operacao, Tipo, Categoria, TipoInvestimento};
use crate::walletlib::banners::{mostrar_opcoes_categoria, mostrar_opcoes_operacao, mostrar_opcoes_tipo, mostrar_opcoes_tipos_investimentos};
use crate::walletlib::algorithms::{calcular_receita_despesa, calculo_reserva_de_emergencia, calculos_fii, calculo_de_renda_passiva, calculo_pericia};

pub fn ler_arquivo_json() -> Vec<Transacao> {
    let mut file = File::open("data.json").expect("Arquivo não encontrado");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Não foi possível ler o arquivo");
    let data: Vec<Transacao> = serde_json::from_str(&contents)
        .expect("Não foi possível desserializar o JSON");
    data
}

pub fn escrever_arquivo_json(transactions: Vec<Transacao>) {
    println!("Escrevendo arquivo JSON");
    let file_name = "data.json";
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
    let mut file: Vec<Transacao> = ler_arquivo_json();
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
        _ => Operacao::Despesa,
    };
    mostrar_opcoes_tipo();
    let opcao_tipo = ler_input("Digite o tipo: ").trim().parse::<i32>().unwrap();
    let tipo = match opcao_tipo {
        1 => Tipo::ValorFixo,
        2 => Tipo::ValorVariavel,
        _ => Tipo::ValorVariavel,
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
        8 => Categoria::Investimento,
        _ => Categoria::Outros,
    };
    if categoria == Categoria::Investimento {
        mostrar_opcoes_tipos_investimentos();
        let opcao_tipo_investimento = ler_input("Digite o tipo de investimento: ").trim().parse::<i32>().unwrap();
        let tipo_investimento = match opcao_tipo_investimento {
            1 => TipoInvestimento::Cdb,
            2 => TipoInvestimento::Fii,
            3 => TipoInvestimento::Bitcoin,
            _ => TipoInvestimento::Nenhum,
        };
        let objetivo = ler_input("Digite o objetivo: ").trim().to_string();    
        let dia = ler_input("Digite o dia: ").trim().parse::<i32>().unwrap();
        let mes = ler_input("Digite o mês: ").trim().parse::<i32>().unwrap();
        let ano = ler_input("Digite o ano: ").trim().parse::<i32>().unwrap();
        if tipo_investimento == TipoInvestimento::Cdb {
            let valor_cdb = ler_input("Digite o valor do CDB: ").trim().parse::<f32>().unwrap();
            let rendimento = ler_input("Digite o rendimento: ").trim().parse::<f32>().unwrap();
            let taxa = ler_input("Digite a taxa: ").trim().parse::<f32>().unwrap();
            let new_transaction = Transacao::new(
                id, 
                descricao, 
                valor,
                operacao, 
                tipo, 
                categoria, 
                Some(objetivo), 
                Some(tipo_investimento), 
                Some(dia), 
                mes,
                Some(ano),
                Some(valor_cdb), 
                Some(rendimento),
                Some(taxa),
                None,
                None,
                None,
                None
            );
            file.push(new_transaction);
            escrever_arquivo_json(file);
            println!("Investimento adicionado");
        } else if tipo_investimento == TipoInvestimento::Fii {
            let valor_cota = ler_input("Digite o valor da cota: ").trim().parse::<f32>().unwrap();
            let dividendos = ler_input("Digite os dividendos: ").trim().parse::<f32>().unwrap();
            let numero_cotas = ler_input("Digite o número de cotas: ").trim().parse::<f32>().unwrap();
            let new_transaction = Transacao::new(
                id, 
                descricao, 
                valor,
                operacao, 
                tipo, 
                categoria, 
                Some(objetivo), 
                Some(tipo_investimento), 
                None, 
                mes,
                Some(ano),
                None, 
                None,
                None,
                Some(valor_cota),
                Some(dividendos),
                Some(numero_cotas),
                None
            );
            file.push(new_transaction);
            escrever_arquivo_json(file);
            println!("Investimento adicionado");
        } else if tipo_investimento == TipoInvestimento::Bitcoin {
            let valor_bitcoin = ler_input("Digite o valor do bitcoin: ").trim().parse::<f32>().unwrap();
            let new_transaction = Transacao::new(
                id, 
                descricao, 
                valor,
                operacao, 
                tipo, 
                categoria, 
                Some(objetivo), 
                Some(tipo_investimento), 
                Some(dia), 
                mes,
                Some(ano),
                None, 
                None,
                None,
                None,
                None,
                None,
                Some(valor_bitcoin)
            );
            file.push(new_transaction);
            escrever_arquivo_json(file);
            println!("Investimento adicionado");
        } else if tipo_investimento == TipoInvestimento::Nenhum {
            println!("Escolha inválida");
            return;
        }
    } else {
        let mes = ler_input("Digite o mês: ").trim().parse::<i32>().unwrap();
        let transaction = Transacao {
            id,
            descricao,
            valor,
            operacao,
            tipo,
            categoria,
            mes,
            ..Default::default()
        };
        file.push(transaction);
        escrever_arquivo_json(file);
        println!("Operação adicionada");
    }
}

pub fn calcular_reserva_de_emergencia() {
    let file = ler_arquivo_json();
    let (total_receitas, total_despesas) = calcular_receita_despesa(file);
    let percentual_a_guardar = 0.3;
    let limite_de_despesas = total_receitas * 0.7;
    if total_despesas > limite_de_despesas {
        println!("Você gastou mais do que 70% da sua receita");
        let diferenca = total_despesas - limite_de_despesas;
        println!("Você precisa reduzir suas despesas em {} para atingir o limite de 70% da sua receita", diferenca);
    } else {
        println!("Você está gastando menos do que 70% da sua receita");
        let objetivo = 12_000.0;
        let rentabilidade_mensal = 0.04;
        let valor_guardado_mensal = total_receitas * percentual_a_guardar;
        let (meses, valor_guardado) = calculo_reserva_de_emergencia(valor_guardado_mensal, objetivo, rentabilidade_mensal);
        println!("Para ter uma reserva de emergência que cubra 6 meses com 3 mil cada mês.");
        println!("Meses para atingir o objetivo: {}", meses);
        println!("Valor guardado no período: {}", valor_guardado);
    }    
}

pub fn calcular_renda_passiva(){
    let file = ler_arquivo_json();
    let (total_receitas, total_despesas) = calcular_receita_despesa(file);
    let percentual_a_guardar = 0.3;
    let limite_de_despesas = total_receitas * 0.7;
    if total_despesas > limite_de_despesas {
        println!("Você gastou mais do que 70% da sua receita");
    } else {
        println!("Você está gastando menos do que 70% da sua receita");
        let objetivo = 10_000.0;
        let rentabilidade_mensal = 0.04;
        let (meses, valor_guardado) = calculo_de_renda_passiva(total_receitas, objetivo, percentual_a_guardar, rentabilidade_mensal);
        println!("Meses para atingir o objetivo: {}", meses);
        println!("Valor guardado no período: {}", valor_guardado);
    }
    
}

pub fn calcular_valor_de_uma_pericia() {
    let horas_esperadas = ler_input("Digite as horas esperadas: ").trim().parse::<i32>().unwrap();
    let honrarios = calculo_pericia(horas_esperadas);
    println!("O valor da perícia é de: {}", honrarios);
}

pub fn calcular_valores_de_fiis() {
    let file = ler_arquivo_json();
    let fiis = file.iter().filter(|&x| x.tipo_investimento == Some(TipoInvestimento::Fii)).collect::<Vec<_>>();
    let mut fundos = Vec::new();
    for fii in fiis {
        let valor_restante = calculos_fii(fii);
        fundos.push(valor_restante);
    }
    println!("Valor necessário total para viver de renda com FIIs: {:.2}", fundos.iter().sum::<f32>());
}
