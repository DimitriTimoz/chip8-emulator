use sdl2::{video::Window, render::Canvas, pixels};

extern crate sdl2;

pub struct DisplayDriver {
    canvas: Canvas<Window>,
    pixels: [[bool; WIDTH as usize]; HEIGHT as usize],
}

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const PIXEL_SIZE: u32 = 10;

impl DisplayDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, String>  {
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(
                "Chip8 Emulator",
                WIDTH * PIXEL_SIZE,
                HEIGHT * PIXEL_SIZE,
            )
            .position(0, 0)
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Ok(Self {
            canvas,
            pixels: [[false; WIDTH as usize]; HEIGHT as usize],
        })
    }

    pub fn init(&mut self) -> Result<(), String> {
        self.clear_screen()?;
        self.pixels = [[false; WIDTH as usize]; HEIGHT as usize];
        Ok(())
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.pixels[y][x]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        self.pixels[y][x] = value
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(sdl2::pixels::Color::WHITE);
        self.canvas.clear();
        self.canvas.set_draw_color(sdl2::pixels::Color::BLACK);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.pixels[y as usize][x as usize] {
                    self.canvas.fill_rect(sdl2::rect::Rect::new(
                        x as i32 * PIXEL_SIZE as i32,
                        y as i32 * PIXEL_SIZE as i32,
                        PIXEL_SIZE,
                        PIXEL_SIZE,
                    ))?;
                }
            }
        }
        self.canvas.present();

        Ok(())
    }


    pub fn clear_screen(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(sdl2::pixels::Color::WHITE);
        self.pixels = [[false; WIDTH as usize]; HEIGHT as usize];
        self.canvas.clear();

        self.canvas.present();
        Ok(())
    }
}