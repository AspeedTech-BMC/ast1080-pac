#[doc = "Register `PRIC_IO27C` reader"]
pub type R = crate::R<PricIo27cSpec>;
#[doc = "Register `PRIC_IO27C` writer"]
pub type W = crate::W<PricIo27cSpec>;
#[doc = "Field `Reserved15` reader - Reserved"]
pub type Reserved15R = crate::BitReader;
#[doc = "Field `Reserved15` writer - Reserved"]
pub type Reserved15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved14` reader - Reserved"]
pub type Reserved14R = crate::BitReader;
#[doc = "Field `Reserved14` writer - Reserved"]
pub type Reserved14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved13` reader - Reserved"]
pub type Reserved13R = crate::BitReader;
#[doc = "Field `Reserved13` writer - Reserved"]
pub type Reserved13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved12` reader - Reserved"]
pub type Reserved12R = crate::BitReader;
#[doc = "Field `Reserved12` writer - Reserved"]
pub type Reserved12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved11` reader - Reserved"]
pub type Reserved11R = crate::BitReader;
#[doc = "Field `Reserved11` writer - Reserved"]
pub type Reserved11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved10` reader - Reserved"]
pub type Reserved10R = crate::BitReader;
#[doc = "Field `Reserved10` writer - Reserved"]
pub type Reserved10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved9` reader - Reserved"]
pub type Reserved9R = crate::BitReader;
#[doc = "Field `Reserved9` writer - Reserved"]
pub type Reserved9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved8` reader - Reserved"]
pub type Reserved8R = crate::BitReader;
#[doc = "Field `Reserved8` writer - Reserved"]
pub type Reserved8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfWDT0` reader - Enable Write Group #0 of WDT 0"]
pub type EnblWrGroup0ofWdt0R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfWDT0` writer - Enable Write Group #0 of WDT 0"]
pub type EnblWrGroup0ofWdt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfWDT0` reader - Enable Write Group #1 of WDT 0"]
pub type EnblWrGroup1ofWdt0R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfWDT0` writer - Enable Write Group #1 of WDT 0"]
pub type EnblWrGroup1ofWdt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfWDT0` reader - Enable Write Group #2 of WDT 0"]
pub type EnblWrGroup2ofWdt0R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfWDT0` writer - Enable Write Group #2 of WDT 0"]
pub type EnblWrGroup2ofWdt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfWDT0` reader - Enable Write Group #3 of WDT 0"]
pub type EnblWrGroup3ofWdt0R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfWDT0` writer - Enable Write Group #3 of WDT 0"]
pub type EnblWrGroup3ofWdt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfWDT0` reader - Enable Write Group #4 of WDT 0"]
pub type EnblWrGroup4ofWdt0R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfWDT0` writer - Enable Write Group #4 of WDT 0"]
pub type EnblWrGroup4ofWdt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfWDT0` reader - Enable Write Group #5 of WDT 0"]
pub type EnblWrGroup5ofWdt0R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfWDT0` writer - Enable Write Group #5 of WDT 0"]
pub type EnblWrGroup5ofWdt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC127CPRIC1_27C\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric127cpric127c2116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric127cpric127c2116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric127cpric127c2116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC127CPRIC127C2116` reader - Enable Reset Tolerance of PRIC127CPRIC1_27C\\[21:16\\]"]
pub type EnblRstToleranceOfPric127cpric127c2116R =
    crate::BitReader<EnblRstToleranceOfPric127cpric127c2116>;
impl EnblRstToleranceOfPric127cpric127c2116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric127cpric127c2116 {
        match self.bits {
            false => EnblRstToleranceOfPric127cpric127c2116::ResetBySrst,
            true => EnblRstToleranceOfPric127cpric127c2116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric127cpric127c2116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric127cpric127c2116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC127CPRIC127C2116` writer - Enable Reset Tolerance of PRIC127CPRIC1_27C\\[21:16\\]"]
pub type EnblRstToleranceOfPric127cpric127c2116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric127cpric127c2116>;
impl<'a, REG> EnblRstToleranceOfPric127cpric127c2116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric127cpric127c2116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric127cpric127c2116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC127CPRIC127C2216` reader - Enable Write Protection of PRIC127CPRIC1_27C\\[22:16\\]"]
pub type EnblWrProtOfPric127cpric127c2216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC127CPRIC127C2216` writer - Enable Write Protection of PRIC127CPRIC1_27C\\[22:16\\]"]
pub type EnblWrProtOfPric127cpric127c2216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfWDT1` reader - Enable Write Group #0 of WDT 1"]
pub type EnblWrGroup0ofWdt1R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfWDT1` writer - Enable Write Group #0 of WDT 1"]
pub type EnblWrGroup0ofWdt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfWDT1` reader - Enable Write Group #1 of WDT 1"]
pub type EnblWrGroup1ofWdt1R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfWDT1` writer - Enable Write Group #1 of WDT 1"]
pub type EnblWrGroup1ofWdt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfWDT1` reader - Enable Write Group #2 of WDT 1"]
pub type EnblWrGroup2ofWdt1R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfWDT1` writer - Enable Write Group #2 of WDT 1"]
pub type EnblWrGroup2ofWdt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfWDT1` reader - Enable Write Group #3 of WDT 1"]
pub type EnblWrGroup3ofWdt1R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfWDT1` writer - Enable Write Group #3 of WDT 1"]
pub type EnblWrGroup3ofWdt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfWDT1` reader - Enable Write Group #4 of WDT 1"]
pub type EnblWrGroup4ofWdt1R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfWDT1` writer - Enable Write Group #4 of WDT 1"]
pub type EnblWrGroup4ofWdt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfWDT1` reader - Enable Write Group #5 of WDT 1"]
pub type EnblWrGroup5ofWdt1R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfWDT1` writer - Enable Write Group #5 of WDT 1"]
pub type EnblWrGroup5ofWdt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC127CPRIC1_27C\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric127cpric127c2924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric127cpric127c2924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric127cpric127c2924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC127CPRIC127C2924` reader - Enable Reset Tolerance of PRIC127CPRIC1_27C\\[29:24\\]"]
pub type EnblRstToleranceOfPric127cpric127c2924R =
    crate::BitReader<EnblRstToleranceOfPric127cpric127c2924>;
impl EnblRstToleranceOfPric127cpric127c2924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric127cpric127c2924 {
        match self.bits {
            false => EnblRstToleranceOfPric127cpric127c2924::ResetBySrst,
            true => EnblRstToleranceOfPric127cpric127c2924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric127cpric127c2924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric127cpric127c2924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC127CPRIC127C2924` writer - Enable Reset Tolerance of PRIC127CPRIC1_27C\\[29:24\\]"]
pub type EnblRstToleranceOfPric127cpric127c2924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric127cpric127c2924>;
impl<'a, REG> EnblRstToleranceOfPric127cpric127c2924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric127cpric127c2924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric127cpric127c2924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC127CPRIC127C3024` reader - Enable Write Protection of PRIC127CPRIC1_27C\\[30:24\\]"]
pub type EnblWrProtOfPric127cpric127c3024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC127CPRIC127C3024` writer - Enable Write Protection of PRIC127CPRIC1_27C\\[30:24\\]"]
pub type EnblWrProtOfPric127cpric127c3024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of WDT 0"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt0(&self) -> EnblWrGroup0ofWdt0R {
        EnblWrGroup0ofWdt0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of WDT 0"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt0(&self) -> EnblWrGroup1ofWdt0R {
        EnblWrGroup1ofWdt0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of WDT 0"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt0(&self) -> EnblWrGroup2ofWdt0R {
        EnblWrGroup2ofWdt0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of WDT 0"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt0(&self) -> EnblWrGroup3ofWdt0R {
        EnblWrGroup3ofWdt0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of WDT 0"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt0(&self) -> EnblWrGroup4ofWdt0R {
        EnblWrGroup4ofWdt0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of WDT 0"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt0(&self) -> EnblWrGroup5ofWdt0R {
        EnblWrGroup5ofWdt0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC127CPRIC1_27C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric127cpric127c2116(
        &self,
    ) -> EnblRstToleranceOfPric127cpric127c2116R {
        EnblRstToleranceOfPric127cpric127c2116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC127CPRIC1_27C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric127cpric127c2216(&self) -> EnblWrProtOfPric127cpric127c2216R {
        EnblWrProtOfPric127cpric127c2216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of WDT 1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt1(&self) -> EnblWrGroup0ofWdt1R {
        EnblWrGroup0ofWdt1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of WDT 1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt1(&self) -> EnblWrGroup1ofWdt1R {
        EnblWrGroup1ofWdt1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of WDT 1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt1(&self) -> EnblWrGroup2ofWdt1R {
        EnblWrGroup2ofWdt1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of WDT 1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt1(&self) -> EnblWrGroup3ofWdt1R {
        EnblWrGroup3ofWdt1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of WDT 1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt1(&self) -> EnblWrGroup4ofWdt1R {
        EnblWrGroup4ofWdt1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of WDT 1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt1(&self) -> EnblWrGroup5ofWdt1R {
        EnblWrGroup5ofWdt1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC127CPRIC1_27C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric127cpric127c2924(
        &self,
    ) -> EnblRstToleranceOfPric127cpric127c2924R {
        EnblRstToleranceOfPric127cpric127c2924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC127CPRIC1_27C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric127cpric127c3024(&self) -> EnblWrProtOfPric127cpric127c3024R {
        EnblWrProtOfPric127cpric127c3024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved15(&mut self) -> Reserved15W<PricIo27cSpec> {
        Reserved15W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&mut self) -> Reserved14W<PricIo27cSpec> {
        Reserved14W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved13(&mut self) -> Reserved13W<PricIo27cSpec> {
        Reserved13W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved12(&mut self) -> Reserved12W<PricIo27cSpec> {
        Reserved12W::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&mut self) -> Reserved11W<PricIo27cSpec> {
        Reserved11W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&mut self) -> Reserved10W<PricIo27cSpec> {
        Reserved10W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&mut self) -> Reserved9W<PricIo27cSpec> {
        Reserved9W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> Reserved8W<PricIo27cSpec> {
        Reserved8W::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo27cSpec> {
        Reserved7W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo27cSpec> {
        Reserved6W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo27cSpec> {
        Reserved5W::new(self, 10)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo27cSpec> {
        Reserved4W::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo27cSpec> {
        Reserved3W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo27cSpec> {
        Reserved2W::new(self, 13)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo27cSpec> {
        Reserved1W::new(self, 14)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of WDT 0"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt0(&mut self) -> EnblWrGroup0ofWdt0W<PricIo27cSpec> {
        EnblWrGroup0ofWdt0W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of WDT 0"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt0(&mut self) -> EnblWrGroup1ofWdt0W<PricIo27cSpec> {
        EnblWrGroup1ofWdt0W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of WDT 0"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt0(&mut self) -> EnblWrGroup2ofWdt0W<PricIo27cSpec> {
        EnblWrGroup2ofWdt0W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of WDT 0"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt0(&mut self) -> EnblWrGroup3ofWdt0W<PricIo27cSpec> {
        EnblWrGroup3ofWdt0W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of WDT 0"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt0(&mut self) -> EnblWrGroup4ofWdt0W<PricIo27cSpec> {
        EnblWrGroup4ofWdt0W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of WDT 0"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt0(&mut self) -> EnblWrGroup5ofWdt0W<PricIo27cSpec> {
        EnblWrGroup5ofWdt0W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC127CPRIC1_27C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric127cpric127c2116(
        &mut self,
    ) -> EnblRstToleranceOfPric127cpric127c2116W<PricIo27cSpec> {
        EnblRstToleranceOfPric127cpric127c2116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC127CPRIC1_27C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric127cpric127c2216(
        &mut self,
    ) -> EnblWrProtOfPric127cpric127c2216W<PricIo27cSpec> {
        EnblWrProtOfPric127cpric127c2216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of WDT 1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_wdt1(&mut self) -> EnblWrGroup0ofWdt1W<PricIo27cSpec> {
        EnblWrGroup0ofWdt1W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of WDT 1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_wdt1(&mut self) -> EnblWrGroup1ofWdt1W<PricIo27cSpec> {
        EnblWrGroup1ofWdt1W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of WDT 1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_wdt1(&mut self) -> EnblWrGroup2ofWdt1W<PricIo27cSpec> {
        EnblWrGroup2ofWdt1W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of WDT 1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_wdt1(&mut self) -> EnblWrGroup3ofWdt1W<PricIo27cSpec> {
        EnblWrGroup3ofWdt1W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of WDT 1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_wdt1(&mut self) -> EnblWrGroup4ofWdt1W<PricIo27cSpec> {
        EnblWrGroup4ofWdt1W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of WDT 1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_wdt1(&mut self) -> EnblWrGroup5ofWdt1W<PricIo27cSpec> {
        EnblWrGroup5ofWdt1W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC127CPRIC1_27C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric127cpric127c2924(
        &mut self,
    ) -> EnblRstToleranceOfPric127cpric127c2924W<PricIo27cSpec> {
        EnblRstToleranceOfPric127cpric127c2924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC127CPRIC1_27C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric127cpric127c3024(
        &mut self,
    ) -> EnblWrProtOfPric127cpric127c3024W<PricIo27cSpec> {
        EnblWrProtOfPric127cpric127c3024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#31\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io27c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io27c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo27cSpec;
impl crate::RegisterSpec for PricIo27cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io27c::R`](R) reader structure"]
impl crate::Readable for PricIo27cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io27c::W`](W) writer structure"]
impl crate::Writable for PricIo27cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO27C to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo27cSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
