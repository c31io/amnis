use crate::{Error, Result};

pub struct GasPlan {
    /// Cap total usage
    all: Option<i64>,

    gas: Gas,
}

pub struct Gas {
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
            gas: Gas {
                time,
                comp,
                memory,
                index,
                store,
                up,
                down,
            },
        };
        match all.is_none() && !plan.gas.is_rigourous() {
            true => Err(Error::InfGasPlan),
            false => Ok(plan),
        }
    }

    pub fn max() -> Self {
        GasPlan {
            all: Some(std::i64::MAX),
            gas: Gas::none(),
        }
    }

    pub fn get_cap(&self) -> Result<i64> {
        match self.gas.is_rigourous() {
            true => {
                let Some(sum) = self.gas.sum() else {
                    return Err(Error::GasPanOverflow);
                };
                Ok(match self.all {
                    Some(all) => std::cmp::min(all, sum),
                    None => sum,
                })
            }
            false => match self.all {
                Some(all) => Ok(all),
                None => unreachable!(),
            },
        }
    }
}

impl Gas {
    pub fn none() -> Self {
        Gas {
            time: None,
            comp: None,
            memory: None,
            index: None,
            store: None,
            up: None,
            down: None,
        }
    }

    pub fn zero() -> Self {
        Gas {
            time: Some(0),
            comp: Some(0),
            memory: Some(0),
            index: Some(0),
            store: Some(0),
            up: Some(0),
            down: Some(0),
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

    /// Panic if `Self.is_rigourous()` is `false`.
    fn sum(&self) -> Option<i64> {
        self.time
            .unwrap()
            .checked_add(self.comp.unwrap())?
            .checked_add(self.memory.unwrap())?
            .checked_add(self.index.unwrap())?
            .checked_add(self.store.unwrap())?
            .checked_add(self.up.unwrap())?
            .checked_add(self.down.unwrap())
    }
}
