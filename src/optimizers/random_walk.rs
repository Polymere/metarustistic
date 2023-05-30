
use crate::{
random::get_random_vec,
};
#[derive(Default)]
pub struct RandomWalk {
    pub positions: Option<Vec<f32>>,
    pub step_number: i64,
}

impl RandomWalk {
    pub fn step(&mut self) {

        if let Some(positions)=&self.positions
        {
            self.positions=Some(positions.clone().into_iter().zip(self.random_vec()).map(|(a, b)| {
                if (a+b).abs()<5.12{
                    a+b
                }
                else{
                    a - b
                }
            } ).collect());
            self.step_number += 1;
        }

    }

    fn random_vec(&self) -> Vec<f32> {
        if let Some(positions) = &self.positions
        {
            get_random_vec(positions.len() as i32, -1.0, 1.0)
        }
        else
        {
        panic!("Optimizer not initialized");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn random_vec_size() {
        let opti: RandomWalk = RandomWalk {
            positions: vec![0.0, 2.001, 0.0],
            step_number: 1,
        };
        assert!(opti.positions.len() == opti.random_vec().len());
    }
    fn step_changes_positions() {
        let mut opti: RandomWalk = RandomWalk {
            positions: vec![0.0, 2.001, 0.0],
            step_number: 1,
        };
        opti.step();
        assert!(opti.step_number == 2)
    }
}
