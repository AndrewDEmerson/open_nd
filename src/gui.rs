use minifb;
use rodio;
mod lib;
use lib::image;

fn main() {
    let audio_dev = rodio::default_output_device().unwrap();
    let audio_sink = rodio::Sink::new(&audio_dev);
    let audio_source = rodio::Decoder::new(std::io::BufReader::new(
        std::fs::File::open("sound.wav").unwrap(),
    ))
    .unwrap();
    audio_sink.append(audio_source);
    // Load the Game window frame, so that there is something to render
    let (frame_width, frame_height, buf) =
        image::decode_png(std::path::PathBuf::from("./FRAME.png"));
    // Create the screen object
    let mut screen = Screen {
        window_height: 480,
        window_width: 640,
        window: minifb::Window::new(
            "OpenND - Press ESC to exit",
            frame_width as usize,
            frame_height as usize,
            minifb::WindowOptions {
                resize: true,
                scale_mode: minifb::ScaleMode::Stretch,
                borderless: false,
                scale: minifb::Scale::FitScreen,
                ..minifb::WindowOptions::default()
            },
        )
        .expect("Unable to open Window"),
        buffer: buf
            .chunks(3)
            .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
            .collect(),
        bounding_boxes: Vec::new(),
        timers: Vec::new(),
    };
    //Create Bounding Boxes for the frame menu buttons
    screen.bounding_boxes.push(BoundBox {
        id: 1,
        upper_left: (595, 40),
        lower_right: (635, 122),
    });
    screen.bounding_boxes.push(BoundBox {
        id: 0,
        upper_left: (0, 35),
        lower_right: (50, 115),
    });
    screen.max_fps(std::time::Duration::from_secs(1 / 60));
    screen.window.set_position(650, 0);
    // Keep track of what the mouse status was last frame, so that we can act on a 'click'
    let mut last_mouse_status = (false, false, false);
    // Main game loop
    while screen.window.is_open() && !screen.window.is_key_down(minifb::Key::Escape) {
        // Draw Bounding Boxes for debug purposes
        if screen.window.is_key_down(minifb::Key::D)
            && screen.window.is_key_down(minifb::Key::LeftCtrl)
        {
            screen.draw_boxes();
        }
        // Draw a scene image onto the buffer
        if screen.window.is_key_down(minifb::Key::Key1) {
            screen.buffer_write(
                image::decode_png(std::path::PathBuf::from("./INTROA.png")),
                (52, 18),
            );
        }
        if screen.window.is_key_down(minifb::Key::Key2) {
            screen.buffer_write(
                image::decode_png(std::path::PathBuf::from("./INTROB.png")),
                (52, 18),
            );
        }
        if screen.window.is_key_down(minifb::Key::Key3) {
            screen.buffer_write(
                image::decode_png(std::path::PathBuf::from("./INTROC.png")),
                (52, 18),
            );
        }
        // Render the screen buffer to the window
        screen.update();
        // The mpuse positon is updated after the window is rendered
        // Calculate the x,y cordinates reguardless of the window size to be the the screen dimensions
        let pos_x: usize;
        let pos_y: usize;
        match screen.window.get_mouse_pos(minifb::MouseMode::Clamp) {
            Some((x, y)) => {
                let (realx, realy) = screen.window.get_size();
                pos_x = ((x / realx as f32) * screen.window_width as f32) as usize * 2;
                pos_y = ((y / realy as f32) * screen.window_height as f32) as usize * 2;
            }
            None => panic!("No mouse position"),
        }
        // Operate on mouse clicks to preform functions
        if last_mouse_status.0 == false && screen.window.get_mouse_down(minifb::MouseButton::Left) {
            last_mouse_status.0 = true;
            match screen.check_boundries(pos_x, pos_y) {
                Some(0) => panic!(),
                Some(id) => println!("No trigger set for Collsion box #{}", id),
                None => (),
            }
        } else if !screen.window.get_mouse_down(minifb::MouseButton::Left) {
            last_mouse_status.0 = false;
        }
        if last_mouse_status.2 == false && screen.window.get_mouse_down(minifb::MouseButton::Right)
        {
            last_mouse_status.2 = true;
            println!("Mouse pos is {},{}", pos_x, pos_y);
            screen.timers.push(Timer {
                begin: std::time::Instant::now(),
                duration: std::time::Duration::from_secs(1),
                draw: (
                    image::decode_png(std::path::PathBuf::from("./INTROC.png")),
                    (52, 18),
                ),
            });

        // wait(std::time::Duration::from_secs(5));
        } else if !screen.window.get_mouse_down(minifb::MouseButton::Right) {
            last_mouse_status.2 = false;
        }
    }
}

struct Timer {
    duration: std::time::Duration,
    begin: std::time::Instant,
    draw: ((u16, u16, Vec<u8>), (usize, usize)),
}

impl Timer {
    fn run(&mut self) -> bool {
        if self.begin.elapsed() >= self.duration {
            println!("Timer finished");
            return true;
        }
        false
    }
}

impl Timer {}

// The Screen manager must be created to use a window
pub struct Screen {
    // The height of the window in pixels
    window_height: usize,
    // The width ot the window in pixels
    window_width: usize,
    // The Window object, that contains Config for the rendering system
    window: minifb::Window,
    // Pixel buffer of form 0x00RRGGBB, must be >= window_height * window_width
    buffer: Vec<u32>,
    // Vector of BoundBox structs, that can be clicked on with the mouse
    bounding_boxes: Vec<BoundBox>,
    timers: Vec<Timer>,
}

impl Screen {
    fn update(&mut self) {
        self.window
            .update_with_buffer(
                &self.buffer,
                self.window_width as usize,
                self.window_height as usize,
            )
            .unwrap();
        for i in (0..self.timers.len()).rev() {
            if self.timers[i].run() {
                self.buffer_write(self.timers[i].draw.0.clone(), self.timers[i].draw.1);
                self.timers.remove(i);
            }
        }
    }
    // Limit the max frames per second to reduce cpu time
    fn max_fps(&mut self, delta: std::time::Duration) {
        self.window.limit_update_rate(Some(delta));
    }

    // Check if the mouse coordinated are inside one of the bounding boxes in the array
    fn check_boundries(&mut self, x: usize, y: usize) -> Option<usize> {
        for i in 0..self.bounding_boxes.len() {
            if self.bounding_boxes[i].upper_left.0 <= x
                && self.bounding_boxes[i].lower_right.0 >= x
                && self.bounding_boxes[i].upper_left.1 <= y
                && self.bounding_boxes[i].lower_right.1 >= y
            {
                return Some(self.bounding_boxes[i].id);
            }
        }
        println!("no object collision");
        None
    }
    // Draw all of the bounding boxes in the array to the screen buffer
    fn draw_boxes(&mut self) {
        for i in 0..self.bounding_boxes.len() {
            for x in self.bounding_boxes[i].upper_left.0..self.bounding_boxes[i].lower_right.0 {
                for y in self.bounding_boxes[i].upper_left.1..self.bounding_boxes[i].lower_right.1 {
                    let screen_pos = ((y as usize) * (self.window_width)) + x as usize;
                    self.buffer[screen_pos] = 0x00AF0000 | self.buffer[screen_pos];
                }
            }
        }
    }
    //Write an image to the screen buffer given its upper-left coordinate
    fn buffer_write(&mut self, image: (u16, u16, Vec<u8>), pos: (usize, usize)) {
        let pixels: Vec<u32> = image
            .2
            .chunks(3)
            .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
            .collect();
        let mut row = 0;
        let mut col = 0;
        for p in 0..pixels.len() {
            let start = ((pos.1 + row as usize) * (self.window_width)) + pos.0 as usize;
            self.buffer[start + col] = pixels[p];
            col += 1;
            if col == image.0 as usize {
                col = 0;
                row += 1;
            }
        }
    }
}

pub struct BoundBox {
    id: usize,
    upper_left: (usize, usize),
    lower_right: (usize, usize),
}
