use crate::walletlib::files::{
    ler_arquivo_json,
    ler_input,
    adicionar,
    remover,
    listar,
    editar,
    calcular_reserva_de_emergencia,
    calcular_renda_passiva,
    calcular_valor_de_uma_pericia,
    calcular_valores_de_fiis
};
use super::algorithms::{
    calcula_comunhao_bens,
    calcular_receita_despesa,
    listar_investimentos
};



pub fn header() {
    let file = ler_arquivo_json();
    let (total_receitas, total_despesas) = calcular_receita_despesa(file);
    let comunhao_bens = calcula_comunhao_bens(total_receitas);
    let limite_depesa = 0.7 * total_receitas;
    let valor_a_guardar = 0.3 * total_receitas;
    println!("-------------------------");
    println!("Sistema de Finanças Pessoais");
    println!("-------------------------");
    println!("Total de receitas:  {:.2?}", total_receitas);
    println!("Total de despesas:  {:.2?}", total_despesas);
    println!("Comunhão de bens:   {:.2?}", comunhao_bens);
    println!("Limite de despesas: {:.2?}", limite_depesa);
    println!("Valor a guardar:    {:.2?}", valor_a_guardar);
    println!("Saldo:              {:.2?}", total_receitas - total_despesas - comunhao_bens);
    println!("-------------------------");
}

pub fn menu_principal() {
    header();
    println!("1 - Carteira");
    println!("2 - Investimentos");
    println!("3 - Sair");
    let opcao = ler_input("Digite uma opção: ");
    match opcao.trim().parse::<i32>() {
        Ok(1) => {
            menu_carteira();
        },
        Ok(2) => {
            menu_investimento();
        },
        Ok(3) => {
            println!("Saindo...");
            std::process::exit(0);
        },
        _ => {
            println!("Opção inválida");
        }
    }
}

fn menu_carteira() {
    header();
    println!("1 - Adicionar");
    println!("2 - Remover");
    println!("3 - Listar");
    println!("4 - Editar");
    println!("5 - Voltar");
    println!();
    println!("6 - Sair");
    let opcao = ler_input("Digite uma opção: ");
    match opcao.trim().parse::<i32>() {
        Ok(1) => adicionar(),
        Ok(2) => {
            let id = ler_input("Digite o id da transação: ");
            remover(id.trim().parse::<i32>().unwrap());
        },
        Ok(3) => {
            listar();
        },
        Ok(4) => {
            let id = ler_input("Digite o id da transação: ");
            editar(id.trim().parse::<i32>().unwrap());
        },
        Ok(5) => {
            menu_principal();
        },
        Ok(6) => {
            println!("Saindo...");
            std::process::exit(0);
        },
        _ => {
            println!("Opção inválida");
        }
    }
}

fn menu_investimento() {
    let file = ler_arquivo_json();
    let investimentos = listar_investimentos(file);
    let total_investido = investimentos.iter().fold(0.0, |acc, x| acc + x.valor);
    println!("-------------------------");
    println!("Investimentos");
    println!("Total investido: {:.2?}", total_investido);
    println!("-------------------------");
    println!("1 - Reserva de emergência");
    println!("2 - Renda Passiva");
    println!("3 - FII");
    println!("4 - Pericia");
    println!("5 - Voltar");
    println!();
    println!("6 - Sair");
    let opcao = ler_input("Digite uma opção: ");
    match opcao.trim().parse::<i32>() {
        Ok(1) => calcular_reserva_de_emergencia(),
        Ok(2) => calcular_renda_passiva(),
        Ok(3) => calcular_valores_de_fiis(),
        Ok(4) => calcular_valor_de_uma_pericia(),
        Ok(5) => menu_principal(),
        Ok(6) => {
            println!("Saindo...");
            std::process::exit(0);
        },
        _ => {
            println!("Opção inválida");
        }       
    }
}

pub fn mostrar_opcoes_tipo() {
    println!("-------------------------");
    println!("1 - Valor Fixo");
    println!("2 - Valor Variável");
    println!("Digite uma opção de tipo: ");
}

pub fn mostrar_opcoes_operacao() {
    println!("-------------------------");
    println!("1 - Despesa");
    println!("2 - Receita");
    println!("Digite uma opção de operação: ");
}

pub fn mostrar_opcoes_categoria() {
    println!("-------------------------");
    println!("1 - Salário");
    println!("2 - Renda Extra");
    println!("3 - Alimentação");
    println!("4 - Transporte");
    println!("5 - Lazer");
    println!("6 - Educação");
    println!("7 - Saúde");
    println!("8 - Outros");
    println!("Digite uma opção de categoria: ");
}