use std::io;
use crate::games::{self, game_interface::{Game, GameInstance}};

fn select_number_of_game() -> i32 {

    loop {
        println!("
            1-StartField
            2-Baldur's Gate 3
            3-Red Dead Redemption 2
        ");
    
        let mut input= String::new();
        io::stdin().read_line(&mut input).expect("Erro para pegar a entrada do usuário!");
        let input = input.trim().parse::<i32>();
        if let Err(_err) = input {
            println!("Digite um número válido!");
            continue;
        }
    
        let input = input.unwrap();
        return input;
    }
    }

pub fn select_game() -> Box<dyn Game>{
    loop {

        let input = select_number_of_game();

        let result:Box<dyn Game>;

        if input == 1{
            result = Box::new(games::starfield::StarField::new());
        }else if input == 2 {
            result = Box::new(games::baldurs_game_3::BaldursGame3::new());
        
        }else if input == 3 {
            result = Box::new(games::red_dead_2::RedDead2::new());
        }else{
            println!("Escolha um game válido");
            continue;
        };
        return result;
    }
}