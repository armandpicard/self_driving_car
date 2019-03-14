extern crate sfml;
extern crate nalgebra as na;

pub mod car {
    use crate::Map;
    use sfml::graphics::{Color, Transformable, RenderWindow, RenderTarget,
                        RectangleShape, Sprite, Texture, Shape};
    use crate::Model;
    use na::DMatrix;
    pub struct Car {
        pub x: f32,
        pub y: f32,
        pub v: f32,
        pub angle: f32,
        pub a: f32,
        pub steering: f32,
        pub radar: [[u64; 7];8],
        pub alive: bool,
        pub d: f32,
    }


    pub fn overlaps(min1: f32, max1: f32, min2: f32, max2: f32) -> bool {
        if min1 > max2 || max1 < min2 {
            return false;
        } else {
            return true;
        }
    }

    pub fn project(axe: &na::Vector2<f32>, points: &Vec<na::Point2<f32>>, min: &mut f32, max: &mut f32) {
        *min = points.get(0).unwrap().x * axe.x + points.get(0).unwrap().y * axe.y;
        *max = *min;

        for a in points.iter() {
            let b = a.x * axe.x + a.y * axe.y;
            if b < *min {
                *min = b;
            } else if b > *max {
                *max = b;
            }
        }
    } 

    pub fn collide(rect1: &RectangleShape, rect2: &RectangleShape) -> bool {
        let mut p1: Vec<na::Point2<f32>> = Vec::new();
        let mut t1 = rect1.transform();
        for i in 0..4 {
            let p = rect1.point(i);
            let p = t1.transform_point(&p);
            p1.push(na::Point2::new(p.x, p.y));
        }
        let mut t2 = rect2.transform();
        let mut p2: Vec<na::Point2<f32>> = Vec::new();
        for i in 0..4 {
            let p = rect2.point(i);
            let p = t2.transform_point(&p);
            p2.push(na::Point2::new(p.x, p.y));
        }

        let mut axes:Vec<na::Vector2<f32>> = Vec::new();
        axes.push(na::Vector2::new(p1.get(0).unwrap().y - p1.get(1).unwrap().y, -(p1.get(0).unwrap().x - p1.get(1).unwrap().x)));
        axes.push(na::Vector2::new(p1.get(1).unwrap().y - p1.get(2).unwrap().y, -(p1.get(1).unwrap().x - p1.get(2).unwrap().x)));
        axes.push(na::Vector2::new(p1.get(2).unwrap().y - p1.get(3).unwrap().y, -(p1.get(2).unwrap().x - p1.get(3).unwrap().x)));
        axes.push(na::Vector2::new(p1.get(3).unwrap().y - p1.get(0).unwrap().y, -(p1.get(3).unwrap().x - p1.get(0).unwrap().x)));

        axes.push(na::Vector2::new(p2.get(0).unwrap().y - p2.get(1).unwrap().y, -(p2.get(0).unwrap().x - p2.get(1).unwrap().x)));
        axes.push(na::Vector2::new(p2.get(1).unwrap().y - p2.get(2).unwrap().y, -(p2.get(1).unwrap().x - p2.get(2).unwrap().x)));
        axes.push(na::Vector2::new(p2.get(2).unwrap().y - p2.get(3).unwrap().y, -(p2.get(2).unwrap().x - p2.get(3).unwrap().x)));
        axes.push(na::Vector2::new(p2.get(3).unwrap().y - p2.get(0).unwrap().y, -(p2.get(3).unwrap().x - p2.get(0).unwrap().x)));

        for a in axes.iter() {
            let mut min1 = 0.;
            let mut max1 = 0.;
            project(a, &p1, &mut min1, &mut max1);
            let mut min2 = 0.;
            let mut max2 = 0.;
            project(a, &p2, &mut min2, &mut max2);
            if !overlaps(min1, max1, min2, max2) {
                return false;
            }
        }
        return true;
    }

    impl Car {
        pub fn new(x: f32, y: f32, angle: f32) -> Car {
            Car {
                x: x,
                y: y,
                angle: angle,
                v: 0.,
                a: 0.,
                steering: 0.,
                alive: true,
                radar: [[0; 7];8],
                d: 0.,
            }
        }

        pub fn render(&self, window: &mut RenderWindow) {
            let mut rect_car = RectangleShape::new();
            rect_car.set_size((32.0 , 16.0));
            rect_car.set_origin((16., 8.));
            rect_car.set_rotation(-self.angle * (180.0 / 3.1415));
            rect_car.set_position((self.x, self.y));
            rect_car.set_fill_color(&Color::rgba(0, 255, 0, 255));
            window.draw(&rect_car);


            let texture = Texture::from_file("resource/car.png").unwrap();
            let mut sprite = Sprite::new();
            sprite.set_texture(&texture, true);
            sprite.set_origin((256., 128.));
            sprite.scale((0.0625, 0.0625));
            sprite.set_rotation(-self.angle * (180.0 / 3.1415));
            sprite.set_position((self.x, self.y));
            window.draw(&sprite);



            // self.render_radar(window);
        }

        pub fn render_radar(&self, window: &mut RenderWindow) {
            let mut rect = RectangleShape::new();
            rect.set_fill_color(&Color::rgba(255, 0, 0, 100));
            rect.set_size((16.0, 16.0));
            rect.set_origin((8.0, 8.0));
            rect.set_scale((0.9, 0.9));
            rect.set_rotation(-self.angle * 180.0/3.1415);
            for (i, col) in self.radar.iter().enumerate() {
                for (j, element) in col.iter().enumerate() {
                    if *element == 1 {
                        rect.set_fill_color(&Color::rgba(255, 0, 0, 150));
                    } else {
                        rect.set_fill_color(&Color::rgba(0, 0, 255, 150));
                    }
                    rect.set_position(((-8.0 + 16.0 * i as f32) * self.angle.cos() + (-48.0 + 16.0 * j as f32) * self.angle.sin() + self.x,
                                        (-8.0 + 16.0 * i as f32) * -self.angle.sin() + (-48.0 + 16.0 * j as f32) * self.angle.cos() + self.y));
                    window.draw(&rect);
                }
            }
        }
        pub fn update_radar(&mut self, map: &Map) {
            let mut rect_car = RectangleShape::new();
            rect_car.set_size((16.0, 16.0));
            rect_car.set_origin((8.0, 8.0));
            rect_car.set_scale((0.5, 0.5));
            rect_car.set_rotation(-self.angle * 180.0/3.1415);
            for (i, col) in self.radar.iter_mut().enumerate() {
                for (j, element) in col.iter_mut().enumerate() {
                    let x = (-8.0 + 16.0 * i as f32) * self.angle.cos() + (-48.0 + 16.0 * j as f32) * self.angle.sin() + self.x;
                    let y = (-8.0 + 16.0 * i as f32) * -self.angle.sin() + (-48.0 + 16.0 * j as f32) * self.angle.cos() + self.y;
                    let x: i32 = if x < 0.{0}else{x as i32 / 4};
                    let y: i32 = if y < 0.{0}else{y as i32 / 4};
                    let mut collide = false;
                    for a in &map.core[if x-2 < 0 {0}else if x-2 > 255 {255}else{x as usize - 2}..if x+3 < 0 {0}else if x+3 > 255 {255}else{x as usize + 3}] {
                        for b in &a[if y-2 < 0 {0}else if y-2 > 255 {255}else{y as usize - 2}..if y+3 < 0 {0}else if y+3 > 255 {255}else{y as usize + 3}] {
                            if *b == 1 {
                                collide = true;
                            }
                        }
                    }
                    if collide {
                        *element = 1;
                    } else {
                        *element = 0;
                    }
                }
            }
        }
        pub fn update(&mut self, map: &Map) {
            if self.alive {
                self.next_steep();
                self.update_radar(map);
                if self.collide(map) {
                    self.alive = false;
                }
            }
        }

        pub fn input(&mut self, model: &Model) {
            let mut input: DMatrix<f64> = DMatrix::new_random(58, 1);
            for (i, a) in input.iter_mut().enumerate() {
                *a = self.radar[i/8][i%7] as f64;
            }
            let result = model.get_result(&input);
            self.a = *result.get((0, 0)).unwrap() as f32;
            self.steering = (*result.get((1, 0)).unwrap() / 50.) as f32;
            // println!("a={}, steering={}", self.a, self.steering);
        }

        pub fn collide(&self, map: &Map) -> bool {
            let mut rect_car = RectangleShape::new();
            rect_car.set_size((16.0 + (self.angle.cos() * 16.0).abs(),
                            16.0 + (self.angle.sin() * 16.0).abs()));
            rect_car.set_position((self.x - 8.0 - (self.angle.cos() * 16.0 / 2.0).abs(),
                            self.y - 8.0 - (self.angle.sin() * 16.0 / 2.0).abs()));
            
            let mut rect_map = RectangleShape::new();
            rect_map.set_size((4., 4.));
            
            let x = self.x as i32 / 4;
            let y = self.y as i32 / 4;
            for i in if x - 4 < 0 {0} else if x - 4 > 255 {255} else {x - 4}..if x + 4 < 0 {0} else if x + 4 > 255 {255} else {x + 4} {
                for j in if y - 4 < 0 {0} else if y - 4 > 255 {255} else {y - 4}..if y + 4 < 0 {0} else if y + 4 > 255 {255} else {y + 4} {
                    let element = map.core[i as usize][j as usize];
                    rect_map.set_position((i as f32 * 4.0, j as f32 * 4.0));
                    if element == 1 && collide(&rect_car, &rect_map) {
                        return true;
                    }
                }
            }
            return false;
        }
        pub fn collide_new(&self, map: &Map) -> bool {
            let mut rect_car = RectangleShape::new();
            rect_car.set_size((16.0, 16.0));
            rect_car.set_origin((8.0, 8.0));
            rect_car.set_position((self.x, self.y));
            
            let mut rect_map = RectangleShape::new();
            rect_map.set_size((4., 4.));
            
            let x = self.x as i32 / 4;
            let y = self.y as i32 / 4;
            for i in if x - 4 < 0 {0} else if x - 4 > 255 {255} else {x - 4}..if x + 4 < 0 {0} else if x + 4 > 255 {255} else {x + 4} {
                for j in if y - 4 < 0 {0} else if y - 4 > 255 {255} else {y - 4}..if y + 4 < 0 {0} else if y + 4 > 255 {255} else {y + 4} {
                    let element = map.core[i as usize][j as usize];
                    rect_map.set_position((i as f32 * 4.0, j as f32 * 4.0));
                    if element == 1 && collide(&rect_car, &rect_map) {
                        return false;
                    }
                }
            }
            return false;
        }

        pub fn next_steep(&mut self) {
            self.angle += self.v * 2.0 * self.steering;
            self.v += self.a;
            if self.v <= 0.0 {
                self.v = 0.0;
            }
            self.x += self.angle.cos() * self.v;
            self.y += -self.angle.sin() * self.v;
            self.d += self.v;
        }
    }
}