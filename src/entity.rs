use game_state;
use map;
use collision;

use::std::string::String;

pub struct Entity {
    pub pos_x: u32,
    pub pos_y: u32,
    pub facing_direction: f64,
    pub sprite_filename: String,
    pub sprite_collision: (u32, u32),
    pub visible: bool,
    pub has_collision: bool
}

impl Entity {
    pub fn init_bounding_box(&mut self, w: u32, h: u32) {
        self.sprite_collision = (w, h);
    }
}

pub trait Mobile {
    fn move_direction(&mut self, map: &map::Map, dir: game_state::Direction);
    fn move_to(&mut self, map: &map::Map, x: u32, y: u32);
    fn next_position(&mut self, map: &map::Map, dir: game_state::Direction) -> (u32, u32, bool);
}

pub struct Isuucc {
    pub entity: Entity,
    pub hp: u32,
    pub dmg: f32,
    pub speed: f32,
    pub tear_delay: f32
}

impl Isuucc {
    pub fn new(x: u32, y: u32) -> Isuucc {
        println!("Creating isuucc at position ({},{})", x, y);
        return
        Isuucc {
            entity: Entity {
                pos_x: x,
                pos_y: y,
                facing_direction: 1.0,
                sprite_filename: "assets/helivg.png".to_string(),
                sprite_collision: (0, 0), //placeholder until we load the texture
                visible: true,
                has_collision: false
            },
            hp: 6,
            dmg: 1.0,
            speed: 1.0,
            tear_delay: 1.0
        }
    }
}

pub struct Tear {
    pub entity: Entity,
    pub speed: f32,
    pub moving: bool
}


impl Tear {
    pub fn new(x: u32, y: u32) -> Tear {
        println!("Creating tear at position ({},{})", x, y);
        return
        Tear {
            entity: Entity {
                pos_x: x,
                pos_y: y,
                facing_direction: 0.0,
                sprite_filename: "assets/tear.png".to_string(),
                sprite_collision: (0, 0),
                visible: false,
                has_collision: false
            },
            speed: 2.0,
            moving: false
        }
    }

    fn move_direction(&mut self, map: &map::Map, dir: game_state::Direction) {
        let next_position: (u32, u32, bool) = self.next_position(map, dir); 
        self.entity.pos_x = next_position.0;
        self.entity.pos_y = next_position.1;
    }
}

impl Mobile for Isuucc {

    fn next_position(&mut self, map: &map::Map, dir: game_state::Direction) -> (u32, u32, bool) {
        let mut pos_x = self.entity.pos_x as i32;
        let mut pos_y = self.entity.pos_y as i32;
        match dir {
            game_state::Direction::Up => pos_y = pos_y - self.speed as i32,
            game_state::Direction::Down => pos_y = pos_y + self.speed as i32,
            game_state::Direction::Left => pos_x = pos_x - self.speed as i32,
            game_state::Direction::Right => pos_x = pos_x + self.speed as i32,
        }
        collision::collision_map(map, pos_x, pos_y, &mut self.entity, dir)
    }

    fn move_direction(&mut self, map: &map::Map, dir: game_state::Direction) {
        let next_position: (u32, u32, bool) = self.next_position(map, dir); 
        self.entity.pos_x = next_position.0;
        self.entity.pos_y = next_position.1;
        self.entity.has_collision = next_position.2
    }

    fn move_to(&mut self, map: &map::Map, x: u32, y: u32) {
        if x < self.entity.pos_x { self.move_direction(map, game_state::Direction::Up); }
        if x > self.entity.pos_x { self.move_direction(map, game_state::Direction::Down); }
        if y < self.entity.pos_y { self.move_direction(map, game_state::Direction::Left); }
        if y > self.entity.pos_y { self.move_direction(map, game_state::Direction::Right); }
    }

}

impl Mobile for Tear {

    fn next_position(&mut self, map: &map::Map, dir: game_state::Direction) -> (u32, u32, bool) {
        let mut pos_x = self.entity.pos_x as i32;
        let mut pos_y = self.entity.pos_y as i32;
        match dir {
            game_state::Direction::Up => pos_y = pos_y - self.speed as i32,
            game_state::Direction::Down => pos_y = pos_y + self.speed as i32,
            game_state::Direction::Left => pos_x = pos_x - self.speed as i32,
            game_state::Direction::Right => pos_x = pos_x + self.speed as i32,
        }
        collision::collision_map(map, pos_x, pos_y, &mut self.entity, dir)
    }

    fn move_direction(&mut self, map: &map::Map, dir: game_state::Direction) {
        let next_position: (u32, u32, bool) = self.next_position(map, dir); 
        self.entity.pos_x = next_position.0;
        self.entity.pos_y = next_position.1;
        self.entity.has_collision = next_position.2;
    }

    fn move_to(&mut self, map: &map::Map, x: u32, y: u32) {

    }

}