use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use super::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.00];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::UP => Direction::DOWN,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
            Direction::DOWN => Direction::UP
        }
    }
}

#[derive(Clone, Debug)]
struct Block {
    x: i32,
    y: i32
}


pub struct Snake {
    body: LinkedList<Block>,
    direction: Direction,
    tail: Option<Block>
}

impl Snake {
    pub fn new(x: i32, y:i32) -> Snake{
        let mut body: LinkedList<Block>  = LinkedList::new();
        body.push_back(Block {
            x: x + 2,
            y
        });
        body.push_back(Block {
            x: x + 1,
            y,
        });
        body.push_back(Block {
            x,
            y
        });
        Snake {
            direction: Direction::RIGHT,
            body,
            tail: None
        }
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d){

        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, ctx, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, direction: Option<Direction>){

        match direction {
            Some(d) => self.direction = d,
            None => ()
        }

        let (last_x, last_y) = self.head_position();

        let new_block = match self.direction {
            Direction::UP => Block {
                x: last_x,
                y: last_y - 1
            },
            Direction::DOWN => Block {
                x: last_x,
                y: last_y + 1
            },
            Direction::LEFT => Block {
                x: last_x - 1,
                y: last_y
            },
            Direction::RIGHT => Block {
                x: last_x + 1,
                y: last_y
            }
        };
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn  head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) = self.head_position();

        let mut moving_direction = self.direction;

        match dir {
            Some(d) => moving_direction = d,
            None => {}
        };

        match moving_direction {
            Direction::UP => (head_x, head_y - 1),
            Direction::DOWN => (head_x, head_y + 1),
            Direction::RIGHT => (head_x + 1, head_y),
            Direction::LEFT => (head_x - 1, head_y)
        }
    }

    /// This method is called when the snake eats the apple
    /// It makes the length of the snake grow
    pub  fn restore_tail(&mut self){
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    pub fn if_tail_overlap(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if block.x == x && block.y == y {
                return true;
            }
            ch +=1;
            if ch == self.body.len() - 1 {
                break;
            }
        }
        false
    }
}
