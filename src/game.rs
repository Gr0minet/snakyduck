extern crate ncurses;
extern crate rand;

use ncurses::*;
use rand::Rng;

pub const WINDOW_HEIGHT: i32 = 20;
pub const WINDOW_WIDTH: i32 = 20;

pub const X_MIN: i32 = 1;
pub const X_MAX: i32 = WINDOW_WIDTH - 2;

pub const Y_MIN: i32 = 1;
pub const Y_MAX: i32 = WINDOW_HEIGHT - 2;

const EGG: u64 = '+' as u64;
pub const HEAD_1: u64 = 'X' as u64;
const BODY_1: u64 = 'x' as u64;
pub const HEAD_2: u64 = 'O' as u64;
const BODY_2: u64 = 'o' as u64;

const KEY_UP_2: i32 = 'z' as i32;
const KEY_DOWN_2: i32 = 's' as i32;
const KEY_RIGHT_2: i32 = 'd' as i32;
const KEY_LEFT_2: i32 = 'q' as i32;

const QUIT: i32 = '!' as i32;

pub enum Collision {
    Player1,
    Player2,
    Both,
    Egg,
}

#[derive(PartialEq)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
    Null,
}

pub struct Block {
    x: i32,
    y: i32,
}

impl Block {
    pub fn new () -> Block {
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

pub struct Snake {
    body: Vec<Block>,
    new: bool,
    dir: Dir,
    id: i8,
}

impl Snake {
    pub fn new (id: i8) -> Snake {
        let mut snake = Snake {
            body: Vec::new(),
            new: false,
            dir: Dir::Right,
            id: id,
        };
        if id == 0 {
            let block = Block { x: X_MIN, y: Y_MIN };
            snake.body.push(block);
        } else if id == 1 {
            let block = Block { x: Y_MAX - 1, y: Y_MAX - 1 };
            snake.body.push(block);
        }
        snake
    }

    fn check_collision (&self, egg: &Block, other: &Snake) 
    -> Option<Collision> {
        let head = &self.body[0];
        for block in &self.body[1..] {
            if head.x == block.x && head.y == block.y {
                if self.id == 0 {
                    return Some(Collision::Player1);
                } else if self.id == 1 {
                    return Some(Collision::Player2);
                }
            }
        }
        for block in &other.body {
            if head.x == block.x && head.y == block.y {
                if self.id == 0 {
                    return Some(Collision::Player1);
                } else if self.id == 1 {
                    return Some(Collision::Player2);
                }
            }
        }

        if head.x == egg.x && head.y == egg.y {
            Some(Collision::Egg)
        } else {
            None
        }
    }

    fn update_dir (&mut self, input: &Input, player: i8) {
        if player == 0 {
            match input.dir_1 {
                Dir::Left => {
                    if self.dir != Dir::Right {
                        self.dir = Dir::Left;
                    }
                },
                Dir::Right => {
                    if self.dir != Dir::Left {
                        self.dir = Dir::Right;
                    }
                },
                Dir::Up => {
                    if self.dir != Dir::Down {
                        self.dir = Dir::Up;
                    }
                },
                Dir::Down => {
                    if self.dir != Dir::Up {
                        self.dir = Dir::Down;
                    }
                },
                Dir::Null => { },
            }
        } else if player == 1 {
            match input.dir_2 {
                Dir::Left => {
                    if self.dir != Dir::Right {
                        self.dir = Dir::Left;
                    }
                },
                Dir::Right => {
                    if self.dir != Dir::Left {
                        self.dir = Dir::Right;
                    }
                },
                Dir::Up => {
                    if self.dir != Dir::Down {
                        self.dir = Dir::Up;
                    }
                },
                Dir::Down => {
                    if self.dir != Dir::Up {
                        self.dir = Dir::Down;
                    }
                },
                Dir::Null => { },
            }
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
            Dir::Null => { }
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

    fn print (&self, head: u64, body: u64) {
        mvaddch(self.body[0].y, self.body[0].x, head);
        for block in &self.body[1..] {
            mvaddch(block.y, block.x, body);
        }
    }

    fn unprint (&self) {
        for block in &self.body {
            mvaddch(block.y, block.x, ' ' as u64);
        }
    }
}

pub struct Input {
    dir_1: Dir,
    dir_2: Dir,
    pub quit: bool,
}

impl Input {
    pub fn new () -> Input {
        let input = Input {
            dir_1: Dir::Null,
            dir_2: Dir::Null,
            quit: false,
        };
        input
    }

    pub fn handle_ch (&mut self, ch: i32) {
        match ch {
            KEY_LEFT => {
                self.dir_1 = Dir::Left;
            },
            KEY_RIGHT => {
                self.dir_1 = Dir::Right;
            },
            KEY_UP => {
                self.dir_1 = Dir::Up;
            },
            KEY_DOWN => {
                self.dir_1 = Dir::Down;
            },
            KEY_LEFT_2 => {
                self.dir_2 = Dir::Left;
            },
            KEY_RIGHT_2 => {
                self.dir_2 = Dir::Right;
            },
            KEY_UP_2 => {
                self.dir_2 = Dir::Up;
            },
            KEY_DOWN_2 => {
                self.dir_2 = Dir::Down;
            },
            QUIT => {
                self.quit = true;
            },
            _ => { },
        }
    }
    
    pub fn reset (&mut self) {
        self.dir_1 = Dir::Null;
        self.dir_2 = Dir::Null;
    }
}

pub fn print (player1: &Snake, player2: &Snake, egg: &Block) {
    player1.print(HEAD_1, BODY_1);
    player2.print(HEAD_2, BODY_2);
    egg.print(EGG);
}

pub fn unprint (player1: &Snake, player2: &Snake) {
    player1.unprint();
    player2.unprint();
}

pub fn update (input: &Input, player1: &mut Snake,
               player2: &mut Snake, egg: &mut Block) -> Option<Collision> {
    player1.update_dir(input, 0);
    player2.update_dir(input, 1);

    player1.update_pos();
    player2.update_pos();

    let col1 = player1.check_collision(egg, player2);
    let col2 = player2.check_collision(egg, player1);
   
    match (col1, col2) {
        (Some(Collision::Player1), Some(Collision::Player2)) => {
            return Some(Collision::Both);
        },
        (Some(Collision::Player1), _) => {
            return Some(Collision::Player1);
        },
        (_, Some(Collision::Player2)) => {
            return Some(Collision::Player2);
        },
        (Some(Collision::Egg), _) => {
            egg.regenerate();
            player1.new = true;
        },
        (_, Some(Collision::Egg)) => {
            egg.regenerate();
            player2.new = true;
        },
        _ => { },
    }
    None
}

