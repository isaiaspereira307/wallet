use clap::Parser;
use crate::banners::banners::header;
use crate::filemanager::filemanager::{
    adicionar,
    to_list,
    to_detail,
    edit,
    remove,
    calcular_valor_de_uma_pericia
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "Wallet")]
#[command(author = "Isaías Pereira")]
#[command(about = "Wallet system to consolidate investments")]
#[command(version, long_about = None)]
pub struct Args {    
    /// Add new transaction
    #[arg(short, long, default_value_t = false)]
    add: bool,

    /// List transactions
    #[arg(short, long, default_value_t = false)]
    list: bool,

    /// Remove transaction
    #[arg(short, long)]
    remove: Option<i32>,

    /// Edit transaction
    #[arg(short, long)]
    edit: Option<i32>,

    /// Detail transaction
    #[arg(short, long)]
    detail: Option<i32>,

    /// Calculate wallet summary
    #[arg(long, default_value_t = false)]
    resumo_carteira: bool,

    /// Calculate reserve
    #[arg(long, default_value_t = false)]
    calcular_reserva: bool,

    /// Calculate expertise value
    #[arg(long)]
    calcular_valor_pericia: Option<i32>,

    /// Calculate total earnings
    #[arg(long, default_value_t = false)]
    total_rendimentos: bool,

    /// Calculate passive income
    #[arg(long, default_value_t = false)]
    calcular_renda_passiva: bool,
}

pub fn comands() {
    let args = Args::parse();

    if args.resumo_carteira != false {
        header();
    }
    if args.add != false {
        adicionar();
    }
    if args.list != false {
        to_list();
    }
    if args.remove.is_some() != false {
        remove(args.remove.unwrap())
    }
    if args.edit.is_some() != false {
        edit(args.edit.unwrap())
    }
    if args.detail.is_some() != false {
        to_detail(args.detail.unwrap())
    }
    if args.calcular_reserva != false {
        println!("{:?}", args.calcular_reserva);
    }
    if args.calcular_valor_pericia.is_some() != false {
        calcular_valor_de_uma_pericia()
    }
    if args.total_rendimentos != false {
        println!("{:?}", args.total_rendimentos);
    }
    if args.calcular_renda_passiva != false {
        println!("{:?}", args.calcular_renda_passiva);
    }
}