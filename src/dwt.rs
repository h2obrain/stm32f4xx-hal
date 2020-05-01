//! Debug and trace and stuff

use crate::rcc::Clocks;
use crate::time::Hertz;
use cortex_m::peripheral::{DCB, DWT};
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

/// Delay in seconds
pub trait DelayS<FXX> {
    /// Delay code by s seconds
    fn delay(&mut self, s: FXX);
}
/// Convert into seconds
pub trait IntoSeconds<Fxx> {
    fn as_secs(&self) -> Fxx;
}

pub trait DwtExt {
    fn constrain(self, dcb: DCB, clocks: Clocks) -> Dwt;
}
impl DwtExt for DWT {
    /// Setup must be called at least once before using other dwt functionality
    fn constrain(mut self, mut dcb: DCB, clocks: Clocks) -> Dwt {
        cortex_m::interrupt::free(|_| {
            // Enable fmc clock
            dcb.enable_trace();

            self.enable_cycle_counter();
        });

        Dwt {
            dwt: self,
            dcb,
            clocks,
        }
    }
}

pub struct Dwt {
    dwt: DWT,
    dcb: DCB,
    clocks: Clocks,
}
impl Dwt {
    pub fn delay(&self) -> Delay {
        Delay {
            dwt_clock: self.clocks.hclk(),
        }
    }
    pub fn stopwatch<'i>(&self, times: &'i mut [u32]) -> StopWatch<'i> {
        StopWatch::new(times, self.clocks.hclk())
    }
    pub fn measure<F: FnOnce()>(&self, f: F) -> ClockDiff {
        let mut times: [u32; 2] = [0; 2];
        let mut sw = self.stopwatch(&mut times);
        f();
        sw.lap().last()
    }
    pub fn release(self) -> (DWT, DCB) {
        (self.dwt, self.dcb)
    }
}

#[derive(Clone, Copy)]
pub struct Delay {
    dwt_clock: Hertz,
}
impl Delay {
    pub fn delay_cycles(cycles: u64) {
        Delay::delay_cycles_(DWT::get_cycle_count(), cycles);
    }
    fn delay_us_(self, us: u64) {
        let start = DWT::get_cycle_count();
        let cycles = (us * self.dwt_clock.0 as u64) / 1_000_000;
        Delay::delay_cycles_(start, cycles);
    }
    fn delay_ms_(self, ms: u64) {
        let start = DWT::get_cycle_count();
        let cycles = (ms * self.dwt_clock.0 as u64) / 1_000;
        Delay::delay_cycles_(start, cycles);
    }
    fn delay_cycles_(mut start: u32, cycles: u64) {
        if cycles < (u32::MAX / 2) as u64 {
            let cycles = cycles as u32;
            while (DWT::get_cycle_count().wrapping_sub(start)) < cycles {}
        } else if cycles <= u32::MAX as u64 {
            let mut cycles = cycles as u32;
            cycles -= u32::MAX / 2;
            while (DWT::get_cycle_count().wrapping_sub(start)) < u32::MAX / 2 {}
            start -= u32::MAX / 2;
            while (DWT::get_cycle_count().wrapping_sub(start)) < cycles {}
        } else {
            let mut rest = (cycles >> 32) as u32;
            let cycles = (cycles & u32::MAX as u64) as u32;
            loop {
                while (DWT::get_cycle_count().wrapping_sub(start)) < cycles {}
                if rest == 0 {
                    break;
                }
                rest -= 1;
                while (DWT::get_cycle_count().wrapping_sub(start)) > cycles {}
            }
        }
    }
}

// Implement DelayUs/DelayMs for various integer types
macro_rules! impl_DelayIntT {
    (for $($t:ty),+) => {$(
        impl DelayUs<$t> for Delay {
            fn delay_us(&mut self, us: $t) {
                self.delay_us_(us as u64);
            }
        }
        impl DelayMs<$t> for Delay {
            fn delay_ms(&mut self, ms: $t) {
                self.delay_ms_(ms as u64);
            }
        }
    )*}
}
impl_DelayIntT!(for usize,u64,u32,u16,u8,i64,i32,i16,i8);

/// Implement DelayS for float types (Note, that the delay itself will need to calculate a lot)
macro_rules! impl_DelayFloatT {
    (for $($t:ty),+) => {$(
        impl DelayS<$t> for Delay {
            fn delay(&mut self, s:$t) {
                let start = DWT::get_cycle_count();
                let cycles = s * self.dwt_clock.0 as $t;
                Delay::delay_cycles_(start, cycles as u64);
            }
        }
    )*}
}
impl_DelayFloatT!(for f32,f64);

/// Very simple stopwatch
/// Notes:
/// - Max tdiff is < (1<<32) / HCLK
/// - Assumes DWT cycle counter is enabled
/// - Assumes Debug is set up
pub struct StopWatch<'l> {
    times: &'l mut [u32],
    timei: usize,
    dwt_clock: Hertz,
}
impl<'l> StopWatch<'l> {
    pub fn new(times: &'l mut [u32], dwt_clock: Hertz) -> Self {
        assert!(times.len() >= 2);
        let sw = StopWatch {
            times,
            timei: 0,
            dwt_clock,
        };
        sw.times[0] = DWT::get_cycle_count();
        sw
    }
    pub fn lap(&mut self) -> &Self {
        let c = DWT::get_cycle_count();
        if self.timei < self.times.len() {
            self.timei += 1;
        }
        self.times[self.timei] = c;
        self
    }
    pub fn lap_time(&self, n: usize) -> ClockDiff {
        if (n < 1) || (self.timei < n) {
            ClockDiff {
                diff: 0,
                dwt_clock: self.dwt_clock,
            }
        } else {
            ClockDiff {
                diff: self.times[n].wrapping_sub(self.times[n - 1]),
                dwt_clock: self.dwt_clock,
            }
        }
    }
    pub fn last(&self) -> ClockDiff {
        self.lap_time(self.timei)
    }
}

/// Clock difference with capability to calculate SI units (s)
pub struct ClockDiff {
    diff: u32,
    dwt_clock: Hertz,
}
impl ClockDiff {
    pub fn diff_clock(&self) -> u32 {
        self.diff
    }
    pub fn as_nanos(&self) -> u64 {
        self.diff as u64 * 1_000_000_000 / self.dwt_clock.0 as u64
    }
}
macro_rules! impl_ClockDiff {
    (for $($t:ty),+) => {$(
        impl IntoSeconds<$t> for ClockDiff {
            fn as_secs(&self) -> $t {
                self.diff as $t / self.dwt_clock.0 as $t
            }
        }
    )*}
}
impl_ClockDiff!(for f32,f64);
