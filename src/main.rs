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
    timeout(1);

    refresh();

    // Get the screen bounds
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    // Quit if size is too small
    if max_y < WINDOW_HEIGHT || max_x < WINDOW_WIDTH {
        endwin();
        println!("Need a bigger screen size! Minimum {}*{}",
                 WINDOW_WIDTH, WINDOW_HEIGHT);
        return;
    }

    let win = create_win(0, 0);

    let mut player1 = Snake::new(0);
    let mut player2 = Snake::new(1);

    let mut egg = Block::new();
    print(&player1, &player2, &egg);
    
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
            unprint(&player1, &player2);

            match update(&mut input, &mut player1, &mut player2, &mut egg) {
                Some(Collision::Both) => {
                    destroy_win(win);
                    endwin();
                    println!("Both players lose!");
                    return;
                }
                Some(Collision::Player1) => {
                    destroy_win(win);
                    endwin();
                    println!("Player 'X' loses!");
                    return;
                }
                Some(Collision::Player2) => {
                    destroy_win(win);
                    endwin();
                    println!("Player 'O' loses!");
                    return;
                }
                _ => { }
            }

            input.reset();
            print(&player1, &player2, &egg);
            prev_time = cur_time;
        }
    }

    destroy_win(win);
    endwin();
}

fn create_win (start_y: i32, start_x: i32) -> WINDOW {
    let win = newwin(WINDOW_HEIGHT, WINDOW_WIDTH, start_y, start_x);
    box_(win, 0, 0);
    wrefresh(win);
    win
}

fn destroy_win (win: WINDOW) {
    let ch = ' ' as chtype;
    wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    wrefresh(win);
    delwin(win);
}

