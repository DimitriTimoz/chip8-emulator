use sdl2::{video::Window, render::Canvas, pixels};

extern crate sdl2;

pub struct DisplayDriver {
    canvas: Canvas<Window>,
}

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;
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
        })
    }

    pub fn init(&mut self) -> Result<(), String> {
        Ok(())
    }

    pub fn draw(&mut self, vram: &[[bool; WIDTH as usize]; HEIGHT as usize]) -> Result<(), String> {
        self.canvas.set_draw_color(sdl2::pixels::Color::WHITE);
        self.canvas.clear();
        self.canvas.set_draw_color(sdl2::pixels::Color::BLACK);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if vram[y as usize][x as usize] {
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
}