use raylib::prelude::*;
use std::time::Instant;
struct Clock {
    last: Instant,
    elapsed: f64,
}

impl Clock {
    const MAX_DELTA: f64 = 0.05;

    fn new() -> Self {
        Self {
            last: Instant::now(),
            elapsed: 0.0,
        }
    }

    fn tick(&mut self) -> f64 {
        let now = Instant::now();
        let delta = (now - self.last).as_secs_f64().clamp(0.0, Self::MAX_DELTA);

        self.last = now;
        self.elapsed += delta;
        delta
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    // init aduio
    let ra = RaylibAudio::init_audio_device().unwrap();
    ra.set_master_volume(1.0);

    let music = ra
        .new_music("./resources/sound/AlexBouncyMaster.ogg")
        .unwrap();
    let bells_sound = ra.new_sound("./resources/sound/bells.wav").unwrap();

    music.play_stream();

    let mut clock = Clock::new();

    let tex: Texture2D = rl
        .load_texture(&thread, "./resources/textures/daxbotsheet.png")
        .unwrap();

    // let mut frame_row: f32 = 0.0;
    // let mut frame_column: f32 = 0.0;

    let frame_width: f32 = 64.0;
    let frame_height: f32 = 68.0;

    let frame_duration_seconds = 0.15;

    let mut frame_time = 0.0;
    let mut current_frame: usize = 0;
    let mut flipped = false;

    let walk_frames = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
    let run_frames = vec![(1, 1), (1, 2), (1, 3)];

    let mut postion = Vector2::new(50.0, 100.0);
    let mut facing_right = true;
    let mut current_state = "none";

    while !rl.window_should_close() {
        // must be called to continue streaming sound from the sound file.
        // music.update_stream();

        // if rl.is_key_pressed(KeyboardKey::KEY_F) {
        //     println!("flipping");
        //     flipped = !flipped;
        //     if !bells_sound.is_playing() {
        //         bells_sound.play();
        //     }
        // }

        let delta = clock.tick() as f32;

        if rl.is_key_released(KeyboardKey::KEY_F) {
            println!("F key released.")
        }

        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
            facing_right = false;
            current_state = "walk";
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            postion.x += -150.0 * delta;
        }
        if rl.is_key_released(KeyboardKey::KEY_LEFT) {
            current_state = "none";
        }

        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            facing_right = true;
            current_state = "walk";
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            postion.x += 150.0 * delta;
        }
        if rl.is_key_released(KeyboardKey::KEY_RIGHT) {
            current_state = "none";
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

        // sheet rectangle, what to pull from the texture.
        frame_time += delta;
        if frame_time >= frame_duration_seconds {
            current_frame = (current_frame + 1) % walk_frames.len();
            frame_time = 0.0;
        }

        let r1;
        if current_state == "none" {
            let mut width = frame_width;
            if !facing_right {
                width = -width;
            }
            r1 = Rectangle::new(
                    frame_width * 0 as f32,
                    frame_height * 0 as f32,
                    width,
                    frame_height,
                );
        } else {
            //if current_state == "walk"  {
                let (r, c) = walk_frames[current_frame];
            //}
            

            if facing_right {
                r1 = Rectangle::new(
                    frame_width * c as f32,
                    frame_height * r as f32,
                    frame_width,
                    frame_height,
                );
            } else {
                r1 = Rectangle::new(
                    frame_width * c as f32,
                    frame_height * r as f32,
                    -1.0 * frame_width,
                    frame_height,
                );
            }
        }

        // screen rectangl3, where in the world.
        let r2 = Rectangle::new(postion.x, postion.y, 64.0, 68.0);
        let origin = Vector2::new(frame_width / 2.0, frame_height - 8.0);
        //let origin = Vector2::new(0.0, 0.0);

        d.draw_texture_pro(&tex, r1, r2, origin, 0.0, Color::WHITE);

        d.draw_circle(50, 100, 5.0, Color::GRAY);
    }
}
