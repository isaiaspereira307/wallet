mod walletlib;
use walletlib::banners::{menu_principal};
use walletlib::consumeapi::pegar_valor_btc;


fn main() {
    let resultado = pegar_valor_btc();
    println!("{:.2}", resultado);
    loop {
        menu_principal();
    }
}
