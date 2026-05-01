#[doc = "Register `PRIC_IO220` reader"]
pub type R = crate::R<PricIo220Spec>;
#[doc = "Register `PRIC_IO220` writer"]
pub type W = crate::W<PricIo220Spec>;
#[doc = "Field `EnblWrGroup0OfADC1` reader - Enable Write Group #0 of ADC 1"]
pub type EnblWrGroup0ofAdc1R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfADC1` writer - Enable Write Group #0 of ADC 1"]
pub type EnblWrGroup0ofAdc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfADC1` reader - Enable Write Group #1 of ADC 1"]
pub type EnblWrGroup1ofAdc1R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfADC1` writer - Enable Write Group #1 of ADC 1"]
pub type EnblWrGroup1ofAdc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfADC1` reader - Enable Write Group #2 of ADC 1"]
pub type EnblWrGroup2ofAdc1R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfADC1` writer - Enable Write Group #2 of ADC 1"]
pub type EnblWrGroup2ofAdc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfADC1` reader - Enable Write Group #3 of ADC 1"]
pub type EnblWrGroup3ofAdc1R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfADC1` writer - Enable Write Group #3 of ADC 1"]
pub type EnblWrGroup3ofAdc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfADC1` reader - Enable Write Group #4 of ADC 1"]
pub type EnblWrGroup4ofAdc1R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfADC1` writer - Enable Write Group #4 of ADC 1"]
pub type EnblWrGroup4ofAdc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfADC1` reader - Enable Write Group #5 of ADC 1"]
pub type EnblWrGroup5ofAdc1R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfADC1` writer - Enable Write Group #5 of ADC 1"]
pub type EnblWrGroup5ofAdc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1220PRIC1_220\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1220pric12200500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1220pric12200500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1220pric12200500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1220PRIC12200500` reader - Enable Reset Tolerance of PRIC1220PRIC1_220\\[05:00\\]"]
pub type EnblRstToleranceOfPric1220pric12200500R =
    crate::BitReader<EnblRstToleranceOfPric1220pric12200500>;
impl EnblRstToleranceOfPric1220pric12200500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1220pric12200500 {
        match self.bits {
            false => EnblRstToleranceOfPric1220pric12200500::ResetBySrst,
            true => EnblRstToleranceOfPric1220pric12200500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1220pric12200500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1220pric12200500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1220PRIC12200500` writer - Enable Reset Tolerance of PRIC1220PRIC1_220\\[05:00\\]"]
pub type EnblRstToleranceOfPric1220pric12200500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1220pric12200500>;
impl<'a, REG> EnblRstToleranceOfPric1220pric12200500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1220pric12200500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1220pric12200500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1220PRIC12200600` reader - Enable Write Protection of PRIC1220PRIC1_220\\[06:00\\]"]
pub type EnblWrProtOfPric1220pric12200600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1220PRIC12200600` writer - Enable Write Protection of PRIC1220PRIC1_220\\[06:00\\]"]
pub type EnblWrProtOfPric1220pric12200600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfJTAG0` reader - Enable Write Group #0 of JTAG0"]
pub type EnblWrGroup0ofJtag0R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfJTAG0` writer - Enable Write Group #0 of JTAG0"]
pub type EnblWrGroup0ofJtag0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfJTAG0` reader - Enable Write Group #1 of JTAG0"]
pub type EnblWrGroup1ofJtag0R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfJTAG0` writer - Enable Write Group #1 of JTAG0"]
pub type EnblWrGroup1ofJtag0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfJTAG0` reader - Enable Write Group #2 of JTAG0"]
pub type EnblWrGroup2ofJtag0R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfJTAG0` writer - Enable Write Group #2 of JTAG0"]
pub type EnblWrGroup2ofJtag0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfJTAG0` reader - Enable Write Group #3 of JTAG0"]
pub type EnblWrGroup3ofJtag0R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfJTAG0` writer - Enable Write Group #3 of JTAG0"]
pub type EnblWrGroup3ofJtag0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfJTAG0` reader - Enable Write Group #4 of JTAG0"]
pub type EnblWrGroup4ofJtag0R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfJTAG0` writer - Enable Write Group #4 of JTAG0"]
pub type EnblWrGroup4ofJtag0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfJTAG0` reader - Enable Write Group #5 of JTAG0"]
pub type EnblWrGroup5ofJtag0R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfJTAG0` writer - Enable Write Group #5 of JTAG0"]
pub type EnblWrGroup5ofJtag0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1220PRIC1_220\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1220pric12201308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1220pric12201308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1220pric12201308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1220PRIC12201308` reader - Enable Reset Tolerance of PRIC1220PRIC1_220\\[13:08\\]"]
pub type EnblRstToleranceOfPric1220pric12201308R =
    crate::BitReader<EnblRstToleranceOfPric1220pric12201308>;
impl EnblRstToleranceOfPric1220pric12201308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1220pric12201308 {
        match self.bits {
            false => EnblRstToleranceOfPric1220pric12201308::ResetBySrst,
            true => EnblRstToleranceOfPric1220pric12201308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1220pric12201308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1220pric12201308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1220PRIC12201308` writer - Enable Reset Tolerance of PRIC1220PRIC1_220\\[13:08\\]"]
pub type EnblRstToleranceOfPric1220pric12201308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1220pric12201308>;
impl<'a, REG> EnblRstToleranceOfPric1220pric12201308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1220pric12201308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1220pric12201308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1220PRIC12201408` reader - Enable Write Protection of PRIC1220PRIC1_220\\[14:08\\]"]
pub type EnblWrProtOfPric1220pric12201408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1220PRIC12201408` writer - Enable Write Protection of PRIC1220PRIC1_220\\[14:08\\]"]
pub type EnblWrProtOfPric1220pric12201408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfSCU` reader - Enable Write Group #0 of SCU"]
pub type EnblWrGroup0ofScuR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfSCU` writer - Enable Write Group #0 of SCU"]
pub type EnblWrGroup0ofScuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfSCU` reader - Enable Write Group #1 of SCU"]
pub type EnblWrGroup1ofScuR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfSCU` writer - Enable Write Group #1 of SCU"]
pub type EnblWrGroup1ofScuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfSCU` reader - Enable Write Group #2 of SCU"]
pub type EnblWrGroup2ofScuR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfSCU` writer - Enable Write Group #2 of SCU"]
pub type EnblWrGroup2ofScuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfSCU` reader - Enable Write Group #3 of SCU"]
pub type EnblWrGroup3ofScuR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfSCU` writer - Enable Write Group #3 of SCU"]
pub type EnblWrGroup3ofScuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfSCU` reader - Enable Write Group #4 of SCU"]
pub type EnblWrGroup4ofScuR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfSCU` writer - Enable Write Group #4 of SCU"]
pub type EnblWrGroup4ofScuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfSCU` reader - Enable Write Group #5 of SCU"]
pub type EnblWrGroup5ofScuR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfSCU` writer - Enable Write Group #5 of SCU"]
pub type EnblWrGroup5ofScuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1220PRIC1_220\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1220pric12202116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1220pric12202116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1220pric12202116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1220PRIC12202116` reader - Enable Reset Tolerance of PRIC1220PRIC1_220\\[21:16\\]"]
pub type EnblRstToleranceOfPric1220pric12202116R =
    crate::BitReader<EnblRstToleranceOfPric1220pric12202116>;
impl EnblRstToleranceOfPric1220pric12202116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1220pric12202116 {
        match self.bits {
            false => EnblRstToleranceOfPric1220pric12202116::ResetBySrst,
            true => EnblRstToleranceOfPric1220pric12202116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1220pric12202116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1220pric12202116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1220PRIC12202116` writer - Enable Reset Tolerance of PRIC1220PRIC1_220\\[21:16\\]"]
pub type EnblRstToleranceOfPric1220pric12202116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1220pric12202116>;
impl<'a, REG> EnblRstToleranceOfPric1220pric12202116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1220pric12202116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1220pric12202116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1220PRIC12202216` reader - Enable Write Protection of PRIC1220PRIC1_220\\[22:16\\]"]
pub type EnblWrProtOfPric1220pric12202216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1220PRIC12202216` writer - Enable Write Protection of PRIC1220PRIC1_220\\[22:16\\]"]
pub type EnblWrProtOfPric1220pric12202216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of ADC 1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_adc1(&self) -> EnblWrGroup0ofAdc1R {
        EnblWrGroup0ofAdc1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of ADC 1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_adc1(&self) -> EnblWrGroup1ofAdc1R {
        EnblWrGroup1ofAdc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of ADC 1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_adc1(&self) -> EnblWrGroup2ofAdc1R {
        EnblWrGroup2ofAdc1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of ADC 1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_adc1(&self) -> EnblWrGroup3ofAdc1R {
        EnblWrGroup3ofAdc1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of ADC 1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_adc1(&self) -> EnblWrGroup4ofAdc1R {
        EnblWrGroup4ofAdc1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of ADC 1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_adc1(&self) -> EnblWrGroup5ofAdc1R {
        EnblWrGroup5ofAdc1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1220PRIC1_220\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1220pric12200500(
        &self,
    ) -> EnblRstToleranceOfPric1220pric12200500R {
        EnblRstToleranceOfPric1220pric12200500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1220PRIC1_220\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1220pric12200600(&self) -> EnblWrProtOfPric1220pric12200600R {
        EnblWrProtOfPric1220pric12200600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of JTAG0"]
    #[inline(always)]
    pub fn enbl_wr_group0of_jtag0(&self) -> EnblWrGroup0ofJtag0R {
        EnblWrGroup0ofJtag0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of JTAG0"]
    #[inline(always)]
    pub fn enbl_wr_group1of_jtag0(&self) -> EnblWrGroup1ofJtag0R {
        EnblWrGroup1ofJtag0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of JTAG0"]
    #[inline(always)]
    pub fn enbl_wr_group2of_jtag0(&self) -> EnblWrGroup2ofJtag0R {
        EnblWrGroup2ofJtag0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of JTAG0"]
    #[inline(always)]
    pub fn enbl_wr_group3of_jtag0(&self) -> EnblWrGroup3ofJtag0R {
        EnblWrGroup3ofJtag0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of JTAG0"]
    #[inline(always)]
    pub fn enbl_wr_group4of_jtag0(&self) -> EnblWrGroup4ofJtag0R {
        EnblWrGroup4ofJtag0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of JTAG0"]
    #[inline(always)]
    pub fn enbl_wr_group5of_jtag0(&self) -> EnblWrGroup5ofJtag0R {
        EnblWrGroup5ofJtag0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1220PRIC1_220\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1220pric12201308(
        &self,
    ) -> EnblRstToleranceOfPric1220pric12201308R {
        EnblRstToleranceOfPric1220pric12201308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1220PRIC1_220\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1220pric12201408(&self) -> EnblWrProtOfPric1220pric12201408R {
        EnblWrProtOfPric1220pric12201408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of SCU"]
    #[inline(always)]
    pub fn enbl_wr_group0of_scu(&self) -> EnblWrGroup0ofScuR {
        EnblWrGroup0ofScuR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of SCU"]
    #[inline(always)]
    pub fn enbl_wr_group1of_scu(&self) -> EnblWrGroup1ofScuR {
        EnblWrGroup1ofScuR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of SCU"]
    #[inline(always)]
    pub fn enbl_wr_group2of_scu(&self) -> EnblWrGroup2ofScuR {
        EnblWrGroup2ofScuR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of SCU"]
    #[inline(always)]
    pub fn enbl_wr_group3of_scu(&self) -> EnblWrGroup3ofScuR {
        EnblWrGroup3ofScuR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of SCU"]
    #[inline(always)]
    pub fn enbl_wr_group4of_scu(&self) -> EnblWrGroup4ofScuR {
        EnblWrGroup4ofScuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of SCU"]
    #[inline(always)]
    pub fn enbl_wr_group5of_scu(&self) -> EnblWrGroup5ofScuR {
        EnblWrGroup5ofScuR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1220PRIC1_220\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1220pric12202116(
        &self,
    ) -> EnblRstToleranceOfPric1220pric12202116R {
        EnblRstToleranceOfPric1220pric12202116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1220PRIC1_220\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1220pric12202216(&self) -> EnblWrProtOfPric1220pric12202216R {
        EnblWrProtOfPric1220pric12202216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of ADC 1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_adc1(&mut self) -> EnblWrGroup0ofAdc1W<PricIo220Spec> {
        EnblWrGroup0ofAdc1W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of ADC 1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_adc1(&mut self) -> EnblWrGroup1ofAdc1W<PricIo220Spec> {
        EnblWrGroup1ofAdc1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of ADC 1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_adc1(&mut self) -> EnblWrGroup2ofAdc1W<PricIo220Spec> {
        EnblWrGroup2ofAdc1W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of ADC 1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_adc1(&mut self) -> EnblWrGroup3ofAdc1W<PricIo220Spec> {
        EnblWrGroup3ofAdc1W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of ADC 1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_adc1(&mut self) -> EnblWrGroup4ofAdc1W<PricIo220Spec> {
        EnblWrGroup4ofAdc1W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of ADC 1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_adc1(&mut self) -> EnblWrGroup5ofAdc1W<PricIo220Spec> {
        EnblWrGroup5ofAdc1W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1220PRIC1_220\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1220pric12200500(
        &mut self,
    ) -> EnblRstToleranceOfPric1220pric12200500W<PricIo220Spec> {
        EnblRstToleranceOfPric1220pric12200500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1220PRIC1_220\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1220pric12200600(
        &mut self,
    ) -> EnblWrProtOfPric1220pric12200600W<PricIo220Spec> {
        EnblWrProtOfPric1220pric12200600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of JTAG0"]
    #[inline(always)]
    pub fn enbl_wr_group0of_jtag0(&mut self) -> EnblWrGroup0ofJtag0W<PricIo220Spec> {
        EnblWrGroup0ofJtag0W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of JTAG0"]
    #[inline(always)]
    pub fn enbl_wr_group1of_jtag0(&mut self) -> EnblWrGroup1ofJtag0W<PricIo220Spec> {
        EnblWrGroup1ofJtag0W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of JTAG0"]
    #[inline(always)]
    pub fn enbl_wr_group2of_jtag0(&mut self) -> EnblWrGroup2ofJtag0W<PricIo220Spec> {
        EnblWrGroup2ofJtag0W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of JTAG0"]
    #[inline(always)]
    pub fn enbl_wr_group3of_jtag0(&mut self) -> EnblWrGroup3ofJtag0W<PricIo220Spec> {
        EnblWrGroup3ofJtag0W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of JTAG0"]
    #[inline(always)]
    pub fn enbl_wr_group4of_jtag0(&mut self) -> EnblWrGroup4ofJtag0W<PricIo220Spec> {
        EnblWrGroup4ofJtag0W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of JTAG0"]
    #[inline(always)]
    pub fn enbl_wr_group5of_jtag0(&mut self) -> EnblWrGroup5ofJtag0W<PricIo220Spec> {
        EnblWrGroup5ofJtag0W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1220PRIC1_220\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1220pric12201308(
        &mut self,
    ) -> EnblRstToleranceOfPric1220pric12201308W<PricIo220Spec> {
        EnblRstToleranceOfPric1220pric12201308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1220PRIC1_220\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1220pric12201408(
        &mut self,
    ) -> EnblWrProtOfPric1220pric12201408W<PricIo220Spec> {
        EnblWrProtOfPric1220pric12201408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of SCU"]
    #[inline(always)]
    pub fn enbl_wr_group0of_scu(&mut self) -> EnblWrGroup0ofScuW<PricIo220Spec> {
        EnblWrGroup0ofScuW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of SCU"]
    #[inline(always)]
    pub fn enbl_wr_group1of_scu(&mut self) -> EnblWrGroup1ofScuW<PricIo220Spec> {
        EnblWrGroup1ofScuW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of SCU"]
    #[inline(always)]
    pub fn enbl_wr_group2of_scu(&mut self) -> EnblWrGroup2ofScuW<PricIo220Spec> {
        EnblWrGroup2ofScuW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of SCU"]
    #[inline(always)]
    pub fn enbl_wr_group3of_scu(&mut self) -> EnblWrGroup3ofScuW<PricIo220Spec> {
        EnblWrGroup3ofScuW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of SCU"]
    #[inline(always)]
    pub fn enbl_wr_group4of_scu(&mut self) -> EnblWrGroup4ofScuW<PricIo220Spec> {
        EnblWrGroup4ofScuW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of SCU"]
    #[inline(always)]
    pub fn enbl_wr_group5of_scu(&mut self) -> EnblWrGroup5ofScuW<PricIo220Spec> {
        EnblWrGroup5ofScuW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1220PRIC1_220\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1220pric12202116(
        &mut self,
    ) -> EnblRstToleranceOfPric1220pric12202116W<PricIo220Spec> {
        EnblRstToleranceOfPric1220pric12202116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1220PRIC1_220\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1220pric12202216(
        &mut self,
    ) -> EnblWrProtOfPric1220pric12202216W<PricIo220Spec> {
        EnblWrProtOfPric1220pric12202216W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo220Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Slave Write Group Setting Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io220::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io220::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo220Spec;
impl crate::RegisterSpec for PricIo220Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io220::R`](R) reader structure"]
impl crate::Readable for PricIo220Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io220::W`](W) writer structure"]
impl crate::Writable for PricIo220Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO220 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo220Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
