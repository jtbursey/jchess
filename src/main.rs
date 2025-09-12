mod chess;
mod input;
mod bots;

use chess::game::Game;
use chess::r#move::{MetaMove, Move};
use input::*;

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
