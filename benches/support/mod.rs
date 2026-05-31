// benches/support.rs
use criterion::Criterion;
use std::{env, time::Duration};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum BenchProfile {
    Quick,
    Report,
}

impl BenchProfile {
    #[inline]
    pub(crate) fn current() -> Self {
        match env::var("BENCH_PROFILE").as_deref() {
            Ok("report") => Self::Report,
            Ok("quick") | Err(_) => Self::Quick,
            Ok(other) => panic!("invalid BENCH_PROFILE={other:?}; expected `quick` or `report`"),
        }
    }

    #[inline]
    pub(crate) fn baseline(self) -> &'static str {
        match self {
            Self::Quick => "quick",
            Self::Report => "report",
        }
    }

    #[inline]
    pub(crate) fn sizes(self) -> &'static [usize] {
        match self {
            Self::Quick => &[64, 256],
            Self::Report => &[64, 256, 1024],
        }
    }

    #[inline]
    pub(crate) fn criterion(self) -> Criterion {
        match self {
            Self::Quick => Criterion::default()
                .sample_size(20)
                .warm_up_time(Duration::from_millis(100))
                .measurement_time(Duration::from_millis(300))
                .nresamples(10_000)
                .without_plots()
                .save_baseline(self.baseline().into()),
            Self::Report => Criterion::default()
                .sample_size(60)
                .warm_up_time(Duration::from_secs(1))
                .measurement_time(Duration::from_secs(2))
                .nresamples(20_000)
                .save_baseline(self.baseline().into()),
        }
    }
}

#[inline]
pub(crate) fn config() -> Criterion {
    BenchProfile::current().criterion()
}

#[inline]
pub(crate) fn profile() -> BenchProfile {
    BenchProfile::current()
}
