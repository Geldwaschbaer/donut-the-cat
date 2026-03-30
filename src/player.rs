use crate::map::room::Room;

pub struct Player<'a> {
    position: &'a Room,
}

impl<'a> Player<'a> {
    pub fn new(position: &'a Room) -> Player<'a> {
        Player { position }
    }

    pub fn draw(&self) {}

    fn enter_room(&mut self, room: &'a Room) {
        self.position = room;
    }

    fn leave_room(&mut self, room: &'a Room) {}
}
