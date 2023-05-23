use rand::Rng;

pub fn get_random_vec(n:i32,low:f64,high:f64)->Vec<f64>{
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let vals: Vec<f64> = (0..n).map(|_| rng.gen_range(low..=high)).collect();
    return vals;
}
pub struct RandomWalk{
    pub positions: Vec<f64>,
    pub step_number: i64,
}

impl RandomWalk{

    pub fn step(&mut self)
    {
        
        self.positions=self.positions.clone().into_iter().zip(self.random_vec()).map(|(a, b)| {
            if (a+b).abs()<5.12{
                a+b
            }
            else{
                a - b
            }
        } ).collect();
        self.step_number+=1;
    }

    fn random_vec(&self)->Vec<f64>
    {
        return get_random_vec(self.positions.len() as i32,-1.0, 1.0)
    }


}



#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn random_vec_size(){
        let opti: RandomWalk = RandomWalk{
            positions: vec![0.0, 2.001, 0.0],
            step_number : 1,
        };
        assert!(opti.positions.len() == opti.random_vec().len());
    }
    fn step_changes_positions(){
        let mut opti: RandomWalk = RandomWalk{
            positions: vec![0.0, 2.001, 0.0],
            step_number : 1,
        };
        opti.step();
        assert!(opti.step_number==2)
    }
}