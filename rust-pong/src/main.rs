use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::from_array([25f32, 130f32]);
const PLAYER_SPEED:f32 = 700f32;

// properly define player and implement functions
struct Player {
    rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self{
            rect: Rect::new(
                20f32,
                screen_height() * 0.5f32 - PLAYER_SIZE.y *0.5f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            )
        }
    }

    // literally draws on screen
    pub fn draw(&self) {        
        draw_rectangle(
            self.rect.x, 
            self.rect.y, 
            self.rect.w, 
            self.rect.h, 
            WHITE
        );
    }

    // update state
    // dt is a delta time to avoid framerate issues
    pub fn update(&mut self, dt: f32) {
        let mut y_move = 0f32;
        if is_key_down(KeyCode::Up){
            y_move -= 1f32
        }
        if is_key_down(KeyCode::Down){
            y_move += 1f32
        }
        self.rect.y += y_move * dt * PLAYER_SPEED;   
        
        // collision
        if self.rect.y <= 0f32{
            self.rect.y = 0f32
        } 
        if self.rect.y >= screen_height() - self.rect.h {
            self.rect.y = screen_height() - self.rect.h
        }
    }
}

// a dumb net
struct Net {
    rect: Rect
}
impl Net {
    pub fn new() -> Self { 
        Self{           
            rect: Rect::new(
                screen_width() / 2f32 - 1f32,
                0f32,
                2f32,
                screen_height(),
            )  
        }     
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x, 
            self.rect.y, 
            self.rect.w, 
            self.rect.h, 
            WHITE
        );
    }
}
// macro to add async functionality
#[macroquad::main("pong")]
async fn main() {
    let mut player = Player::new();
    let net = Net::new();
    // macroquad colors range from 0 to 1
    let bg_color = Color::new(0.37, 0.37, 0.37, 0.0);
    loop {
        //get frametime at any time to update for framedrops
        player.update(get_frame_time());
        // clear_background take macroquad::Color struct
        clear_background(bg_color);
        player.draw();
        net.draw();
        next_frame().await
    }
}
