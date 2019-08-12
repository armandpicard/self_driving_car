extern crate sfml;
extern crate image;
extern crate nalgebra as na;
extern crate rand;

use rand::Rng;

mod model;
mod env;

use model::model::{Model, Layer, Activation};
use env::map::map::{Map, Level};
use env::car::car::{Car};
use sfml::graphics::{Color, RenderWindow, RenderTarget, RectangleShape};
use sfml::window::{Event, Key, Style};

use std::sync::mpsc;
use std::{thread, time};

fn get_event(window: &mut RenderWindow) -> bool{
    while let Some(event) = window.poll_event() {
        match event {
            Event::Closed | Event::KeyPressed {
                code: Key::Escape, ..
            } => return true,
            _ => {}
        }
    }
    return false;
}

fn render(window: &mut RenderWindow, map: &Map) {
    map.render(window);
}

fn get_best_of_gen_show(window: &mut RenderWindow, map: &Map, models: Vec<Model>) -> Model{
    let mut cars: Vec<Car> = Vec::new();
    let mut n = 0;

    for _ in models.iter() {
        cars.push(Car::new(500.0, 500., 0.0));
    }
    loop {
        if get_event(window) {
            panic!();
        }
        for (i, a) in cars.iter_mut().enumerate() {
            a.input(models.get(i).unwrap());
            a.update(&map);
        }
        
        let mut vie = false;
        for a in cars.iter() {
            if a.alive && a.v >= 0.1 {
                vie = true;
            }
        }
        if !vie || n > 200 {
            println!("tout est mort ou on a fini: n={}", n);
            let mut best = 0;
            for (i, a) in cars.iter().enumerate() {
                if a.d >= cars.get(best).unwrap().d {
                    best = i;
                }
            }
            return models.get(best).unwrap().clone();
        }
        window.clear(&Color::WHITE);
        for a in cars.iter_mut() {
            a.render(window);
        }
        render(window, &map);
        window.display();
        n = n + 1;
        //println!(" n={}", n);
    }
}

fn get_best_of_gen(map: &Map, models: Vec<Model>) -> Model{
    let mut cars: Vec<Car> = Vec::new();
    let mut n = 0;

    for _ in models.iter() {
        cars.push(Car::new(500.0, 500., 0.0));
    }
    loop {
        for (i, a) in cars.iter_mut().enumerate() {
            a.input(models.get(i).unwrap());
            a.update(&map);
        }
        
        let mut vie = false;
        for a in cars.iter() {
            if a.alive && a.v >= 0.1 {
                vie = true;
            }
        }
        if !vie || n > 300 {
            let mut best = 0;
            for (i, a) in cars.iter().enumerate() {
                if a.d >= cars.get(best).unwrap().d {
                    best = i;
                }
            }
            println!("tout est mort ou on a fini: n={}, d={}", n, cars.get(best).unwrap().d);
            return models.get(best).unwrap().clone();
        }
        n = n + 1;
    }
}

fn get_n_best_of_gen(map: &Map, models: Vec<Model>, num: usize) -> Vec<(Model, usize)>{
    let mut cars: Vec<Car> = Vec::new();
    let mut n = 0;
    let mut my_models: Vec<(Model, usize)> = Vec::new();
    for _ in models.iter() {
        cars.push(Car::new(500.0, 500., 0.0));
    }

    for m in models {
        my_models.push((m, 0));
    }

    loop {
        for (i, a) in cars.iter_mut().enumerate() {
            a.input(&my_models.get(i).unwrap().0);
            a.update(&map);
        }
        
        let mut vie = false;
        for a in cars.iter() {
            if a.alive && a.v >= 0.1 {
                vie = true;
            }
        }
        if !vie || n > 500 {
            // while models.len() > num {
            //     let mut worst = 0;
            //     for (i, a) in cars.iter().enumerate() {
            //         if a.d <= cars.get(worst).unwrap().d {
            //             worst = i;
            //         }
            //     }
            //     cars.remove(worst);
            //     models.remove(worst);
            // }
            for (i, a) in cars.iter().enumerate() {
                my_models[i].1 = a.d as usize;
            }

            my_models.sort_by(|a, b| b.1.cmp(&a.1));
            my_models.truncate(num);
    
            //println!("tout est mort ou on a fini: n={}, worst={} ,best={}", n, worst, best);
            return my_models;
        }
        n = n + 1;
    }
}

fn show_model(window: &mut RenderWindow, map: &Map, model: &Model) {
    let mut car: Car = Car::new(500.0, 500., 0.0);
    let mut n = 0;

    loop {
        if get_event(window) {
            panic!();
        }
        car.input(model);
        car.update(map);
        
        let vie = car.alive;
        if !vie {
            return;
        }
        window.clear(&Color::WHITE);
        render(window, &map);
        car.render(window);
        window.display();
        n = n + 1;
    }
}

fn show_models(window: &mut RenderWindow, map: &Map, models: &Vec<Model>) {
    let mut cars: Vec<Car> = Vec::new();
    let mut n = 0;

    for _ in models.iter() {
      cars.push(Car::new(500.0, 500.0, 0.0))  
    }
    loop {
        let start = std::time::Instant::now();
        if get_event(window) {
            panic!();
        }
        for (i, car) in cars.iter_mut().enumerate() {
            car.input(models.get(i).unwrap());
            car.update(map);
        }
        let mut vie = false;
        for car in cars.iter() {
            if car.alive {
                vie = true;
            }
        }
        if !vie {
            return;
        }
        window.clear(&Color::WHITE);
        render(window, &map);
        for car in cars.iter() {
            car.render(window);
        }
        window.display();
        let dt = start.elapsed();
        match std::time::Duration::new(0, 33333333).checked_sub(dt) {
            Some(t) => {
                println!("sleep for {:?}", t);
                thread::sleep(t);
            },
            None => {
                println!("je ne sleep pas {:?}", dt);
            },
        };
        n = n + 1;
    }
}

fn train_with_genetic(map: &Map, model: Model) -> Vec<Model> {
    let mut models: Vec<Model> = Vec::new();

    //initial population init random
    for _ in 0..500 {
        models.push(model.copy_mut(1.0, 3.0));
    }
    //get the 5 best of the first generation to create more like those
    let mut bests = get_n_best_of_gen(&map, models, 8);

    //for each generation we get the 8 best and create more like those
    for generation in 1..20 {
        println!("start generation: {}", generation);
        let mut my_threads = Vec::new();
        let mut my_rx = Vec::new();
        let level = map.level;

        let chunk = bests.chunks(bests.len() / 8);
        for best in bests {
            let (tx, rx) = mpsc::channel();
            my_rx.push(rx);
            my_threads.push(thread::spawn(move || {
                let my_map = Map::from_level(level);
                let mut my_models = Vec::new();
                my_models.push(best.0.clone());
                //create 100 child model from one of the bests if the last generation
                for _ in 0..500 {
                    my_models.push(best.0.copy_mut(0.05, 0.01));
                }
                for _ in 0..500 {
                    my_models.push(best.0.copy_mut(0.05, 0.5));
                }
                for _ in 0..500 {
                    my_models.push(best.0.copy_mut(0.05, 0.5));
                }

                let bests_current = get_n_best_of_gen(&my_map, my_models, 2);
                tx.send(bests_current).unwrap();
            }));
        }
        bests = Vec::new();
        //wait for for all thread
        for thrd in my_threads {
            thrd.join().unwrap();
        }
        //get bests of each thread
        for rx in my_rx.iter() {
            let best_rec = rx.recv().unwrap();
            for a in best_rec {
                bests.push(a);
            }
        }
        bests.sort_by(|a, b| b.1.cmp(&a.1));
        bests.truncate(8);
    };
    let mut result = Vec::new();
    for m in bests {
        result.push(m.0);
    }
    return result;
}

fn main() {
    let mut window = RenderWindow::new(
        (1024, 1024),
        "Self driving car",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(30);
    window.set_vertical_sync_enabled(true);

    let map = Map::from_level(Level::Level1);
    let mut model = Model::new();
    let l1 = Layer::new_random(58, 20, Activation::Sigmoid);
    // let l2 = Layer::new_random(20, 20, Activation::Sigmoid);
    let l3 = Layer::new_random(20, 10, Activation::Sigmoid);
    // let l4 = Layer::new_random(10, 10, Activation::Sigmoid);
    let l5 = Layer::new_random(10, 2, Activation::Tanh);    
    model.add_layer(l1);
    // model.add_layer(l2);
    model.add_layer(l3);
    // model.add_layer(l4);
    model.add_layer(l5);

    let best = train_with_genetic(&map, model);

    let unic_best = get_best_of_gen(&map, best);
    
    loop {
        show_model(&mut window, &map, &unic_best);
    }
}

