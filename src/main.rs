use macroquad::prelude::{*, collections::storage::get};

const BLOCK_SIZE: Vec2 = Vec2::from_array([100f32, 40f32]);
const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);
const PLAYER_SPEED: f32 = 700.0;
const BALL_SIZE: f32 = 50f32;
const BALL_SPEED: f32 = 400f32;

pub fn draw_title_text(text: &str, font: Font) {
    let dims = measure_text(text, Some(font), 50u16, 1.0f32);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - dims.width * 0.5f32,
        screen_height() * 0.5f32 - dims.height * 0.5f32,
        TextParams{font, font_size: 50u16, color: BLACK, ..Default::default()}
    );
}

pub enum GameState {
    Menu,
    Game,
    LevelCompleted,
    Dead,
}

struct Player {
    rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x*0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }

    pub fn update(&mut self, dt: f32) {
        // key detect for movement
        let mut x_move: f32 = 0.0;
        // move if mouse is to the right or left of the player
        let (mouse_x, _mouse_y) = mouse_position();
        // left
        if mouse_x < self.rect.x + (self.rect.w / 2f32) {
            x_move -= 1.0;
        }
        // right
        if mouse_x > self.rect.x + (self.rect.w / 2f32) {
            x_move += 1.0;
        }
        // key movements
        /*
        if is_key_down(KeyCode::A) {
            x_move -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            x_move += 1.0;
        }
        */

        // update player x
        self.rect.x += x_move * dt * PLAYER_SPEED;

        // window edge collision checking
        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }
}

/*  18:30 - rust breakout game tutorial
pub enum BlockType {
    Regular,
}
*/

struct Block {
    rect: Rect,
    lives: i32,
}

impl Block {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            lives: 2,
        }
    }

    pub fn draw(&self) {
        let color = match self.lives {
            2 => RED,
            _ => ORANGE,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}

pub struct Ball {
    rect: Rect,
    vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        // update postion
        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;
        // wall collisions
        if self.rect.x < 0f32 {
            self.vel.x = 1f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.vel.x = -1f32;
        }
        if self.rect.y < 0f32 {
            self.vel.y = 1f32;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }
}

fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    // early exit
    let intersection = match a.intersect(*b) {
        Some(intersection) => intersection,
        None => return false,
    };
    let a_center = a.point() + a.size() * 0.5f32;
    let b_center = b.point() + b.size() * 0.5f32;
    let to = b_center - a_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            // bounce on y
            a.y -= to_signum.y * intersection.h;
            vel.y = -to_signum.y * vel.y.abs();
        }
        false => {
            // bounce on x
            a.x -= to_signum.x * intersection.w;
            vel.x = -to_signum.x * vel.x.abs();
        }
    }
    true
}

fn reset_game(
    score: &mut i32,
    player_lives: &mut i32,
    blocks: &mut Vec<Block>,
    balls: &mut Vec<Ball>,
    player: &mut Player,
    awaiting_ball: &mut bool,
) {
    *player = Player::new();
    *score = 0;
    *player_lives = 3;
    balls.clear();
    blocks.clear();
    init_blocks(blocks);
    *awaiting_ball = false;
    balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32)));
}

fn init_blocks(blocks: &mut Vec<Block>) {
    let (width, height) = (6, 6);
    let padding = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2((screen_width() - (total_block_size.x * width as f32)) * 0.5f32, 50f32);
    // create multiple blacks based on screen width and height
    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y)));
    }
}

fn window_config() -> Conf {
    Conf {
        window_title: "Breakout".to_owned(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let font = load_ttf_font("res/Heebo-VariableFont_wght.ttf").await.unwrap();
    let mut game_state = GameState::Menu;
    let mut score = 0;
    let mut player_lives = 3;
    let mut awaiting_ball = false;

    let mut player = Player::new();
    let mut blocks = Vec::new();
    let mut balls = Vec::new();

    // blocks code
    init_blocks(&mut blocks);

    balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32)));

    show_mouse(false);

    loop {
        

        // clear screen
        clear_background(WHITE);
        // draw player
        player.draw();
        // draw blocks
        for block in blocks.iter() {
            block.draw();
        }
        // draw balls
        for ball in balls.iter() {
            ball.draw();
        }

        /*
        draw_text_ex(
            &format!("{}", get_fps()),
            30.0,
            80.0,
            TextParams{font, font_size: 30u16, color: BLACK, ..Default::default()}
        );
        */

        match game_state {
            GameState::Menu => {
                draw_title_text("CLICK to start", font);
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    game_state = GameState::Game;
                }
            },
            GameState::Game => {
                /*
                if is_key_pressed(KeyCode::Space) {
                    balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32)));
                }
                */
                if awaiting_ball {
                    draw_title_text(&format!("CLICK to spawn new ball"), font);
                    if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                        balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32)));
                        awaiting_ball = false;
                    }
                }
        
                // update player
                player.update(get_frame_time());
                // update balls
                for ball in balls.iter_mut() {
                    ball.update(get_frame_time());
                }
        
                for ball in balls.iter_mut() {
                    resolve_collision(&mut ball.rect, &mut ball.vel, &player.rect);
                    for block in blocks.iter_mut() {
                        if resolve_collision(&mut ball.rect, &mut ball.vel, &block.rect) {
                            block.lives -= 1;
                            if block.lives <= 0 {
                                score += 10;
                            }
                        }
                    }
                }
        
                let balls_len = balls.len();
                let was_last_ball = balls_len == 1;
                balls.retain(|ball| ball.rect.y < screen_height());
                let removed_balls = balls_len - balls.len();
                if removed_balls > 0 && was_last_ball {
                    player_lives -= 1;
                    if player_lives <= 0 {
                        game_state = GameState::Dead;
                    }
                    awaiting_ball = true;
                    // balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32))); // load new ball when one dies
                }
        
                blocks.retain(|block| block.lives > 0);
                if blocks.is_empty() {
                    game_state = GameState::LevelCompleted;
                }

                let score_text = format!("score: {score}");
                let score_text_dim = measure_text(&score_text, Some(font), 30u16, 1.0);
                draw_text_ex(
                    &format!("score: {score}"),
                    screen_width() * 0.5f32 - score_text_dim.width * 0.5f32,
                    40.0,
                    TextParams{font, font_size: 30u16, color: BLACK, ..Default::default()}
                );
        
                draw_text_ex(
                    &format!("lives: {player_lives}"),
                    30.0,
                    40.0,
                    TextParams{font, font_size: 30u16, color: BLACK, ..Default::default()}
                );},
            GameState::LevelCompleted => {
                draw_title_text(&format!("you win! {score} score"), font);
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    game_state = GameState::Menu;
                    reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player, &mut awaiting_ball);
                }
            },
            GameState::Dead => {
                draw_title_text(&format!("you died! {score} score"), font);
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    game_state = GameState::Menu;
                    reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player, &mut awaiting_ball);
                }
            },
        }

        // advance to next frame
        next_frame().await
    }
}
