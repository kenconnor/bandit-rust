use rand::Rng;

struct Bandit {
    rates: Vec<f64>,
}

impl Bandit {
    fn new(arms: Option<u32>) -> Bandit {
        let mut rng = rand::thread_rng();
        Bandit { 
            rates: (0..arms.unwrap_or(10)).map(|_| rng.gen::<f64>()).collect(),
        }
    }
    fn play(&self, arm: usize) -> f64 {
        let rate = self.rates[arm];
        let mut rng = rand::thread_rng();
        if rate > rng.gen::<f64>() {
            1f64
        } else {
            0f64
        }
    }
}

fn main() {
    //let bandit = Bandit::new(Some(10));
    let bandit = Bandit::new(None);
    for _n in 1..4 {
        println!("{}", bandit.play(0));
    }

    let bandit = Bandit::new(None);
    let mut q = 0f64;
    for n in 1..11 {
        let reward = bandit.play(0);
        q += (reward - q) / n as f64;
        println!("{}", q);
    }
}
