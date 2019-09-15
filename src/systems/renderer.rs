use crate::components::{Position, Sprite};
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, Texture},
    video::Window,
};
use specs::{Join, ReadStorage, System};
use std::fmt;

pub struct Renderer<'t> {
    canvas: Canvas<Window>,
    textures: Vec<Texture<'t>>,
}

impl<'t> Renderer<'t> {
    pub fn new(mut canvas: Canvas<Window>, textures: Vec<Texture<'t>>) -> Self {
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        Self { canvas, textures }
    }
}

impl<'a, 't> System<'a> for Renderer<'t> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Sprite>);

    fn run(&mut self, data: Self::SystemData) {
        let (screen_width, screen_height) = self.canvas.output_size().unwrap();
        let world_to_screen_offset = Point::new(screen_width as i32 / 2, screen_height as i32 / 2);

        self.canvas.set_draw_color(Color::RGB(128, 128, 160));
        self.canvas.clear();

        let (pos, sprites) = data;

        for (pos, sprite) in (&pos, &sprites).join() {
            if let Some(cords) = &sprite.coordinates {
                // self.canvas.set_draw_color(Color::RGB(255, 255, 255));

                // Position
                let point = Point::new(pos.x, pos.y) + world_to_screen_offset;
                let rect = Rect::from_center(point, cords.width(), cords.height());

                // Sprite
                self.canvas
                    .copy(&self.textures[sprite.texture_id], sprite.coordinates, rect)
                    .unwrap();
            }
        }

        self.canvas.present();
    }
}

impl<'t> fmt::Debug for Renderer<'t> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Renderer").finish()
    }
}
