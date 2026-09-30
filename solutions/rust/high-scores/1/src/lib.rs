#[derive(Debug)]
pub struct HighScores{
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self{
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
&self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut new_list = self.scores.clone();
        new_list.sort_unstable();
        new_list.reverse();
        new_list.truncate(3);
        new_list
    }
}
