use plotpy::{Curve, Plot};
use rand::{thread_rng, Rng};

use simple_qp::problem::Problem;
use simple_qp::solver::clarabel_solver::ClarabelSolver;
use simple_qp::solver::osqp_solver::OSQPSolver;
use simple_qp::{constraint, Float};

fn smooth_path(points: Vec<[Float; 2]>, max_deviation: Float) -> Vec<[Float; 2]> {
    let mut prob = Problem::default();
    let xs = prob.add_vec(points.len(), None, None);
    let ys = prob.add_vec(points.len(), None, None);

    for coords in [&xs, &ys] {
        for x in coords.windows(3) {
            prob.objective = prob.objective + (x[2] - 2.0 * x[1] + x[0]).square();
        }
    }

    for (i, p) in points.iter().enumerate() {
        prob.constraints
            .push(constraint!(-max_deviation <= xs[i] - p[0] <= max_deviation));
        prob.constraints
            .push(constraint!(-max_deviation <= ys[i] - p[1] <= max_deviation));
    }

    let n = points.len();
    prob.constraints.push(constraint!(xs[0] == points[0][0]));
    prob.constraints.push(constraint!(ys[0] == points[0][1]));
    prob.constraints
        .push(constraint!(xs[n - 1] == points[n - 1][0]));
    prob.constraints
        .push(constraint!(ys[n - 1] == points[n - 1][1]));

    let mut solver = OSQPSolver::default();
    solver.settings = solver.settings.verbose(false);

    let solver = ClarabelSolver::default();

    let solution = prob.solve(&solver).unwrap();
    solution
        .eval_vec(xs)
        .into_iter()
        .zip(solution.eval_vec(ys))
        .map(|(x, y)| [x, y])
        .collect()
}

fn random_path(n: usize) -> Vec<[f64; 2]> {
    let mut ret = vec![];

    let mut rng = thread_rng();
    for i in 0..n {
        let x = i as f64 + rng.gen_range(-0.5..0.5);
        let y = (i as f64 * 0.05).sin() + rng.gen_range(-0.5..0.5);
        ret.push([x, y]);
    }

    ret
}

fn path_xy(path: &[[Float; 2]]) -> (Vec<Float>, Vec<Float>) {
    (
        path.iter().map(|it| it[0]).collect::<Vec<_>>(),
        path.iter().map(|it| it[1]).collect::<Vec<_>>(),
    )
}

fn main() {
    //let original = vec![[0.0, 0.0], [1.0, 1.0], [2.0, 2.0], [3.0, 1.0], [4.0, 0.0]];
    let original = random_path(300);
    let smooth = smooth_path(original.clone(), 0.5);

    let mut plot = Plot::new();
    let mut c = Curve::new();
    let (x, y) = path_xy(&original);
    c.set_label("original");
    c.draw(&x, &y);
    let (x, y) = path_xy(&smooth);
    c.set_label("smooth");
    c.draw(&x, &y);
    plot.add(&c);
    plot.legend();
    plot.save_and_show("/tmp/tmp.png").unwrap()
}
