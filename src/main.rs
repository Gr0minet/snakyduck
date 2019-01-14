extern crate ncurses;

use ncurses::*;

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
    timeout(100);

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

    let mut snake = Snake::new();

    let mut egg = Block::new();
    
    let mut ch = getch();
    while ch != 'q' as i32 {
        print(&snake, &egg);

        ch = getch();

        unprint(&snake);
        if update(ch, &mut snake, &mut egg) {
            endwin();
            println!("You lose!");
            return;
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
