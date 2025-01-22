#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let v = self.get_top(1);

        if v.len() > 0 {
            Some(v[0])
        } else {
            None
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.get_top(3)
    }

    fn get_top(&self, n: usize) -> Vec<u32> {
        let mut v = self.scores.to_vec();
        v.sort_unstable_by(|x, y| y.cmp(x));
        v.truncate(n);
        v
    }
}
