#[doc = "Register `PRIC_IO368` reader"]
pub type R = crate::R<PricIo368Spec>;
#[doc = "Register `PRIC_IO368` writer"]
pub type W = crate::W<PricIo368Spec>;
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
#[doc = "Field `EnblReadGroup0OfVUART1` reader - Enable Read Group #0 of VUART1"]
pub type EnblReadGroup0ofVuart1R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfVUART1` writer - Enable Read Group #0 of VUART1"]
pub type EnblReadGroup0ofVuart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfVUART1` reader - Enable Read Group #1 of VUART1"]
pub type EnblReadGroup1ofVuart1R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfVUART1` writer - Enable Read Group #1 of VUART1"]
pub type EnblReadGroup1ofVuart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfVUART1` reader - Enable Read Group #2 of VUART1"]
pub type EnblReadGroup2ofVuart1R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfVUART1` writer - Enable Read Group #2 of VUART1"]
pub type EnblReadGroup2ofVuart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfVUART1` reader - Enable Read Group #3 of VUART1"]
pub type EnblReadGroup3ofVuart1R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfVUART1` writer - Enable Read Group #3 of VUART1"]
pub type EnblReadGroup3ofVuart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfVUART1` reader - Enable Read Group #4 of VUART1"]
pub type EnblReadGroup4ofVuart1R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfVUART1` writer - Enable Read Group #4 of VUART1"]
pub type EnblReadGroup4ofVuart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfVUART1` reader - Enable Read Group #5 of VUART1"]
pub type EnblReadGroup5ofVuart1R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfVUART1` writer - Enable Read Group #5 of VUART1"]
pub type EnblReadGroup5ofVuart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1368PRIC1_368\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1368pric13682116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1368pric13682116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1368pric13682116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1368PRIC13682116` reader - Enable Reset Tolerance of PRIC1368PRIC1_368\\[21:16\\]"]
pub type EnblRstToleranceOfPric1368pric13682116R =
    crate::BitReader<EnblRstToleranceOfPric1368pric13682116>;
impl EnblRstToleranceOfPric1368pric13682116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1368pric13682116 {
        match self.bits {
            false => EnblRstToleranceOfPric1368pric13682116::ResetBySrst,
            true => EnblRstToleranceOfPric1368pric13682116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1368pric13682116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1368pric13682116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1368PRIC13682116` writer - Enable Reset Tolerance of PRIC1368PRIC1_368\\[21:16\\]"]
pub type EnblRstToleranceOfPric1368pric13682116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1368pric13682116>;
impl<'a, REG> EnblRstToleranceOfPric1368pric13682116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1368pric13682116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1368pric13682116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1368PRIC13682216` reader - Enable Write Protection of PRIC1368PRIC1_368\\[22:16\\]"]
pub type EnblWrProtOfPric1368pric13682216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1368PRIC13682216` writer - Enable Write Protection of PRIC1368PRIC1_368\\[22:16\\]"]
pub type EnblWrProtOfPric1368pric13682216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfVUART2` reader - Enable Read Group #0 of VUART2"]
pub type EnblReadGroup0ofVuart2R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfVUART2` writer - Enable Read Group #0 of VUART2"]
pub type EnblReadGroup0ofVuart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfVUART2` reader - Enable Read Group #1 of VUART2"]
pub type EnblReadGroup1ofVuart2R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfVUART2` writer - Enable Read Group #1 of VUART2"]
pub type EnblReadGroup1ofVuart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfVUART2` reader - Enable Read Group #2 of VUART2"]
pub type EnblReadGroup2ofVuart2R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfVUART2` writer - Enable Read Group #2 of VUART2"]
pub type EnblReadGroup2ofVuart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfVUART2` reader - Enable Read Group #3 of VUART2"]
pub type EnblReadGroup3ofVuart2R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfVUART2` writer - Enable Read Group #3 of VUART2"]
pub type EnblReadGroup3ofVuart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfVUART2` reader - Enable Read Group #4 of VUART2"]
pub type EnblReadGroup4ofVuart2R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfVUART2` writer - Enable Read Group #4 of VUART2"]
pub type EnblReadGroup4ofVuart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfVUART2` reader - Enable Read Group #5 of VUART2"]
pub type EnblReadGroup5ofVuart2R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfVUART2` writer - Enable Read Group #5 of VUART2"]
pub type EnblReadGroup5ofVuart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1368PRIC1_368\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1368pric13682924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1368pric13682924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1368pric13682924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1368PRIC13682924` reader - Enable Reset Tolerance of PRIC1368PRIC1_368\\[29:24\\]"]
pub type EnblRstToleranceOfPric1368pric13682924R =
    crate::BitReader<EnblRstToleranceOfPric1368pric13682924>;
impl EnblRstToleranceOfPric1368pric13682924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1368pric13682924 {
        match self.bits {
            false => EnblRstToleranceOfPric1368pric13682924::ResetBySrst,
            true => EnblRstToleranceOfPric1368pric13682924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1368pric13682924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1368pric13682924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1368PRIC13682924` writer - Enable Reset Tolerance of PRIC1368PRIC1_368\\[29:24\\]"]
pub type EnblRstToleranceOfPric1368pric13682924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1368pric13682924>;
impl<'a, REG> EnblRstToleranceOfPric1368pric13682924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1368pric13682924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1368pric13682924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1368PRIC13683024` reader - Enable Write Protection of PRIC1368PRIC1_368\\[30:24\\]"]
pub type EnblWrProtOfPric1368pric13683024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1368PRIC13683024` writer - Enable Write Protection of PRIC1368PRIC1_368\\[30:24\\]"]
pub type EnblWrProtOfPric1368pric13683024W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 16 - Enable Read Group #0 of VUART1"]
    #[inline(always)]
    pub fn enbl_read_group0of_vuart1(&self) -> EnblReadGroup0ofVuart1R {
        EnblReadGroup0ofVuart1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of VUART1"]
    #[inline(always)]
    pub fn enbl_read_group1of_vuart1(&self) -> EnblReadGroup1ofVuart1R {
        EnblReadGroup1ofVuart1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of VUART1"]
    #[inline(always)]
    pub fn enbl_read_group2of_vuart1(&self) -> EnblReadGroup2ofVuart1R {
        EnblReadGroup2ofVuart1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of VUART1"]
    #[inline(always)]
    pub fn enbl_read_group3of_vuart1(&self) -> EnblReadGroup3ofVuart1R {
        EnblReadGroup3ofVuart1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of VUART1"]
    #[inline(always)]
    pub fn enbl_read_group4of_vuart1(&self) -> EnblReadGroup4ofVuart1R {
        EnblReadGroup4ofVuart1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of VUART1"]
    #[inline(always)]
    pub fn enbl_read_group5of_vuart1(&self) -> EnblReadGroup5ofVuart1R {
        EnblReadGroup5ofVuart1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1368PRIC1_368\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1368pric13682116(
        &self,
    ) -> EnblRstToleranceOfPric1368pric13682116R {
        EnblRstToleranceOfPric1368pric13682116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1368PRIC1_368\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1368pric13682216(&self) -> EnblWrProtOfPric1368pric13682216R {
        EnblWrProtOfPric1368pric13682216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of VUART2"]
    #[inline(always)]
    pub fn enbl_read_group0of_vuart2(&self) -> EnblReadGroup0ofVuart2R {
        EnblReadGroup0ofVuart2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of VUART2"]
    #[inline(always)]
    pub fn enbl_read_group1of_vuart2(&self) -> EnblReadGroup1ofVuart2R {
        EnblReadGroup1ofVuart2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of VUART2"]
    #[inline(always)]
    pub fn enbl_read_group2of_vuart2(&self) -> EnblReadGroup2ofVuart2R {
        EnblReadGroup2ofVuart2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of VUART2"]
    #[inline(always)]
    pub fn enbl_read_group3of_vuart2(&self) -> EnblReadGroup3ofVuart2R {
        EnblReadGroup3ofVuart2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of VUART2"]
    #[inline(always)]
    pub fn enbl_read_group4of_vuart2(&self) -> EnblReadGroup4ofVuart2R {
        EnblReadGroup4ofVuart2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of VUART2"]
    #[inline(always)]
    pub fn enbl_read_group5of_vuart2(&self) -> EnblReadGroup5ofVuart2R {
        EnblReadGroup5ofVuart2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1368PRIC1_368\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1368pric13682924(
        &self,
    ) -> EnblRstToleranceOfPric1368pric13682924R {
        EnblRstToleranceOfPric1368pric13682924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1368PRIC1_368\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1368pric13683024(&self) -> EnblWrProtOfPric1368pric13683024R {
        EnblWrProtOfPric1368pric13683024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved15(&mut self) -> Reserved15W<PricIo368Spec> {
        Reserved15W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&mut self) -> Reserved14W<PricIo368Spec> {
        Reserved14W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved13(&mut self) -> Reserved13W<PricIo368Spec> {
        Reserved13W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved12(&mut self) -> Reserved12W<PricIo368Spec> {
        Reserved12W::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&mut self) -> Reserved11W<PricIo368Spec> {
        Reserved11W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&mut self) -> Reserved10W<PricIo368Spec> {
        Reserved10W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&mut self) -> Reserved9W<PricIo368Spec> {
        Reserved9W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> Reserved8W<PricIo368Spec> {
        Reserved8W::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo368Spec> {
        Reserved7W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo368Spec> {
        Reserved6W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo368Spec> {
        Reserved5W::new(self, 10)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo368Spec> {
        Reserved4W::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo368Spec> {
        Reserved3W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo368Spec> {
        Reserved2W::new(self, 13)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo368Spec> {
        Reserved1W::new(self, 14)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of VUART1"]
    #[inline(always)]
    pub fn enbl_read_group0of_vuart1(&mut self) -> EnblReadGroup0ofVuart1W<PricIo368Spec> {
        EnblReadGroup0ofVuart1W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of VUART1"]
    #[inline(always)]
    pub fn enbl_read_group1of_vuart1(&mut self) -> EnblReadGroup1ofVuart1W<PricIo368Spec> {
        EnblReadGroup1ofVuart1W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of VUART1"]
    #[inline(always)]
    pub fn enbl_read_group2of_vuart1(&mut self) -> EnblReadGroup2ofVuart1W<PricIo368Spec> {
        EnblReadGroup2ofVuart1W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of VUART1"]
    #[inline(always)]
    pub fn enbl_read_group3of_vuart1(&mut self) -> EnblReadGroup3ofVuart1W<PricIo368Spec> {
        EnblReadGroup3ofVuart1W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of VUART1"]
    #[inline(always)]
    pub fn enbl_read_group4of_vuart1(&mut self) -> EnblReadGroup4ofVuart1W<PricIo368Spec> {
        EnblReadGroup4ofVuart1W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of VUART1"]
    #[inline(always)]
    pub fn enbl_read_group5of_vuart1(&mut self) -> EnblReadGroup5ofVuart1W<PricIo368Spec> {
        EnblReadGroup5ofVuart1W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1368PRIC1_368\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1368pric13682116(
        &mut self,
    ) -> EnblRstToleranceOfPric1368pric13682116W<PricIo368Spec> {
        EnblRstToleranceOfPric1368pric13682116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1368PRIC1_368\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1368pric13682216(
        &mut self,
    ) -> EnblWrProtOfPric1368pric13682216W<PricIo368Spec> {
        EnblWrProtOfPric1368pric13682216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of VUART2"]
    #[inline(always)]
    pub fn enbl_read_group0of_vuart2(&mut self) -> EnblReadGroup0ofVuart2W<PricIo368Spec> {
        EnblReadGroup0ofVuart2W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of VUART2"]
    #[inline(always)]
    pub fn enbl_read_group1of_vuart2(&mut self) -> EnblReadGroup1ofVuart2W<PricIo368Spec> {
        EnblReadGroup1ofVuart2W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of VUART2"]
    #[inline(always)]
    pub fn enbl_read_group2of_vuart2(&mut self) -> EnblReadGroup2ofVuart2W<PricIo368Spec> {
        EnblReadGroup2ofVuart2W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of VUART2"]
    #[inline(always)]
    pub fn enbl_read_group3of_vuart2(&mut self) -> EnblReadGroup3ofVuart2W<PricIo368Spec> {
        EnblReadGroup3ofVuart2W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of VUART2"]
    #[inline(always)]
    pub fn enbl_read_group4of_vuart2(&mut self) -> EnblReadGroup4ofVuart2W<PricIo368Spec> {
        EnblReadGroup4ofVuart2W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of VUART2"]
    #[inline(always)]
    pub fn enbl_read_group5of_vuart2(&mut self) -> EnblReadGroup5ofVuart2W<PricIo368Spec> {
        EnblReadGroup5ofVuart2W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1368PRIC1_368\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1368pric13682924(
        &mut self,
    ) -> EnblRstToleranceOfPric1368pric13682924W<PricIo368Spec> {
        EnblRstToleranceOfPric1368pric13682924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1368PRIC1_368\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1368pric13683024(
        &mut self,
    ) -> EnblWrProtOfPric1368pric13683024W<PricIo368Spec> {
        EnblWrProtOfPric1368pric13683024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#26\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io368::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io368::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo368Spec;
impl crate::RegisterSpec for PricIo368Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io368::R`](R) reader structure"]
impl crate::Readable for PricIo368Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io368::W`](W) writer structure"]
impl crate::Writable for PricIo368Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO368 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo368Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
