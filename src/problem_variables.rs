use crate::expressions::variable::Variable;
use crate::Float;

#[derive(Default)]
pub struct ProblemVariables {
    pub(crate) bounds: Vec<(Option<Float>, Option<Float>)>,
}

impl ProblemVariables {
    pub fn add_variable(&mut self, lower: Option<Float>, upper: Option<Float>) -> Variable {
        let var = Variable(self.bounds.len());
        self.bounds.push((lower, upper));
        var
    }

    pub fn add_vector(
        &mut self,
        n: usize,
        lower: Option<Float>,
        upper: Option<Float>,
    ) -> Vec<Variable> {
        (0..n)
            .map(|_| self.add_variable(lower, upper))
            .collect::<Vec<_>>()
    }
}
