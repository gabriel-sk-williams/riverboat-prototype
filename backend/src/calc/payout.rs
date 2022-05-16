use std::collections::HashMap;

// PAYOUTS
// payouts with this method are calculated to each of person's certainty in the group
// using a "reverse waterfall" method

// receives hashmap of prediction models -> name: { outcome: certainty, ... }
// returns hashmap of payouts -> name: { outcome: payout, ... }
pub fn payouts(models: HashMap<String, HashMap<String, f64>>, 
               outcomes: Vec<String>,
               stake: f64) 
               -> HashMap<String, HashMap<String, f64>> {
    
    // outcome: { name: payout, ... }
    let mut outcome_map = HashMap::new();
    for oc in outcomes.iter() {
        let oc_vec = outcome_vec(&models, String::from(oc));
        let pomap = payout_map(oc_vec, stake); // one outcome each
        outcome_map.insert(String::from(oc), pomap);
    }

    // convert back to original form -> name: { outcome: payout, ... }
    let mut payout_map = HashMap::new();
    for (name, _model) in models.iter() {
        let mut personal_map = HashMap::new();
        for oc in outcomes.iter() { // build payout struct -> outcome: payment
            let payout: f64 = aggregate(String::from(oc), String::from(name), outcome_map.clone());
            personal_map.insert(String::from(oc), payout); 
        }
        payout_map.insert(String::from(name), personal_map);
    }

    return payout_map
}

// generate a vec of tuples -> outcome: [ (name, certainty)... ]
// representing each person's prediction of each possible outcome
pub fn outcome_vec(models: &HashMap<String, HashMap<String, f64>>,
                   outcome: String) 
                   -> Vec<(String, f64)> {

    let mut outcome_vec = Vec::new();

    for (name, model) in models {
        let value = model.get(&outcome).expect("goods"); // returns Option
        outcome_vec.push((String::from(name), value.clone()));
    }

    // sorts from low certainty -> high
    outcome_vec.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

    return outcome_vec 
}

// receive the vec of tuples -> outcome: [ (name, certainty), ... ]
// return hashmap of payouts -> outcome: { name: payout, ... }
pub fn payout_map(mut oc_vec: Vec<(String, f64)>, 
                      stake: f64) 
                      -> HashMap<String, Vec<f64>> {
    
    // create empty vec for each name: [ payout0, payout1, ... ]
    fn blank_map(oc_vec: Vec<(String, f64)>) -> HashMap<String, Vec<f64>> {
        let mut pomap: HashMap<String, Vec<f64>> = HashMap::new();
        for (name, _certainty) in oc_vec.iter() {
            pomap.insert(name.clone(), Vec::new());
        }
        return pomap;
    }

    // calculate raw loss of bad prediction
    fn raw_loss(certainty: f64, stake: f64) -> f64 {
        let cfl = certainty as f64;
        let fraction: f64 = cfl/100_f64;
        let dim: f64 = (fraction * stake * 100_f64).round() / 100_f64;
        return dim - stake;
    }
    
    // sum the certainties of remaining field
    fn sum_certs(rem: Vec<(String, f64)>) -> f64 {
        rem.iter().map(|n| n.1 as f64).sum()
    }

    // moderate according to most correct person in field
    fn mod_loss(mut rem: Vec<(String, f64)>) -> f64 {
        let best: f64 = rem.pop().unwrap().1;
        return best / 100_f64
    }

    // get the number of consecutive identical certainties in sorted group
    fn get_consecutive(data: Vec<(String, f64)>) -> Option<usize> {

        let index = data
            .iter()
            .zip(data.iter().skip(1))
            .position(|(current, next)| current.1 != next.1);

        return index
    }

    let mut pomap: HashMap<String, Vec<f64>> = blank_map(oc_vec.clone());
    
    // drain least certain predictors as cohort, deliver payments upward
    while oc_vec.len() > 0 {
        let consecutive = get_consecutive(oc_vec.clone());

        let trust: Vec<(String, f64)> = match consecutive {
            Some(consecutive) => oc_vec.drain(0..consecutive+1).collect(),
            None => break
        };

        for (current_name, current_cert) in trust {
            let raw_loss: f64 = raw_loss(current_cert.clone(), stake);
            let mod_loss: f64 = mod_loss(oc_vec.clone());
            let paid_loss: f64 = raw_loss * mod_loss;
            pomap.get_mut(&current_name).expect("").push(paid_loss);

            let payout: f64 = paid_loss.abs();
            let mount: f64 = sum_certs(oc_vec.clone());

            for (next_name, next_cert) in oc_vec.iter() {
                let mass: f64 = next_cert.clone() as f64 / mount;
                let portion: f64 = payout * mass;
                pomap.get_mut(next_name).expect("").push(portion);
            } 
        }
    }

    return pomap
}

// access vec name: [payouts, ...] and sum all values to get final payout
fn aggregate(outcome: String, 
             name: String, 
             oculus: HashMap<String, HashMap<String, Vec<f64>>>) 
             -> f64 {
   
   let ocmap = oculus.get(&outcome).expect("");
   let payvec = ocmap.get(&name).expect("");

   // round all values, sum, and round again for consistency
   let rounded: Vec<f64> = payvec.iter().map(|f| (f*100.0_f64).round() / 100.0_f64 ).collect();
   let collapsed: f64 = rounded.iter().sum();
   let final_round: f64 = (collapsed*100.0_f64).round() / 100.0_f64;

   return final_round
}
