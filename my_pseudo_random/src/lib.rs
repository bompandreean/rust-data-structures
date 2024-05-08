use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref RANDOM_GENERATOR: Mutex<PseudoRandom> = Mutex::new(PseudoRandom::new(34052));
}

pub fn rand(max: usize) -> usize {
    RANDOM_GENERATOR.lock().unwrap().next_v(max)
}

struct PseudoRandom {
    curr: usize,
    mul: usize,
    inc: usize,
    modulo: usize,
}

impl PseudoRandom {
    pub fn new(curr: usize) -> Self {
        PseudoRandom {
            curr,
            mul: 1103515245,
            inc: 12363345,
            modulo: 65536538,
        }
    }

    pub fn next_v(&mut self, max: usize) -> usize {
        self.curr = (self.curr * self.mul + self.inc) % self.modulo;
        self.curr % max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        for i in 0..100 {
            println!("random #{} = {}", i, rand(100));
        }
    }
}
