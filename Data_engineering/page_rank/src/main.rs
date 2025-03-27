use textwrap::fill;

struct PageRank {
    damping: f64,
    iterations: usize,
}

impl PageRank {
    fn new(damping: f64, iterations: usize) -> Self {
        Self { damping, iterations }
    }

    fn rank(&self, graph: &Vec<Vec<usize>>) -> Vec<f64> {

        let n = graph.len();

        let mut ranks = vec![1.0 / n as f64; n];

        for _ in 0..self.iterations {
            let mut new_ranks = vec![0.0; n];
        }
    }
}