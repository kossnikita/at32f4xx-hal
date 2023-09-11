use crate::pac::CRM;

pub struct MainPll {
    pub use_pll: bool,
    pub pllsclk: Option<u32>,
    pub pll48clk: Option<u32>,
}

impl MainPll {
    pub fn fast_setup(
        pllsrcclk: u32,
        use_hext: bool,
        hext_div2: Option<bool>,
        pllsclk: Option<u32>,
        pll48clk: bool,
    ) -> MainPll {
        let sclk = pllsclk.unwrap_or(pllsrcclk);
        if pllsclk.is_none() && !pll48clk {
            return MainPll {
                use_pll: false,
                pll48clk: None,
                pllsclk: None,
            };
        }

        // Sysclk output divisor must be one of 1, 2, 4, 8, 16 or 32
        let sclk_div = core::cmp::min(32, (432_000_000 / sclk) & !1);

        unsafe { &*CRM::ptr() }.cfg.modify(|_, w| unsafe {
            w.pllrcs().bit(use_hext);
            w.pllhextdiv().bit(hext_div2.unwrap_or(false))
        });

        let (pll_ms, pll_ms_bits) = (1, 1);
        let (pll_ns, pll_ns_bits) = (192, 192);
        let (pll_fr, pll_fr_bits) = (8, 3);

        unsafe { &*CRM::ptr() }.pll.write(|w| unsafe {
            w.pllcfgen().set_bit();
            w.pll_ms().bits(pll_ms_bits);
            w.pll_ns().bits(pll_ns_bits);
            w.pll_fr().bits(pll_fr_bits)
        });

        let real_pllsclk = pllsrcclk / pll_ms * pll_ns / pll_fr;

        MainPll {
            use_pll: true,
            pllsclk: Some(real_pllsclk),
            pll48clk: if pll48clk { Some(real_pllsclk) } else { None },
        }
    }
}
