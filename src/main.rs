use macroquad::prelude::*;

const GRAVITY: f32 = 800.0;

struct Player {
    pos: Vec2,
    vel: Vec2,
    size: Vec2,
    speed: f32,
    grounded: bool,
}

impl Player {
    fn update(&mut self, dt: f32) {
        if self.grounded == true {
            self.vel.x *= 0.96;
        }
        self.vel.y += GRAVITY * dt;
        self.pos += self.vel * dt;
        draw_rectangle(self.pos.x, self.pos.y, self.size.x, self.size.y, BLACK);
    }
    fn collide(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, self.size.x, self.size.y)
    }
    fn input(&mut self) {
        if is_key_down(KeyCode::D) {
            self.vel.x += self.speed
        }
        if is_key_down(KeyCode::A) {
            self.vel.x += -self.speed
        }
        if is_key_pressed(KeyCode::Space) && self.grounded == true {
            self.vel.y -= 440.0
        }
    }
}

struct Block {
    pos: Vec2,
    size: Vec2,
    color: Color,
}

impl Block {
    fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.size.x, self.size.y, self.color);
    }
    fn collide(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, self.size.x, self.size.y)
    }
}
#[macroquad::main("MyGame")]
async fn main() {
    let mut player = Player {
        pos: vec2(100.0, 100.0),
        vel: vec2(0.0, 0.0),
        size: vec2(25.0, 25.0),
        speed: 15.0,
        grounded: false,
    };

    let blocks = vec![
        Block {
            pos: vec2(100.0, 300.0),
            size: vec2(500.0, 10.0),
            color: RED,
        },
        Block {
            pos: vec2(200.0, 101.0),
            size: vec2(500.0, 10.0),
            color: RED,
        },
    ];

    loop {
        let dt = get_frame_time();
        clear_background(WHITE);
        player.grounded = false;
        for b in &blocks {
            if let Some(i) = player.collide().intersect(b.collide()) {
                if i.w < i.h {
                    // horizontal
                    if player.pos.x < b.pos.x {
                        player.pos.x -= i.w;
                    } else {
                        player.pos.x += i.w;
                    }
                    player.vel.x = 0.0;
                } else {
                    // vertical
                    if player.pos.y < b.pos.y {
                        player.pos.y -= i.h;
                        player.grounded = true;
                    } else {
                        player.pos.y += i.h;
                    }
                    player.vel.y = 0.0;
                }
            }
            b.draw();
        }

        player.input();
        player.update(get_frame_time());
        next_frame().await;
    }
}
