extern crate sfml;
extern crate image;

pub mod map {
    use sfml::graphics::{Color, Transformable, RenderWindow, RenderTarget,
                    RectangleShape, Shape};
    use image::GenericImageView;
    use std::path::Path;
    pub struct Map {
        pub core: [[u64; 256]; 256],
        pub level: Level,
    }

    #[derive(Copy, Clone)]
    pub enum Level {
        Level1,
        Level2,
        Level3,
    }

    impl Map {
        pub fn from_level(level: Level) -> Map {
            let img = match level {
                Level::Level1 => {
                    image::open(&Path::new("resource/level1.png")).unwrap()
                },
                Level::Level2 => {
                    image::open(&Path::new("resource/level2.png")).unwrap()
                },
                Level::Level3 => {
                    image::open(&Path::new("resource/level3.png")).unwrap()
                }
            };
            let mut map = Map { core: [[0; 256]; 256], level: level};
            let (width, height) = img.dimensions();
            if width == 256 && height == 256 {
                for x in 0..width {
                    for y in 0..height {
                        let v = img.get_pixel(x, y).data;
                        if v[0] == 0 {
                            map.set_map(&(x as usize), &(y as usize), 1);
                        }
                    }
                }
            } else {
                panic!("wrong image size");
            }
            return map;
        }

        pub fn set_map(&mut self, x: &usize, y: &usize, value: u64) {
            self.core[*x][*y] = value;
        }

        pub fn get_map(&self, x: &f32, y: &f32) -> u64 {
            if *x < 0. || *y < 0. || *x > 1080. || *y > 1080. {
                return 0;
            } else {
                self.core[*x as usize % 4][*y as usize % 4]
            }
        }

        pub fn render(&self, window: &mut RenderWindow) {
            for a in 0..256 {
                for b in 0..256 {
                    if self.core[a][b] == 1 {
                        let mut rect = RectangleShape::new();
                        rect.set_size((4.0, 4.0));
                        rect.set_position((a as f32 * 4.0, b as f32 * 4.0));
                        rect.set_fill_color(&Color::RED);
                        window.draw(&rect);
                    }
                }
            }
        }
    }
}