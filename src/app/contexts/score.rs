use std::rc::Rc;

use yew::prelude::*;

#[allow(dead_code)]
pub enum ScoreAction {
    Add(u32),
    Reset,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Score {
    pub current: u32,
    pub best: u32,
}

pub type ScoreContext = UseReducerHandle<Score>;

impl Reducible for Score {
    type Action = ScoreAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let (next_curr, next_best) = match action {
            ScoreAction::Add(n) => {
                let next_curr = self.current + n;
                let next_best = if next_curr > self.best {
                    next_curr
                } else {
                    self.best
                };
                (next_curr, next_best)
            }
            ScoreAction::Reset => (0, self.best),
        };

        Self {
            current: next_curr,
            best: next_best,
        }
        .into()
    }
}
