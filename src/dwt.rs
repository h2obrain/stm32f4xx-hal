//! Debug and trace and stuff

use crate::rcc::Clocks;
use crate::time::Hertz;
use cortex_m::peripheral::{DCB, DWT};
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

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
    pub fn release(self) -> (DWT, DCB) {
        (self.dwt, self.dcb)
    }
    pub fn delay(&self) -> Delay {
        Delay {
            clock: self.clocks.hclk(),
        }
    }
    pub fn stopwatch<'i>(&self, times: &'i mut [u32]) -> StopWatch<'i> {
        StopWatch::new(times, self.clocks.hclk())
    }
    pub fn measure<F: FnOnce()>(&self, f: F) -> ClockDuration {
        let mut times: [u32; 2] = [0; 2];
        let mut sw = self.stopwatch(&mut times);
        f();
        sw.lap().lap_time(1).unwrap()
    }
}

#[derive(Clone, Copy)]
pub struct Delay {
    clock: Hertz,
}
impl Delay {
    pub fn delay(duration: ClockDuration) {
        let ticks = duration.ticks as u64;
        Delay::delay_ticks(DWT::get_cycle_count(), ticks);
    }
    fn delay_ticks(mut start: u32, ticks: u64) {
        if ticks < (u32::MAX / 2) as u64 {
            let ticks = ticks as u32;
            while (DWT::get_cycle_count().wrapping_sub(start)) < ticks {}
        } else if ticks <= u32::MAX as u64 {
            let mut ticks = ticks as u32;
            ticks -= u32::MAX / 2;
            while (DWT::get_cycle_count().wrapping_sub(start)) < u32::MAX / 2 {}
            start -= u32::MAX / 2;
            while (DWT::get_cycle_count().wrapping_sub(start)) < ticks {}
        } else {
            let mut rest = (ticks >> 32) as u32;
            let ticks = (ticks & u32::MAX as u64) as u32;
            loop {
                while (DWT::get_cycle_count().wrapping_sub(start)) < ticks {}
                if rest == 0 {
                    break;
                }
                rest -= 1;
                while (DWT::get_cycle_count().wrapping_sub(start)) > ticks {}
            }
        }
    }
}

// Implement DelayUs/DelayMs for various integer types
impl DelayUs<u64> for Delay {
    fn delay_us(&mut self, us: u64) {
        let start = DWT::get_cycle_count();
        let ticks = (us * self.clock.0 as u64) / 1_000_000;
        Delay::delay_ticks(start, ticks);
    }
}
impl DelayMs<u64> for Delay {
    fn delay_ms(&mut self, ms: u64) {
        let start = DWT::get_cycle_count();
        let ticks = (ms * self.clock.0 as u64) / 1_000;
        Delay::delay_ticks(start, ticks);
    }
}
macro_rules! impl_DelayIntT {
    (for $($t:ty),+) => {$(
        impl DelayUs<$t> for Delay {
            fn delay_us(&mut self, us: $t) {
                self.delay_us(us as u64);
            }
        }
        impl DelayMs<$t> for Delay {
            fn delay_ms(&mut self, ms: $t) {
                self.delay_ms(ms as u64);
            }
        }
    )*}
}
impl_DelayIntT!(for usize,  u32, u16, u8);

/// Very simple stopwatch
pub struct StopWatch<'l> {
    times: &'l mut [u32],
    timei: usize,
    clock: Hertz,
}
impl<'l> StopWatch<'l> {
    fn new(times: &'l mut [u32], clock: Hertz) -> Self {
        assert!(times.len() >= 2);
        let mut sw = StopWatch {
            times,
            timei: 0,
            clock,
        };
        sw.reset();
        sw
    }
    pub fn lap_count(&self) -> usize {
        self.timei
    }
    pub fn reset(&mut self) {
        self.timei = 0;
        self.times[0] = DWT::get_cycle_count();
    }
    pub fn lap(&mut self) -> &Self {
        let c = DWT::get_cycle_count();
        if self.timei < self.times.len() {
            self.timei += 1;
        }
        self.times[self.timei] = c;
        self
    }
    pub fn lap_time(&self, n: usize) -> Option<ClockDuration> {
        if (n < 1) || (self.timei < n) {
            None
        } else {
            Some(ClockDuration {
                ticks: self.times[n].wrapping_sub(self.times[n - 1]),
                clock: self.clock,
            })
        }
    }
}

/// Clock difference with capability to calculate SI units (s)
#[derive(Clone, Copy)]
pub struct ClockDuration {
    ticks: u32,
    clock: Hertz,
}
impl ClockDuration {
    pub fn as_ticks(self) -> u32 {
        self.ticks
    }
    pub fn as_millis(self) -> u64 {
        self.ticks as u64 * 1_000 / self.clock.0 as u64
    }
    pub fn as_micros(self) -> u64 {
        self.ticks as u64 * 1_000_000 / self.clock.0 as u64
    }
    pub fn as_nanos(self) -> u64 {
        self.ticks as u64 * 1_000_000_000 / self.clock.0 as u64
    }
    pub fn as_secs_f32(self) -> f32 {
        self.ticks as f32 / self.clock.0 as f32
    }
    pub fn as_secs_f64(self) -> f64 {
        self.ticks as f64 / self.clock.0 as f64
    }
}
