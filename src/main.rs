extern crate ncurses;
extern crate rand;

use ncurses::*;
use rand::Rng;

static WINDOW_HEIGHT: i32 = 20;
static WINDOW_WIDTH: i32 = 20;

static X_MIN: i32 = 1;
static X_MAX: i32 = WINDOW_WIDTH - 2;

static Y_MIN: i32 = 1;
static Y_MAX: i32 = WINDOW_HEIGHT - 2;

static EGG: u64 = 'O' as u64;
static HEAD: u64 = 'X' as u64;
static BODY: u64 = 'x' as u64;

//#[derive(PartialEq)]
enum Collision {
    Body,
    Egg,
}

#[derive(PartialEq)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

struct Block {
    x: i32,
    y: i32,
}

impl Block {
    fn new () -> Block {
        let block = Block {
            x: rand::thread_rng().gen_range(X_MIN, X_MAX),
            y: rand::thread_rng().gen_range(Y_MIN, Y_MAX),
        };
        block
    }

    fn regenerate (&mut self) {
        self.x = rand::thread_rng().gen_range(X_MIN, X_MAX);
        self.y = rand::thread_rng().gen_range(Y_MIN, Y_MAX);
    }

    fn print (&self, ch: u64) {
        mvaddch(self.y, self.x, ch);
    }
}

struct Snake {
    body: Vec<Block>,
    new: bool,
    dir: Dir,
}

impl Snake {
    fn check_collision (&self, egg: &Block) -> Option<Collision> {
        let head = &self.body[0];
        for block in &self.body[1..] {
            if head.x == block.x && head.y == block.y {
                return Some(Collision::Body);
            }
        }

        if head.x == egg.x && head.y == egg.y {
            Some(Collision::Egg)
        } else {
            None
        }
    }

    fn update_dir (&mut self, ch: i32) {
        match ch {
            KEY_LEFT => {
                if self.dir != Dir::Right {
                    self.dir = Dir::Left;
                }
            },
            KEY_RIGHT => {
                if self.dir != Dir::Left {
                    self.dir = Dir::Right;
                }
            },
            KEY_UP => {
                if self.dir != Dir::Down {
                    self.dir = Dir::Up;
                }
            },
            KEY_DOWN => {
                if self.dir != Dir::Up {
                    self.dir = Dir::Down;
                }
            },
            _ => { },
        }
    }

    fn update_pos (&mut self) {
        let mut head = &mut self.body[0];
        let mut prev_pos = (head.x, head.y);

        match self.dir {
            Dir::Left => {
                if head.x == X_MIN {
                    head.x = X_MAX;
                } else {
                    head.x -= 1;
                }
            }, 
            Dir::Right => {
                if head.x == X_MAX {
                    head.x = X_MIN;
                } else {
                    head.x += 1;
                }
            },
            Dir::Up => {
                if head.y == Y_MIN {
                    head.y = Y_MAX;
                } else {
                    head.y -= 1;
                }
            },
            Dir::Down => {
                if head.y == Y_MAX {
                    head.y = Y_MIN;
                } else {
                    head.y += 1;
                }
            },
        }

        for block in &mut self.body[1..] {
            let temp_pos = (block.x, block.y);
            block.x = prev_pos.0;
            block.y = prev_pos.1;
            prev_pos.0 = temp_pos.0;
            prev_pos.1 = temp_pos.1;
        }

        if self.new {
            let bloc = Block { x: prev_pos.0, y: prev_pos.1 };
            self.body.push(bloc);
            self.new = false;
        }
    }

    fn print (&self) {
        mvaddch(self.body[0].y, self.body[0].x, HEAD);
        for block in &self.body[1..] {
            mvaddch(block.y, block.x, BODY);
        }
    }

    fn unprint (&self) {
        for block in &self.body {
            mvaddch(block.y, block.x, ' ' as u64);
        }
    }
}

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
        println!("Need a bigger screen size! Minimum 24*80");
        return;
    }

    let win = create_win(0, 0);

    let mut snake = Snake {
        body: Vec::new(),
        new: false,
        dir: Dir::Right,
    };

    let block = Block { x: 1, y: 1 };
    snake.body.push(block);

    let mut egg = Block::new();
    
    let mut ch = getch();
    while ch != 'q' as i32 {
        print(&snake, &egg);

        ch = getch();

        unprint(&snake);
        if let Some(Collision::Body) = update(ch, &mut snake, &mut egg) {
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

fn print (snake: &Snake, egg: &Block) {
    snake.print();
    egg.print(EGG);
}

fn unprint (snake: &Snake) {
    snake.unprint();
}

fn update (ch: i32, snake: &mut Snake, egg: &mut Block) -> Option<Collision> {
    snake.update_dir(ch);
    snake.update_pos();
    let col = snake.check_collision(egg);
    if let Some(Collision::Egg) = col {
        egg.regenerate();
        snake.new = true;
    }
    col
}
