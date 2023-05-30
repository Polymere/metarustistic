use metarustistic::functions::ObjectiveFunction;
use metarustistic::optimizers::Optimizer;
use metarustistic::random::get_random_vec;


pub struct OptimizationProblem {
    function: ObjectiveFunction,
    optimizer: Optimizer,
}

impl OptimizationProblem {
    fn solve(self, max_step: i16) {
        for _i in 0..max_step {
            self.optimizer.step();
            let score: f64 = self.function.eval(&self.optimizer.positions);
        }
    }
    fn init_optimizer(self) {
        match self.function {
            ObjectiveFunction::BoundedFunction => {
                self.optimizer.positions = get_random_vec(
                    self.function.n_dim,
                    self.function.min_bound,
                    self.function.max_bound,
                );
            }
            other => panic!("Unknown Objective function kind for init"),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn random_walk_bounded_init() {
        let problem: OptimizationProblem = OptimizationProblem {
            function: BoundedFunction {
                function: rastrigin,
                min_bound: -5.12,
                max_bound: 5.12,
                n_dim: 2,
            },
            optimizer: RandomWalk {},
        };

        problem.init_optimizer();
        asserteq!(optimizer.positions.len() == 2)
    }
}
