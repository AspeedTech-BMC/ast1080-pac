#[doc = "Register `PRIC_IO31C` reader"]
pub type R = crate::R<PricIo31cSpec>;
#[doc = "Register `PRIC_IO31C` writer"]
pub type W = crate::W<PricIo31cSpec>;
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
#[doc = "Field `EnblReadGroup0OfADC0` reader - Enable Read Group #0 of ADC 0"]
pub type EnblReadGroup0ofAdc0R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfADC0` writer - Enable Read Group #0 of ADC 0"]
pub type EnblReadGroup0ofAdc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfADC0` reader - Enable Read Group #1 of ADC 0"]
pub type EnblReadGroup1ofAdc0R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfADC0` writer - Enable Read Group #1 of ADC 0"]
pub type EnblReadGroup1ofAdc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfADC0` reader - Enable Read Group #2 of ADC 0"]
pub type EnblReadGroup2ofAdc0R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfADC0` writer - Enable Read Group #2 of ADC 0"]
pub type EnblReadGroup2ofAdc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfADC0` reader - Enable Read Group #3 of ADC 0"]
pub type EnblReadGroup3ofAdc0R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfADC0` writer - Enable Read Group #3 of ADC 0"]
pub type EnblReadGroup3ofAdc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfADC0` reader - Enable Read Group #4 of ADC 0"]
pub type EnblReadGroup4ofAdc0R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfADC0` writer - Enable Read Group #4 of ADC 0"]
pub type EnblReadGroup4ofAdc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfADC0` reader - Enable Read Group #5 of ADC 0"]
pub type EnblReadGroup5ofAdc0R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfADC0` writer - Enable Read Group #5 of ADC 0"]
pub type EnblReadGroup5ofAdc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC131CPRIC1_31C\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric131cpric131c2924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric131cpric131c2924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric131cpric131c2924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC131CPRIC131C2924` reader - Enable Reset Tolerance of PRIC131CPRIC1_31C\\[29:24\\]"]
pub type EnblRstToleranceOfPric131cpric131c2924R =
    crate::BitReader<EnblRstToleranceOfPric131cpric131c2924>;
impl EnblRstToleranceOfPric131cpric131c2924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric131cpric131c2924 {
        match self.bits {
            false => EnblRstToleranceOfPric131cpric131c2924::ResetBySrst,
            true => EnblRstToleranceOfPric131cpric131c2924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric131cpric131c2924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric131cpric131c2924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC131CPRIC131C2924` writer - Enable Reset Tolerance of PRIC131CPRIC1_31C\\[29:24\\]"]
pub type EnblRstToleranceOfPric131cpric131c2924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric131cpric131c2924>;
impl<'a, REG> EnblRstToleranceOfPric131cpric131c2924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric131cpric131c2924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric131cpric131c2924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC131CPRIC131C3024` reader - Enable Write Protection of PRIC131CPRIC1_31C\\[30:24\\]"]
pub type EnblWrProtOfPric131cpric131c3024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC131CPRIC131C3024` writer - Enable Write Protection of PRIC131CPRIC1_31C\\[30:24\\]"]
pub type EnblWrProtOfPric131cpric131c3024W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 24 - Enable Read Group #0 of ADC 0"]
    #[inline(always)]
    pub fn enbl_read_group0of_adc0(&self) -> EnblReadGroup0ofAdc0R {
        EnblReadGroup0ofAdc0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of ADC 0"]
    #[inline(always)]
    pub fn enbl_read_group1of_adc0(&self) -> EnblReadGroup1ofAdc0R {
        EnblReadGroup1ofAdc0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of ADC 0"]
    #[inline(always)]
    pub fn enbl_read_group2of_adc0(&self) -> EnblReadGroup2ofAdc0R {
        EnblReadGroup2ofAdc0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of ADC 0"]
    #[inline(always)]
    pub fn enbl_read_group3of_adc0(&self) -> EnblReadGroup3ofAdc0R {
        EnblReadGroup3ofAdc0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of ADC 0"]
    #[inline(always)]
    pub fn enbl_read_group4of_adc0(&self) -> EnblReadGroup4ofAdc0R {
        EnblReadGroup4ofAdc0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of ADC 0"]
    #[inline(always)]
    pub fn enbl_read_group5of_adc0(&self) -> EnblReadGroup5ofAdc0R {
        EnblReadGroup5ofAdc0R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC131CPRIC1_31C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric131cpric131c2924(
        &self,
    ) -> EnblRstToleranceOfPric131cpric131c2924R {
        EnblRstToleranceOfPric131cpric131c2924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC131CPRIC1_31C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric131cpric131c3024(&self) -> EnblWrProtOfPric131cpric131c3024R {
        EnblWrProtOfPric131cpric131c3024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo31cSpec> {
        Reserved5W::new(self, 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo31cSpec> {
        Reserved4W::new(self, 7)
    }
    #[doc = "Bits 8:14 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo31cSpec> {
        Reserved3W::new(self, 8)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo31cSpec> {
        Reserved2W::new(self, 15)
    }
    #[doc = "Bits 16:22 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo31cSpec> {
        Reserved1W::new(self, 16)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of ADC 0"]
    #[inline(always)]
    pub fn enbl_read_group0of_adc0(&mut self) -> EnblReadGroup0ofAdc0W<PricIo31cSpec> {
        EnblReadGroup0ofAdc0W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of ADC 0"]
    #[inline(always)]
    pub fn enbl_read_group1of_adc0(&mut self) -> EnblReadGroup1ofAdc0W<PricIo31cSpec> {
        EnblReadGroup1ofAdc0W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of ADC 0"]
    #[inline(always)]
    pub fn enbl_read_group2of_adc0(&mut self) -> EnblReadGroup2ofAdc0W<PricIo31cSpec> {
        EnblReadGroup2ofAdc0W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of ADC 0"]
    #[inline(always)]
    pub fn enbl_read_group3of_adc0(&mut self) -> EnblReadGroup3ofAdc0W<PricIo31cSpec> {
        EnblReadGroup3ofAdc0W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of ADC 0"]
    #[inline(always)]
    pub fn enbl_read_group4of_adc0(&mut self) -> EnblReadGroup4ofAdc0W<PricIo31cSpec> {
        EnblReadGroup4ofAdc0W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of ADC 0"]
    #[inline(always)]
    pub fn enbl_read_group5of_adc0(&mut self) -> EnblReadGroup5ofAdc0W<PricIo31cSpec> {
        EnblReadGroup5ofAdc0W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC131CPRIC1_31C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric131cpric131c2924(
        &mut self,
    ) -> EnblRstToleranceOfPric131cpric131c2924W<PricIo31cSpec> {
        EnblRstToleranceOfPric131cpric131c2924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC131CPRIC1_31C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric131cpric131c3024(
        &mut self,
    ) -> EnblWrProtOfPric131cpric131c3024W<PricIo31cSpec> {
        EnblWrProtOfPric131cpric131c3024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io31c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io31c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo31cSpec;
impl crate::RegisterSpec for PricIo31cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io31c::R`](R) reader structure"]
impl crate::Readable for PricIo31cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io31c::W`](W) writer structure"]
impl crate::Writable for PricIo31cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO31C to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo31cSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
