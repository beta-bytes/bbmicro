//#![windows_subsystem = "windows"]
use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS};

use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;

mod api;
use api::{BBMicroGame, InputState};

mod game1;
use game1::Game1;

struct PlayerInput {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    space: bool,
}

impl PlayerInput {
    fn new() -> PlayerInput {
        PlayerInput {
            up: false,
            down: false,
            left: false,
            right: false,
            space: false,
        }
    }
}

fn main() -> Result<(), String> {
    // Setup sdl core.
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "nearest");

    // Setup window.
    let window = video_subsystem
        .window("BBMicro", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // Setup audio.
    let _audio = sdl_context.audio();
    let frequency = 44_100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;
    sdl2::mixer::allocate_channels(4);
    let _mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::MP3);

    //TODO this works
    //let music = sdl2::mixer::Music::from_file("music.mp3").expect("Missing bgm music.mp3");
    //music.play(-1).expect("Failed to play bgm");

    // Setup canvas.
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    canvas
        .set_logical_size(128, 128)
        .expect("could not set logical size");

    let texture_creator = canvas.texture_creator();

    // Setup texture creators.
    let mut events = sdl_context.event_pump()?;

    let mut game = Game1::new();
    let mut api = api::BBMicroApi::new(&mut canvas, &texture_creator);

    // Setup the game.
    game.init(&mut api);

    'mainloop: loop {
        let mut input_state = InputState::new();
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'mainloop,
                Event::KeyDown {
                    keycode: Some(k), ..
                } => {
                    match k {
                        Keycode::Up => input_state.up_pressed = true,
                        Keycode::Down => input_state.down_pressed = true,
                        Keycode::Left => input_state.left_pressed = true,
                        Keycode::Right => input_state.right_pressed = true,
                        Keycode::A => input_state.a_pressed = true,
                        Keycode::B => input_state.b_pressed = true,
                        _ => {}
                    };
                }
                _ => {}
            }
        }

        let keys: Vec<Keycode> = events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        // Figure out
        if keys.contains(&Keycode::Up) {
            input_state.up_down = true;
        }
        if keys.contains(&Keycode::Down) {
            input_state.down_down = true;
        }
        if keys.contains(&Keycode::Left) {
            input_state.left_down = true;
        }
        if keys.contains(&Keycode::Right) {
            input_state.right_down = true
        }
        if keys.contains(&Keycode::A) {
            input_state.a_down = true;
        }
        if keys.contains(&Keycode::B) {
            input_state.b_down = true;
        }

        api.update_input(input_state);

        game.update(&mut api);

        api.cls(0);

        game.draw(&mut api);

        api.flip();

    }

    Ok(())
}
