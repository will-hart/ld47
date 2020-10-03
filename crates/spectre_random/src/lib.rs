use rand::Rng;
/// A quasi-random number generator (i.e. DOTA 2 style proc)
pub struct QRNG {
    pub initial: f32,
    pub increment: f32,
    pub maximum: f32,
    unsuccessful: isize,
}

impl QRNG {
    pub fn new() -> Self {
        QRNG {
            initial: 0.1,
            increment: 0.05,
            maximum: 0.4,
            unsuccessful: 0,
        }
    }

    /// Returns true if the QRNG proc'd and resets
    pub fn test(&mut self) -> bool {
        let p =
            (self.initial + (self.unsuccessful as f32) * self.increment).max(self.maximum) as f64;

        let success = rand::thread_rng().gen_bool(p);

        if success {
            self.unsuccessful = 0;
        } else {
            self.unsuccessful += 1;
        }

        success
    }
}

/// A basically empty wrapper around an RNG
pub struct RNG;

impl RNG {
    pub fn next() -> f32 {
        rand::thread_rng().gen()
    }

    pub fn test(p: f32) -> bool {
        rand::thread_rng().gen_bool(p as f64)
    }

    /// Inclusive of the last element
    pub fn i32_between(min: i32, max: i32) -> i32 {
        rand::thread_rng().gen_range(min, max + 1)
    }

    /// Exclusive of the last element
    pub fn usize_between(min: usize, max: usize) -> usize {
        rand::thread_rng().gen_range(min, max)
    }
}
