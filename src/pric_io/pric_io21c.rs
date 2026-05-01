#[doc = "Register `PRIC_IO21C` reader"]
pub type R = crate::R<PricIo21cSpec>;
#[doc = "Register `PRIC_IO21C` writer"]
pub type W = crate::W<PricIo21cSpec>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EnblWrGroup0OfADC0` reader - Enable Write Group #0 of ADC 0"]
pub type EnblWrGroup0ofAdc0R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfADC0` writer - Enable Write Group #0 of ADC 0"]
pub type EnblWrGroup0ofAdc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfADC0` reader - Enable Write Group #1 of ADC 0"]
pub type EnblWrGroup1ofAdc0R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfADC0` writer - Enable Write Group #1 of ADC 0"]
pub type EnblWrGroup1ofAdc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfADC0` reader - Enable Write Group #2 of ADC 0"]
pub type EnblWrGroup2ofAdc0R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfADC0` writer - Enable Write Group #2 of ADC 0"]
pub type EnblWrGroup2ofAdc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfADC0` reader - Enable Write Group #3 of ADC 0"]
pub type EnblWrGroup3ofAdc0R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfADC0` writer - Enable Write Group #3 of ADC 0"]
pub type EnblWrGroup3ofAdc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfADC0` reader - Enable Write Group #4 of ADC 0"]
pub type EnblWrGroup4ofAdc0R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfADC0` writer - Enable Write Group #4 of ADC 0"]
pub type EnblWrGroup4ofAdc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfADC0` reader - Enable Write Group #5 of ADC 0"]
pub type EnblWrGroup5ofAdc0R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfADC0` writer - Enable Write Group #5 of ADC 0"]
pub type EnblWrGroup5ofAdc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC121CPRIC1_21C\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric121cpric121c2924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric121cpric121c2924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric121cpric121c2924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC121CPRIC121C2924` reader - Enable Reset Tolerance of PRIC121CPRIC1_21C\\[29:24\\]"]
pub type EnblRstToleranceOfPric121cpric121c2924R =
    crate::BitReader<EnblRstToleranceOfPric121cpric121c2924>;
impl EnblRstToleranceOfPric121cpric121c2924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric121cpric121c2924 {
        match self.bits {
            false => EnblRstToleranceOfPric121cpric121c2924::ResetBySrst,
            true => EnblRstToleranceOfPric121cpric121c2924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric121cpric121c2924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric121cpric121c2924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC121CPRIC121C2924` writer - Enable Reset Tolerance of PRIC121CPRIC1_21C\\[29:24\\]"]
pub type EnblRstToleranceOfPric121cpric121c2924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric121cpric121c2924>;
impl<'a, REG> EnblRstToleranceOfPric121cpric121c2924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric121cpric121c2924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric121cpric121c2924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC121CPRIC121C3024` reader - Enable Write Protection of PRIC121CPRIC1_21C\\[30:24\\]"]
pub type EnblWrProtOfPric121cpric121c3024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC121CPRIC121C3024` writer - Enable Write Protection of PRIC121CPRIC1_21C\\[30:24\\]"]
pub type EnblWrProtOfPric121cpric121c3024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of ADC 0"]
    #[inline(always)]
    pub fn enbl_wr_group0of_adc0(&self) -> EnblWrGroup0ofAdc0R {
        EnblWrGroup0ofAdc0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of ADC 0"]
    #[inline(always)]
    pub fn enbl_wr_group1of_adc0(&self) -> EnblWrGroup1ofAdc0R {
        EnblWrGroup1ofAdc0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of ADC 0"]
    #[inline(always)]
    pub fn enbl_wr_group2of_adc0(&self) -> EnblWrGroup2ofAdc0R {
        EnblWrGroup2ofAdc0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of ADC 0"]
    #[inline(always)]
    pub fn enbl_wr_group3of_adc0(&self) -> EnblWrGroup3ofAdc0R {
        EnblWrGroup3ofAdc0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of ADC 0"]
    #[inline(always)]
    pub fn enbl_wr_group4of_adc0(&self) -> EnblWrGroup4ofAdc0R {
        EnblWrGroup4ofAdc0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of ADC 0"]
    #[inline(always)]
    pub fn enbl_wr_group5of_adc0(&self) -> EnblWrGroup5ofAdc0R {
        EnblWrGroup5ofAdc0R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC121CPRIC1_21C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric121cpric121c2924(
        &self,
    ) -> EnblRstToleranceOfPric121cpric121c2924R {
        EnblRstToleranceOfPric121cpric121c2924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC121CPRIC1_21C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric121cpric121c3024(&self) -> EnblWrProtOfPric121cpric121c3024R {
        EnblWrProtOfPric121cpric121c3024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo21cSpec> {
        Reserved5W::new(self, 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo21cSpec> {
        Reserved4W::new(self, 7)
    }
    #[doc = "Bits 8:14 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo21cSpec> {
        Reserved3W::new(self, 8)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo21cSpec> {
        Reserved2W::new(self, 15)
    }
    #[doc = "Bits 16:22 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo21cSpec> {
        Reserved1W::new(self, 16)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of ADC 0"]
    #[inline(always)]
    pub fn enbl_wr_group0of_adc0(&mut self) -> EnblWrGroup0ofAdc0W<PricIo21cSpec> {
        EnblWrGroup0ofAdc0W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of ADC 0"]
    #[inline(always)]
    pub fn enbl_wr_group1of_adc0(&mut self) -> EnblWrGroup1ofAdc0W<PricIo21cSpec> {
        EnblWrGroup1ofAdc0W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of ADC 0"]
    #[inline(always)]
    pub fn enbl_wr_group2of_adc0(&mut self) -> EnblWrGroup2ofAdc0W<PricIo21cSpec> {
        EnblWrGroup2ofAdc0W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of ADC 0"]
    #[inline(always)]
    pub fn enbl_wr_group3of_adc0(&mut self) -> EnblWrGroup3ofAdc0W<PricIo21cSpec> {
        EnblWrGroup3ofAdc0W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of ADC 0"]
    #[inline(always)]
    pub fn enbl_wr_group4of_adc0(&mut self) -> EnblWrGroup4ofAdc0W<PricIo21cSpec> {
        EnblWrGroup4ofAdc0W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of ADC 0"]
    #[inline(always)]
    pub fn enbl_wr_group5of_adc0(&mut self) -> EnblWrGroup5ofAdc0W<PricIo21cSpec> {
        EnblWrGroup5ofAdc0W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC121CPRIC1_21C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric121cpric121c2924(
        &mut self,
    ) -> EnblRstToleranceOfPric121cpric121c2924W<PricIo21cSpec> {
        EnblRstToleranceOfPric121cpric121c2924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC121CPRIC1_21C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric121cpric121c3024(
        &mut self,
    ) -> EnblWrProtOfPric121cpric121c3024W<PricIo21cSpec> {
        EnblWrProtOfPric121cpric121c3024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io21c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io21c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo21cSpec;
impl crate::RegisterSpec for PricIo21cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io21c::R`](R) reader structure"]
impl crate::Readable for PricIo21cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io21c::W`](W) writer structure"]
impl crate::Writable for PricIo21cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO21C to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo21cSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
