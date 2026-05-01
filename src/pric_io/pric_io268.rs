#[doc = "Register `PRIC_IO268` reader"]
pub type R = crate::R<PricIo268Spec>;
#[doc = "Register `PRIC_IO268` writer"]
pub type W = crate::W<PricIo268Spec>;
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
#[doc = "Field `EnblWrGroup0OfVUART1` reader - Enable Write Group #0 of VUART1"]
pub type EnblWrGroup0ofVuart1R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfVUART1` writer - Enable Write Group #0 of VUART1"]
pub type EnblWrGroup0ofVuart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfVUART1` reader - Enable Write Group #1 of VUART1"]
pub type EnblWrGroup1ofVuart1R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfVUART1` writer - Enable Write Group #1 of VUART1"]
pub type EnblWrGroup1ofVuart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfVUART1` reader - Enable Write Group #2 of VUART1"]
pub type EnblWrGroup2ofVuart1R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfVUART1` writer - Enable Write Group #2 of VUART1"]
pub type EnblWrGroup2ofVuart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfVUART1` reader - Enable Write Group #3 of VUART1"]
pub type EnblWrGroup3ofVuart1R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfVUART1` writer - Enable Write Group #3 of VUART1"]
pub type EnblWrGroup3ofVuart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfVUART1` reader - Enable Write Group #4 of VUART1"]
pub type EnblWrGroup4ofVuart1R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfVUART1` writer - Enable Write Group #4 of VUART1"]
pub type EnblWrGroup4ofVuart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfVUART1` reader - Enable Write Group #5 of VUART1"]
pub type EnblWrGroup5ofVuart1R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfVUART1` writer - Enable Write Group #5 of VUART1"]
pub type EnblWrGroup5ofVuart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1268PRIC1_268\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1268pric12682116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1268pric12682116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1268pric12682116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1268PRIC12682116` reader - Enable Reset Tolerance of PRIC1268PRIC1_268\\[21:16\\]"]
pub type EnblRstToleranceOfPric1268pric12682116R =
    crate::BitReader<EnblRstToleranceOfPric1268pric12682116>;
impl EnblRstToleranceOfPric1268pric12682116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1268pric12682116 {
        match self.bits {
            false => EnblRstToleranceOfPric1268pric12682116::ResetBySrst,
            true => EnblRstToleranceOfPric1268pric12682116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1268pric12682116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1268pric12682116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1268PRIC12682116` writer - Enable Reset Tolerance of PRIC1268PRIC1_268\\[21:16\\]"]
pub type EnblRstToleranceOfPric1268pric12682116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1268pric12682116>;
impl<'a, REG> EnblRstToleranceOfPric1268pric12682116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1268pric12682116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1268pric12682116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1268PRIC12682216` reader - Enable Write Protection of PRIC1268PRIC1_268\\[22:16\\]"]
pub type EnblWrProtOfPric1268pric12682216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1268PRIC12682216` writer - Enable Write Protection of PRIC1268PRIC1_268\\[22:16\\]"]
pub type EnblWrProtOfPric1268pric12682216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfVUART2` reader - Enable Write Group #0 of VUART2"]
pub type EnblWrGroup0ofVuart2R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfVUART2` writer - Enable Write Group #0 of VUART2"]
pub type EnblWrGroup0ofVuart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfVUART2` reader - Enable Write Group #1 of VUART2"]
pub type EnblWrGroup1ofVuart2R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfVUART2` writer - Enable Write Group #1 of VUART2"]
pub type EnblWrGroup1ofVuart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfVUART2` reader - Enable Write Group #2 of VUART2"]
pub type EnblWrGroup2ofVuart2R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfVUART2` writer - Enable Write Group #2 of VUART2"]
pub type EnblWrGroup2ofVuart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfVUART2` reader - Enable Write Group #3 of VUART2"]
pub type EnblWrGroup3ofVuart2R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfVUART2` writer - Enable Write Group #3 of VUART2"]
pub type EnblWrGroup3ofVuart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfVUART2` reader - Enable Write Group #4 of VUART2"]
pub type EnblWrGroup4ofVuart2R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfVUART2` writer - Enable Write Group #4 of VUART2"]
pub type EnblWrGroup4ofVuart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfVUART2` reader - Enable Write Group #5 of VUART2"]
pub type EnblWrGroup5ofVuart2R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfVUART2` writer - Enable Write Group #5 of VUART2"]
pub type EnblWrGroup5ofVuart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1268PRIC1_268\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1268pric12682924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1268pric12682924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1268pric12682924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1268PRIC12682924` reader - Enable Reset Tolerance of PRIC1268PRIC1_268\\[29:24\\]"]
pub type EnblRstToleranceOfPric1268pric12682924R =
    crate::BitReader<EnblRstToleranceOfPric1268pric12682924>;
impl EnblRstToleranceOfPric1268pric12682924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1268pric12682924 {
        match self.bits {
            false => EnblRstToleranceOfPric1268pric12682924::ResetBySrst,
            true => EnblRstToleranceOfPric1268pric12682924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1268pric12682924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1268pric12682924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1268PRIC12682924` writer - Enable Reset Tolerance of PRIC1268PRIC1_268\\[29:24\\]"]
pub type EnblRstToleranceOfPric1268pric12682924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1268pric12682924>;
impl<'a, REG> EnblRstToleranceOfPric1268pric12682924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1268pric12682924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1268pric12682924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1268PRIC12683024` reader - Enable Write Protection of PRIC1268PRIC1_268\\[30:24\\]"]
pub type EnblWrProtOfPric1268pric12683024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1268PRIC12683024` writer - Enable Write Protection of PRIC1268PRIC1_268\\[30:24\\]"]
pub type EnblWrProtOfPric1268pric12683024W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 16 - Enable Write Group #0 of VUART1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_vuart1(&self) -> EnblWrGroup0ofVuart1R {
        EnblWrGroup0ofVuart1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of VUART1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_vuart1(&self) -> EnblWrGroup1ofVuart1R {
        EnblWrGroup1ofVuart1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of VUART1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_vuart1(&self) -> EnblWrGroup2ofVuart1R {
        EnblWrGroup2ofVuart1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of VUART1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_vuart1(&self) -> EnblWrGroup3ofVuart1R {
        EnblWrGroup3ofVuart1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of VUART1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_vuart1(&self) -> EnblWrGroup4ofVuart1R {
        EnblWrGroup4ofVuart1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of VUART1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_vuart1(&self) -> EnblWrGroup5ofVuart1R {
        EnblWrGroup5ofVuart1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1268PRIC1_268\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1268pric12682116(
        &self,
    ) -> EnblRstToleranceOfPric1268pric12682116R {
        EnblRstToleranceOfPric1268pric12682116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1268PRIC1_268\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1268pric12682216(&self) -> EnblWrProtOfPric1268pric12682216R {
        EnblWrProtOfPric1268pric12682216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of VUART2"]
    #[inline(always)]
    pub fn enbl_wr_group0of_vuart2(&self) -> EnblWrGroup0ofVuart2R {
        EnblWrGroup0ofVuart2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of VUART2"]
    #[inline(always)]
    pub fn enbl_wr_group1of_vuart2(&self) -> EnblWrGroup1ofVuart2R {
        EnblWrGroup1ofVuart2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of VUART2"]
    #[inline(always)]
    pub fn enbl_wr_group2of_vuart2(&self) -> EnblWrGroup2ofVuart2R {
        EnblWrGroup2ofVuart2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of VUART2"]
    #[inline(always)]
    pub fn enbl_wr_group3of_vuart2(&self) -> EnblWrGroup3ofVuart2R {
        EnblWrGroup3ofVuart2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of VUART2"]
    #[inline(always)]
    pub fn enbl_wr_group4of_vuart2(&self) -> EnblWrGroup4ofVuart2R {
        EnblWrGroup4ofVuart2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of VUART2"]
    #[inline(always)]
    pub fn enbl_wr_group5of_vuart2(&self) -> EnblWrGroup5ofVuart2R {
        EnblWrGroup5ofVuart2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1268PRIC1_268\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1268pric12682924(
        &self,
    ) -> EnblRstToleranceOfPric1268pric12682924R {
        EnblRstToleranceOfPric1268pric12682924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1268PRIC1_268\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1268pric12683024(&self) -> EnblWrProtOfPric1268pric12683024R {
        EnblWrProtOfPric1268pric12683024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved15(&mut self) -> Reserved15W<PricIo268Spec> {
        Reserved15W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&mut self) -> Reserved14W<PricIo268Spec> {
        Reserved14W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved13(&mut self) -> Reserved13W<PricIo268Spec> {
        Reserved13W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved12(&mut self) -> Reserved12W<PricIo268Spec> {
        Reserved12W::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&mut self) -> Reserved11W<PricIo268Spec> {
        Reserved11W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&mut self) -> Reserved10W<PricIo268Spec> {
        Reserved10W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&mut self) -> Reserved9W<PricIo268Spec> {
        Reserved9W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> Reserved8W<PricIo268Spec> {
        Reserved8W::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo268Spec> {
        Reserved7W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo268Spec> {
        Reserved6W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo268Spec> {
        Reserved5W::new(self, 10)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo268Spec> {
        Reserved4W::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo268Spec> {
        Reserved3W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo268Spec> {
        Reserved2W::new(self, 13)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo268Spec> {
        Reserved1W::new(self, 14)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of VUART1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_vuart1(&mut self) -> EnblWrGroup0ofVuart1W<PricIo268Spec> {
        EnblWrGroup0ofVuart1W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of VUART1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_vuart1(&mut self) -> EnblWrGroup1ofVuart1W<PricIo268Spec> {
        EnblWrGroup1ofVuart1W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of VUART1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_vuart1(&mut self) -> EnblWrGroup2ofVuart1W<PricIo268Spec> {
        EnblWrGroup2ofVuart1W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of VUART1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_vuart1(&mut self) -> EnblWrGroup3ofVuart1W<PricIo268Spec> {
        EnblWrGroup3ofVuart1W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of VUART1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_vuart1(&mut self) -> EnblWrGroup4ofVuart1W<PricIo268Spec> {
        EnblWrGroup4ofVuart1W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of VUART1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_vuart1(&mut self) -> EnblWrGroup5ofVuart1W<PricIo268Spec> {
        EnblWrGroup5ofVuart1W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1268PRIC1_268\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1268pric12682116(
        &mut self,
    ) -> EnblRstToleranceOfPric1268pric12682116W<PricIo268Spec> {
        EnblRstToleranceOfPric1268pric12682116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1268PRIC1_268\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1268pric12682216(
        &mut self,
    ) -> EnblWrProtOfPric1268pric12682216W<PricIo268Spec> {
        EnblWrProtOfPric1268pric12682216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of VUART2"]
    #[inline(always)]
    pub fn enbl_wr_group0of_vuart2(&mut self) -> EnblWrGroup0ofVuart2W<PricIo268Spec> {
        EnblWrGroup0ofVuart2W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of VUART2"]
    #[inline(always)]
    pub fn enbl_wr_group1of_vuart2(&mut self) -> EnblWrGroup1ofVuart2W<PricIo268Spec> {
        EnblWrGroup1ofVuart2W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of VUART2"]
    #[inline(always)]
    pub fn enbl_wr_group2of_vuart2(&mut self) -> EnblWrGroup2ofVuart2W<PricIo268Spec> {
        EnblWrGroup2ofVuart2W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of VUART2"]
    #[inline(always)]
    pub fn enbl_wr_group3of_vuart2(&mut self) -> EnblWrGroup3ofVuart2W<PricIo268Spec> {
        EnblWrGroup3ofVuart2W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of VUART2"]
    #[inline(always)]
    pub fn enbl_wr_group4of_vuart2(&mut self) -> EnblWrGroup4ofVuart2W<PricIo268Spec> {
        EnblWrGroup4ofVuart2W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of VUART2"]
    #[inline(always)]
    pub fn enbl_wr_group5of_vuart2(&mut self) -> EnblWrGroup5ofVuart2W<PricIo268Spec> {
        EnblWrGroup5ofVuart2W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1268PRIC1_268\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1268pric12682924(
        &mut self,
    ) -> EnblRstToleranceOfPric1268pric12682924W<PricIo268Spec> {
        EnblRstToleranceOfPric1268pric12682924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1268PRIC1_268\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1268pric12683024(
        &mut self,
    ) -> EnblWrProtOfPric1268pric12683024W<PricIo268Spec> {
        EnblWrProtOfPric1268pric12683024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#26\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io268::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io268::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo268Spec;
impl crate::RegisterSpec for PricIo268Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io268::R`](R) reader structure"]
impl crate::Readable for PricIo268Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io268::W`](W) writer structure"]
impl crate::Writable for PricIo268Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO268 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo268Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
