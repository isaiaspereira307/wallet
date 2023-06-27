mod walletlib;
mod investmentlib;
use walletlib::banners::{menu_principal};

// fn menu_principal() {
//     header();
//     println!("1 - Carteira");
//     println!("2 - Investimentos");
//     println!("3 - Sair");
//     let opcao = ler_input("Digite uma opção: ");
//     match opcao.trim().parse::<i32>() {
//         Ok(1) => {
//             menu_carteira();
//         },
//         Ok(2) => {
//             menu_investimento();
//         },
//         Ok(3) => {
//             println!("Saindo...");
//             std::process::exit(0);
//         },
//         _ => {
//             println!("Opção inválida");
//         }
//     }
// }
fn main() {
    loop {
        menu_principal();
    }
}
