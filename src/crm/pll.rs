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

        unsafe { &*CRM::ptr() }.cfg.modify(|_, w| unsafe {
            w.pllrcs().bit(use_hext);
            w.pllhextdiv().bit(hext_div2.unwrap_or(false))
        });

        unsafe { &*CRM::ptr() }.pll.write(|w| unsafe {
            w.pllcfgen().set_bit();
            w.pll_ms().bits(0x1);
            w.pll_ns().bits(0x1F);
            w.pll_fr().bits(0x0)
        });

        MainPll {
            use_pll: true,
            pllsclk: pllsclk,
            pll48clk: if pll48clk { Some(48_000_000) } else { None },
        }
    }
}
