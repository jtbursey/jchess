mod chess;
mod input;
mod bots;

use chess::game::Game;
use chess::r#move::{MetaMove, Move};
use chess::setup::Setup;
use input::*;

fn main() {
    let mut game = Game::new();
    let exit = false;

    game.default_board();

    while !exit {
        game.title();
        game.fancy_print();
        let input = read_line();

        if input == "1"
        {
            game_loop(&mut game);
            let _ = read_line();
        }
        else if input == "2"
        {
            setup_loop(&mut game);
        }
        else if input == "3"
        {
            break;
        }
    }
}

fn game_loop(game: &mut Game) {
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
        if quit {
            break;
        }

        m = match game.current_player().get_move(&game) {
            Ok(mo) => mo,
            Err(s) => {game.set_error(s); continue},
        };
        if m.meta == MetaMove::Quit || m.meta == MetaMove::Concede
        {
            game.set_concede();
            quit = true;
            game.clear_hl();
            game.fancy_print();
            continue;
        }
        else if m.meta == MetaMove::Flip
        {
            game.flip_board();
            continue;
        }

        if let Some(s) = game.disambiguate(&mut m)
        {
            game.set_error(s);
            continue;
        }

        game.clear_notes();
        game.clear_hl();
        let mut tmp = game.clone();
        let _ = tmp.do_move(m);
        if tmp.is_check()
        {
            game.hl_king();
            continue;
        }
        else
        {
            let mut prev = game.do_move(m);
            prev.clear_hl();
            history.push(prev);
        }

        game.next_turn();
    }
}

fn setup_loop(game: &mut Game) {
    let mut quit = false;
    let mut config = Setup::new();

    game.start_setup();
    while !quit
    {
        game.fancy_print_setup(&config);
        let input = read_line();
        let idx = match input.parse::<usize>() {
            Ok(i) => i - 1,
            Err(_) => continue,
        };
        if config.select(idx, game) {
            quit = true;
        }
    }
}
