extern crate nalgebra as na;


pub mod model {
    use na::DMatrix;
    use rand::prelude::*;
    #[derive(Debug)]
    pub enum Activation {
        Relu,
        Sigmoid,
        Tanh,
        ArcTan,
        Binaty,
        Logistic(f64, f64, f64),
        Softsign,
    }

    impl Clone for Activation {
        fn clone(&self) -> Activation { 
            match *self {
                Activation::Relu => Activation::Relu,
                Activation::Sigmoid => Activation::Sigmoid,
                Activation::Tanh => Activation::Tanh,
                Activation::ArcTan => Activation::ArcTan,
                Activation::Binaty => Activation::Binaty,
                Activation::Logistic(l, k, x0) => Activation::Logistic(l, k, x0),
                Activation::Softsign => Activation::Softsign,
            }
        }
    }

    pub fn signoid(matrix: &DMatrix<f64>) -> DMatrix<f64> {
        let mut r: DMatrix<f64> = matrix.clone();
        for a in r.iter_mut() {
            *a = 1.0/(1.0 + a.exp());
        }
        return r;
    }

    pub fn relu(matrix: &DMatrix<f64>) -> DMatrix<f64> {
        let mut r: DMatrix<f64> = matrix.clone();
        for a in r.iter_mut() {
            if *a < 0. {
                *a = 0.;
            }
        }
        return r;
    }

    pub fn tanh(matrix: &DMatrix<f64>) -> DMatrix<f64> {
        let mut r: DMatrix<f64> = matrix.clone();
        for a in r.iter_mut() {
            *a = a.tanh();
        }
        return r;
    }

    pub fn arctan(matrix: &DMatrix<f64>) -> DMatrix<f64> {
        let mut r: DMatrix<f64> = matrix.clone();
        for a in r.iter_mut() {
            *a = a.atan();
        }
        return r;
    }

    pub fn binary(matrix: &DMatrix<f64>) -> DMatrix<f64> {
        let mut r: DMatrix<f64> = matrix.clone();
        for a in r.iter_mut() {
            if *a < 0. {
                *a = 0.;
            } else {
                *a = 1.;
            }
        }
        return r;
    }

    pub fn logistic(matrix: &DMatrix<f64>, l: f64, k: f64, x0: f64) -> DMatrix<f64> {
        let mut r: DMatrix<f64> = matrix.clone();
        for a in r.iter_mut() {
            *a = l / (1. + (-k * (*a - x0)).exp());
        }
        return r;
    }

    pub fn softsign(matrix: &DMatrix<f64>) -> DMatrix<f64> {
        let mut r: DMatrix<f64> = matrix.clone();
        for a in r.iter_mut() {
            *a = *a / (1. + (*a).abs());
        }
        return r;
    }

    #[derive(Debug)]
    pub struct Layer {
        input: u32,
        neuron: u32,
        w: DMatrix<f64>,
        b: DMatrix<f64>,
        a: Activation,
    }

    impl Clone for Layer {
        fn clone(&self) -> Layer {
            Layer {
                input: self.input,
                neuron: self.neuron,
                w: self.w.clone(),
                b: self.b.clone(),
                a: self.a.clone(),
            }
        }
    }

    impl Layer {
        pub fn new_random(input: u32, neuron: u32, activation: Activation) -> Layer {
            Layer {
                input: input,
                neuron: neuron,
                w: DMatrix::new_random(neuron as usize, input as usize),
                b: DMatrix::new_random(neuron as usize, 1),
                a: activation,
            }
        }

        pub fn get_result(&self, input: &DMatrix<f64>) -> DMatrix<f64> {
            let preactivation = &self.w * input + &self.b;
            match self.a {
                Activation::Sigmoid => signoid(&preactivation),
                Activation::Relu => relu(&preactivation),
                Activation::Tanh => tanh(&preactivation),
                Activation::ArcTan => arctan(&preactivation),
                Activation::Binaty => binary(&preactivation),
                Activation::Logistic(l, k, x0) => logistic(&preactivation, l, k, x0),
                Activation::Softsign => softsign(&preactivation),
            }
        }
    }
    #[derive(Debug)]
    pub struct Model {
        layers: Vec<Layer>,
    }

    impl Clone for Model {
        fn clone(&self) -> Model {
            let mut r = Model {
                layers: Vec::new(),
            };
            for a in self.layers.iter() {
                r.layers.push(a.clone());
            }
            return r;
        }
    }

    impl Model {
        pub fn new() -> Model {
            Model {
                layers: Vec::new(),
            }
        }

        pub fn make_a_child(&self, papa: &Model, percent: f64, delta_max: f64) -> Model{
            let mut model = self.clone();
            for (i, a) in model.layers.iter_mut().enumerate() {
                let layer_papa = papa.layers.get(i).unwrap();
                for (j, b) in a.w.iter_mut().enumerate() {
                    let mut p:f64 = rand::random();
                    if p < 0.5 {
                        *b = *layer_papa.w.get(j).unwrap();
                    }
                    p = rand::random();
                    if p < percent {
                        p = rand::random();
                        if p < 0.5 {
                            p = rand::random();
                            *b = *b + p * delta_max;
                        } else {
                            p = rand::random();
                            *b = *b - p * delta_max;
                        }
                    }
                }
                for (j, b) in a.b.iter_mut().enumerate() {
                    let mut p:f64 = rand::random();
                    if p < 0.5 {
                        *b = *layer_papa.b.get(j).unwrap();
                    }
                    p = rand::random();
                    if p < percent {
                        p = rand::random();
                        if p < 0.5 {
                            p = rand::random();
                            *b = *b + p * delta_max;
                        } else {
                            p = rand::random();
                            *b = *b - p * delta_max;
                        }
                    }
                }
            }
            return model;
        }

        pub fn copy_mut(&self, percent: f64, delta_max: f64) -> Model {
            let mut model = self.clone();
            for a in model.layers.iter_mut() {
                for b in a.w.iter_mut() {
                    let mut p:f64 = rand::random();
                    if p < percent {
                        p = rand::random();
                        if p < 0.5 {
                            p = rand::random();
                            *b = *b + p * delta_max;
                        } else {
                            p = rand::random();
                            *b = *b - p * delta_max;
                        }
                    }
                }
                for b in a.b.iter_mut() {
                    let mut p:f64 = rand::random();
                    if p < percent {
                        p = rand::random();
                        if p < 0.5 {
                            p = rand::random();
                            *b = *b + p * delta_max;
                        } else {
                            p = rand::random();
                            *b = *b - p * delta_max;
                        }
                    }
                }
            }
            return model;
        }

        pub fn add_layer(&mut self, layer: Layer) {
            self.layers.push(layer);
        }

        pub fn get_result(&self, input: &DMatrix<f64>) -> DMatrix<f64> {
            let mut temp: DMatrix<f64> = input.clone();
            for a in self.layers.iter() {
                temp = a.get_result(&temp);
            }
            return temp;
        }
    }
}