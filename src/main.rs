use std::io;

fn main() {
    let mut valtot = String :: new();
    let mut valcom = String :: new();
    let mut valite = String :: new();
    let mut quant = String :: new();
    let mut ext: bool;
    let mut extd: bool;
    let mut menu = String :: new();
    let mut menud = String :: new();

    ext = true;
    while ext {
        println!("Selecione uma opção: \n\n0. Sair.\n1. Nova venda. \n2. Consulta. \n\nDigite: ");

        io::stdin().read_line(&mut menu).expect("Fail");

        if convert_to_int(&menu) == 0 {
            ext = false;
        }

        else if convert_to_int(&menu) == 1 {
            //entra na função 1
            extd = true;
            while extd {
                println!("Selecione a forma de pagamento: \n\n0. Cancelar. \n1. Cartão Crédito. \n2. Cartão Débto. \n3. Dinheiro. \n4. Pix. \n\nDigite: ");
                
                io::stdin().read_line(&mut menud).expect("Fail");
                
                if convert_to_int(&menud) == 0 {
                    extd = false;
                }

                else if convert_to_int(&menud) == 1 {
                    //Função pagar crédito
                }

                else if convert_to_int(&menud) == 2 {
                    //Função pagar débito
                }

                else if convert_to_int(&menud) == 3 {
                    //Função pagar Dinheiro
                }

                else if convert_to_int(&menud) == 4 {
                    //Função pagar pix
                }

                else {
                    println!("Valor inválido, por favor, digite novamente.");
                }
                
                menud.clear();

            }
            
        }

        else if convert_to_int(&menu) == 2 {
            //entra na função 2
        }

        else {
            println!("\nValor inválido, digite novamente.\n");
        }
        menu.clear();
    }
    
}

fn convert_to_int(n: &String) -> u8{
    let x = n.trim().parse::<u8>().ok().unwrap_or(100);
    x
}

fn convert_to_float(n: &String) -> f64{
    let x = n.trim().parse::<f64>().ok().unwrap_or(0.0);
    x
}
