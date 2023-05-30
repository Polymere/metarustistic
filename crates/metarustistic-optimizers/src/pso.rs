use rand::Rng;

pub fn get_random_vec(n:i32,low:f32,high:f32)->Vec<f32>{
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let vals: Vec<f32> = (0..n).map(|_| rng.gen_range(low..=high)).collect();
    return vals;
}
pub struct PSO{
    pub positions: Vec<Vec<f32>>,
    pub scores: Vec<f32>,
    pub n_particles: i32,
    pub n_dim: i32,
    pub step_number: i32,
    pub best_particle: Option<Vec<f32>>,
    pub best_score: Option<f32>,
}
//https://stackoverflow.com/questions/19650265/is-there-a-faster-shorter-way-to-initialize-variables-in-a-rust-struct
impl Default for PSO{
    fn default() -> PSO{
        PSO{
            positions : vec![],
            scores: vec![],
            n_particles : 2,
            n_dim: 2,
            step_number:0,
            best_particle: None,
            best_score: None
        }
    }
}
// https://en.wikipedia.org/wiki/Particle_swarm_optimization
impl PSO{

    fn init_particles(&mut self)
    {
        for i in 0..self.n_particles
        {
            let particle = get_random_vec(self.n_dim,-5.12,5.12);
            self.positions.push(particle);
        }

    }
    fn compute_scores(&mut self,eval_function:fn(&Vec<f32>)->f32){
        for particle in &self.positions
        {
            let score = eval_function(particle);
            self.scores.push(score);
            println!("Particle {:?}", particle);
            // println!("Score {:?}", score);
            match self.best_score
            {
                None => self.best_score = Some(score),
                Some(x) => if score > x
                {
                    self.best_score = Some(score);
                    // self.best_particle = Some(*particle);
                    println!("Best score {:?}", score);
                }
            }
        }
    }
    // https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html
    pub fn step(&mut self,eval_function:fn(&Vec<f32>)->f32)
    {
        if self.step_number == 0{
            self.init_particles();
            self.step_number = 1;
        }
        else{
            // self.positions=self.positions.clone().into_iter().zip(get_random_vec(self.n_dim,-1.0,1.0)).map(|(a, b)| {
            //     if (a+b).abs()<5.12{
            //         a+b
            //     }
            //     else{
            //         a - b
            //     }
            // } ).collect();
            self.step_number+=1;

        }
        self.compute_scores(eval_function)

    }

}



#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn random_vec_size(){
        let opti: PSO = PSO{
            positions: vec![0.0, 2.001, 0.0],
            step_number : 1,
        };
        assert!(opti.positions.len() == opti.random_vec().len());
    }
    fn step_changes_positions(){
        let mut opti: PSO = PSO{
            positions: vec![0.0, 2.001, 0.0],
            step_number : 1,
        };
        opti.step();
        assert!(opti.step_number==2)
    }
}