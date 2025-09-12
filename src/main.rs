use std::io;

mod chess;
mod bots;

use chess::game::Game;
use chess::r#move::Move;

fn main() {
    let mut game = Game::new();

    game.default_board();
    game.fancy_print();
    let _ = read_line();
    game_loop(game);
    let _ = read_line();
}

fn game_loop(mut game: Game) {
    let mut history: Vec<Game> = Vec::new();
    let mut m : Move;

    game.start_game();
    let mut quit : bool = false;
    while !quit
    {
        if !game.any_valid_moves()
        {
            if game.is_check()
            {
                game.set_checkmate();
            }
            else
            {
                game.set_stalemate();
            }
            quit = true;
        }
        game.fancy_print();
        if quit == true
        {
            continue;
        }

        if game.current_is_human()
        {
            let input = read_line();
            if input == "quit" || input == "concede" || input == "exit"
            {
                game.set_concede();
                quit = true;
                game.clear_hl();
                game.fancy_print();
                continue;
            }

            if input == "flip"
            {
                game.flip_board();
                continue;
            }

            m = match game.parse_notation(input) {
                Ok(m) => m,
                Err(s) => {game.set_error(s); continue},
            };
        }
        else if game.current_is_bot()
        {
            m = Move::new()
        }
        else
        {
            panic!("Unknown player type!")
        }

        if let Some(s) = game.disambiguate(&mut m)
        {
            game.set_error(s);
            continue;
        }

        game.clear_notes();
        game.clear_hl();
        let mut prev = game.do_move(m);
        if game.is_check()
        {
            game = prev;
            game.hl_king();
            continue;
        }
        else
        {
            prev.clear_hl();
            history.push(prev);
        }

        game.next_turn();
    }
}

fn read_line() -> String {
    let mut input = String::new();
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    String::from(input.trim())
}

