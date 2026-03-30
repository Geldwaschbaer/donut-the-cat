use crate::action::Action;
use macroquad::math::Vec2;
use std::boxed::Box;

pub struct Room {
    action: Box<dyn Action>,
    position: Vec2,
    neighbours: Vec<usize>,
}

impl Room {
    pub fn new(action: Box<dyn Action>, position: Vec2) -> Room {
        Room {
            action,
            position,
            neighbours: Vec::new(),
        }
    }

    pub fn with(action: Box<dyn Action>, position: Vec2, neighbours: Vec<usize>) -> Room {
        Room {
            action,
            position,
            neighbours,
        }
    }

    pub fn link_neighbour(&mut self, room: usize) {
        self.neighbours.push(room);
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_neighbours(&self) -> &Vec<usize> {
        &self.neighbours
    }
}
