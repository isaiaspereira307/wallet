use crate::algorithms::algorithms::{calcula_comunhao_bens, calcular_receita_despesa};
use crate::filemanager::filemanager::{
    read_input,
    adicionar,
    remove,
    to_detail,
    to_list,
    edit
};


pub fn header() {
    let (total_receitas, total_despesas) = calcular_receita_despesa();
    let comunhao_bens = calcula_comunhao_bens(total_receitas);
    let limite_depesa = 0.45 * total_receitas;
    let valor_a_guardar = 0.30 * total_receitas;
    let valor_a_investir = 0.25 * total_receitas;
    println!("-------------------------");
    println!("Sistema de Finanças Pessoais");
    println!("-------------------------");
    println!("Total de receitas:  R$ {:.2}", total_receitas);
    println!("Total de despesas:  R$ {:.2}", total_despesas);
    println!("Comunhão de bens:   R$ {:.2}", comunhao_bens);
    println!();
    println!("Limite de despesas: R$ {:.2}", limite_depesa);
    println!("Valor a guardar:    R$ {:.2}", valor_a_guardar);
    println!("Valor a Investir:    R$ {:.2}", valor_a_investir);
    println!();
    println!("Saldo:              R$ {:.2}", total_receitas - total_despesas - comunhao_bens);
    println!("-------------------------");
}

pub fn menu_principal() {
    header();
    println!("1 - Carteira");
    println!("3 - Sair");
    let opcao = read_input("Digite uma opção: ");
    match opcao.trim().parse::<i32>() {
        Ok(1) => {
            menu_carteira();
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
    println!("5 - Detalhes");
    println!("6 - Voltar");
    println!();
    println!("7 - Sair");
    let opcao = read_input("Digite uma opção: ");
    match opcao.trim().parse::<i32>() {
        Ok(1) => adicionar(),
        Ok(2) => {
            let id = read_input("Digite o id da transação: ");
            remove(id.trim().parse::<i32>().unwrap());
        },
        Ok(3) => {
            to_list();
        },
        Ok(4) => {
            let id = read_input("Digite o id da transação: ");
            edit(id.trim().parse::<i32>().unwrap());
        },
        Ok(5) => {
            let id = read_input("Digite o id da transação: ");
            to_detail(id.trim().parse::<i32>().unwrap());
        }
        Ok(6) => {
            menu_principal();
        },
        Ok(7) => {
            println!("Saindo...");
            std::process::exit(0);
        },
        _ => {
            println!("Opção inválida");
        }
    }
}
