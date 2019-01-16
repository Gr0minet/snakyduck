extern crate ncurses;

use ncurses::*;
use std::time::Instant;

mod game;

use crate::game::*;

fn main () {
    // Setup ncurses
    initscr();
    cbreak();

    // Allow for extended keyboard (like F1)
    keypad(stdscr(), true);
    noecho();

    // Invisible cursor
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // Don't block on getch
    timeout(0);

    refresh();

    // Get the screen bounds
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    // Quit if size is too small
    if max_y < MAIN_HEIGHT || max_x < MAIN_WIDTH {
        endwin();
        println!("Need a bigger screen size! Minimum {}*{}",
                 MAIN_WIDTH, MAIN_HEIGHT);
        return;
    }

    let (win1, win2) = create_border();
    let game_win = create_win(1, 1, GAME_HEIGHT, GAME_WIDTH, false);
    let info_win = create_win(1, 2 + GAME_WIDTH, INFO_HEIGHT, INFO_WIDTH,
                              false);
    
    let mut player1 = Snake::new(0);
    let mut player2 = Snake::new(1);

    let mut egg = Block::new();
    egg.regenerate(&player1, &player2);

    print(game_win, &player1, &player2, &egg);
    wrefresh(game_win);
    
    let mut prev_time = Instant::now();
    let mut ch: i32;
    let mut input = Input::new();

    while !input.quit {

        ch = getch();
        if ch != -1 {
            input.handle_ch (ch);
        }
        let cur_time = Instant::now();
        let diff = cur_time.duration_since(prev_time);
        let diff = diff.as_secs() * 1_000 + 
                   (diff.subsec_nanos() / 1_000_000) as u64;
        if diff >= 100 {
            unprint(game_win, &player1, &player2);

            match update(&mut input, &mut player1, &mut player2, &mut egg) {
                Collision::Both => {
                    destroy_win(game_win);
                    destroy_win(info_win);
                    delete_border(win1, win2);
                    endwin();
                    println!("Both players lose!");
                    return;
                }
                Collision::Player1 => {
                    destroy_win(game_win);
                    destroy_win(info_win);
                    delete_border(win1, win2);
                    endwin();
                    println!("Player 'X' loses!");
                    return;
                }
                Collision::Player2 => {
                    destroy_win(game_win);
                    destroy_win(info_win);
                    delete_border(win1, win2);
                    endwin();
                    println!("Player 'O' loses!");
                    return;
                }
                _ => { }
            }

            input.reset();
            print(game_win, &player1, &player2, &egg);
            wrefresh(game_win);
            prev_time = cur_time;
        }
    }

    destroy_win(game_win);
    destroy_win(info_win);
    delete_border(win1, win2);
    endwin();
}

fn create_win (start_y: i32, start_x: i32, height: i32, width: i32,
               border: bool) -> WINDOW {
    let win = newwin(height, width, start_y, start_x);
    if border {
        box_(win, 0, 0);
    }
    wrefresh(win);
    win
}

fn destroy_win (win: WINDOW) {
    let ch = ' ' as chtype;
    wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    wrefresh(win);
    delwin(win);
}

fn create_border () -> (WINDOW, WINDOW) {
    let win1 = create_win (0, 0, GAME_HEIGHT + 2, GAME_WIDTH + 2, true);
    let win2 = create_win (0, GAME_WIDTH + 1, INFO_HEIGHT + 2,
                           INFO_WIDTH + 2, true);

    mvaddch(0, GAME_WIDTH + 1, ACS_TTEE());
    mvaddch(MAIN_HEIGHT - 1, GAME_WIDTH + 1, ACS_BTEE());

    (win1, win2)
}

fn delete_border (win1: WINDOW, win2: WINDOW) {
    destroy_win(win1);
    destroy_win(win2);
}

