type SingleObjFunction = fn(Vec<f64>) -> f64;

pub struct BoundedFunction {
    function: SingleObjFunction,
    pub min_bound: f64,
    pub max_bound: f64,
    pub n_dim: i8,
}
impl BoundedFunction {
    fn eval(self, point: Vec<f64>) -> f64 {
        return (self.function)(point);
    }
}
