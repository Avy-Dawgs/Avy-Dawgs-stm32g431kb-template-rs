#![no_main] 
#![no_std]

use stm32g4xx_hal::{
    self as hal, cortex_m, gpio::{self, *}, prelude::OutputPin, rcc::{self, RccExt}, stm32,
};

use cortex_m_rt::entry;
use panic_halt as _;
use defmt_rtt as _;

use defmt::info; 

#[entry]
fn main() -> ! 
{
    info!("Program Starting");

    let dp = stm32::Peripherals::take().unwrap();       // device peripherals
    let cp = cortex_m::Peripherals::take().unwrap();    // core peripherals

    // configure clocks
    let mut rcc = config_clks(dp.RCC);

    // configure led
    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut led = gpiob.pb8.into_push_pull_output();

    info!("sys clock freq: {} Hz", rcc.clocks.sys_clk.0);
    info!("core clock freq: {} Hz", rcc.clocks.core_clk.0);

    loop {

    }
}

/// Configures clocks. 
#[inline]
fn config_clks(rcc: stm32::RCC) -> rcc::Rcc
{
    // pll config 
    let mut pll_cfg = rcc::PllConfig::default();
    pll_cfg.n = rcc::PllNMul::MUL_85; 
    pll_cfg.m = rcc::PllMDiv::DIV_4;
    pll_cfg.r = Some(rcc::PllRDiv::DIV_2);
    pll_cfg.q = Some(rcc::PllQDiv::DIV_2); 
    pll_cfg.p = Some(rcc::PllPDiv::DIV_2);

    // clock config object
    let clk_cfg = rcc::Config::new(rcc::SysClockSrc::PLL);  
    let clk_cfg = clk_cfg.pll_cfg(pll_cfg);             // apply pll config

    // apply config
    rcc.constrain().freeze(clk_cfg)
}
