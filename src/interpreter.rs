use std::collections::VecDeque;

use crate::ast::{Ops, Program};
use fxhash::FxHashMap;
use rand::RngExt;

const EPS_PRECISION: f32 = 0.000001;

#[derive(Debug)]
pub struct State {
    current_register: f32,
    storage: FxHashMap<usize, f32>,
    finished: bool,
    jumped: bool,
    input: VecDeque<f32>,
    ip: usize,
}

impl State {
    pub fn new(input: VecDeque<f32>) -> Self {
        Self {
            current_register: 0.0,
            storage: FxHashMap::default(),
            finished: false,
            jumped: false,
            input,
            ip: 0,
        }
    }

    fn execute_op(&mut self, op: &Ops) {
        let get_value = |&n| self.storage.get(&n).cloned().unwrap_or_default();
        match op {
            Ops::Stop => self.finished = true,
            Ops::Nop => (),
            Ops::Read => {
                self.current_register = self.input.pop_front().expect("Called READ on empty input")
            }
            Ops::Print => println!("{}", self.current_register),
            Ops::Write(s) => println!("{}", s),
            Ops::Store(n) => {
                self.storage.insert(*n, self.current_register);
            }
            Ops::Const(f) => self.current_register = *f,
            Ops::Recall(n) => {
                if let Some(n) = n {
                    self.current_register = get_value(n)
                } else {
                    let n = self.current_register.round() as usize;
                    self.current_register = get_value(&n)
                }
            }
            Ops::Swap(n) => {
                self.current_register = self
                    .storage
                    .insert(*n, self.current_register)
                    .unwrap_or_default();
            }
            Ops::Copy(n) => {
                let value = get_value(n);
                let n = self.current_register.round() as usize;
                self.storage.insert(n, value).unwrap();
            }

            Ops::Add(n) => {
                let value = get_value(n);
                self.current_register += value;
            }
            Ops::Sub(n) => {
                let value = get_value(n);
                self.current_register -= value;
            }
            Ops::Mul(n) => {
                let value = get_value(n);
                self.current_register *= value;
            }
            Ops::Div(n) => {
                let value = get_value(n);
                self.current_register /= value;
            }
            Ops::Sign => self.current_register = self.current_register.signum(),
            Ops::Abs => self.current_register = self.current_register.abs(),
            Ops::Sqrt => self.current_register = self.current_register.sqrt(),
            Ops::Exp => self.current_register = self.current_register.exp(),
            Ops::Log => self.current_register = self.current_register.ln(),
            Ops::Sin => self.current_register = self.current_register.sin(),
            Ops::Cos => self.current_register = self.current_register.cos(),
            Ops::Tan => self.current_register = self.current_register.tan(),
            Ops::Floor => self.current_register = self.current_register.floor(),
            Ops::Ceil => self.current_register = self.current_register.ceil(),
            Ops::Trunc => self.current_register = self.current_register.trunc(),
            Ops::Round => self.current_register = self.current_register.round(),
            Ops::Rand => {
                self.current_register = {
                    let mut rng = rand::rng();
                    rng.random()
                }
            }
            Ops::Jump(n) => {
                if let Some(n) = n {
                    self.ip = *n;
                } else {
                    self.ip = self.current_register.round() as usize;
                }
                self.jumped = true;
            }
            Ops::Positive(n) => {
                if self.current_register.is_sign_positive() {
                    self.ip = *n;
                    self.jumped = true;
                }
            }
            Ops::Negative(n) => {
                if self.current_register.is_sign_negative() {
                    self.ip = *n;
                    self.jumped = true;
                }
            }
            Ops::Zero(n) => {
                if self.current_register.abs() < EPS_PRECISION {
                    self.ip = *n;
                    self.jumped = true;
                }
            }
        }
    }

    pub fn execute(&mut self, program: &Program) {
        while !self.finished {
            let instruction = program
                .get(self.ip)
                .expect("Program has run out of operations");
            self.execute_op(instruction);
            if self.jumped {
                self.jumped = false;
            } else {
                self.ip += 1;
            }
        }
    }

    pub fn execute_print_state(&mut self, program: &Program) {
        println!("Program: {program:?}");
        while !self.finished {
            let instruction = program
                .get(self.ip)
                .expect("Program has run out of operations");
            println!("Current state: {self:?}");
            println!("Current OP: {instruction:?}");
            self.execute_op(instruction);
            if self.jumped {
                self.jumped = false;
            } else {
                self.ip += 1;
            }
        }
    }
}
