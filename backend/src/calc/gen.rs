use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;

// RANDOM MODEL GENERATOR
pub fn gen_random(risk: i64, outcomes: i64) -> Vec<i64> {

    fn throw_normal() -> i64 {
        thread_rng().gen_range(0..101)
    }
    
    fn subtract(minuend: i64, subtraend: i64) -> i64 {
        if subtraend > minuend { return 0 } else { return minuend - subtraend }
    }
    
    fn map_subtract(boxa: Vec<i64>) -> Vec<i64> {
        let mut boxb = Vec::new();
        for (pos, num) in boxa.iter().enumerate() {
            if pos == 0 {
                boxb.push(boxa[pos]);
            } else {
                let diff = num-boxa[pos-1];
                boxb.push(diff);
            }
        }
        return boxb
    }

    let initial: Vec<i64> = (0..outcomes-1).map(|_| throw_normal()).collect();
    let mut boxa: Vec<i64> = initial.into_iter().map(|x| subtract(x, risk*5)).collect();
    boxa.push(100);
    boxa.sort();
    let mut boxb = map_subtract(boxa);
    boxb.shuffle(&mut thread_rng());

    return boxb
}
