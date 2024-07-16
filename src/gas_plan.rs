use crate::{Error, Result};

pub struct GasPlan {
    /// Cap total usage
    all: Option<i64>,

    /// Timeout
    time: Option<i64>,

    /// Computation time
    comp: Option<i64>,

    /// Working memory
    memory: Option<i64>,

    /// Data in database
    index: Option<i64>,

    /// Data in storage
    store: Option<i64>,

    /// Upload traffic
    up: Option<i64>,

    /// Download traffic
    down: Option<i64>,
}

impl GasPlan {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        all: Option<i64>,
        time: Option<i64>,
        comp: Option<i64>,
        memory: Option<i64>,
        index: Option<i64>,
        store: Option<i64>,
        up: Option<i64>,
        down: Option<i64>,
    ) -> Result<Self> {
        let plan = GasPlan {
            all,
            time,
            comp,
            memory,
            index,
            store,
            up,
            down,
        };
        match all.is_none() && !plan.is_rigourous() {
            true => Err(Error::InfGasPlan),
            false => Ok(plan),
        }
    }

    fn is_rigourous(&self) -> bool {
        self.time.is_some()
            && self.comp.is_some()
            && self.memory.is_some()
            && self.index.is_some()
            && self.store.is_some()
            && self.up.is_some()
            && self.down.is_some()
    }

    pub fn get_cap(&self) -> i64 {
        match self.is_rigourous() {
            true => {
                let sum = self.time.unwrap()
                    + self.comp.unwrap()
                    + self.memory.unwrap()
                    + self.index.unwrap()
                    + self.store.unwrap()
                    + self.up.unwrap()
                    + self.down.unwrap();
                match self.all {
                    Some(all) => std::cmp::min(all, sum),
                    None => sum,
                }
            }
            false => match self.all {
                Some(all) => all,
                None => unreachable!(),
            },
        }
    }
}
