#[doc = "Register `PRIC_IO320` reader"]
pub type R = crate::R<PricIo320Spec>;
#[doc = "Register `PRIC_IO320` writer"]
pub type W = crate::W<PricIo320Spec>;
#[doc = "Field `EnblReadGroup0OfADC1` reader - Enable Read Group #0 of ADC 1"]
pub type EnblReadGroup0ofAdc1R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfADC1` writer - Enable Read Group #0 of ADC 1"]
pub type EnblReadGroup0ofAdc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfADC1` reader - Enable Read Group #1 of ADC 1"]
pub type EnblReadGroup1ofAdc1R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfADC1` writer - Enable Read Group #1 of ADC 1"]
pub type EnblReadGroup1ofAdc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfADC1` reader - Enable Read Group #2 of ADC 1"]
pub type EnblReadGroup2ofAdc1R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfADC1` writer - Enable Read Group #2 of ADC 1"]
pub type EnblReadGroup2ofAdc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfADC1` reader - Enable Read Group #3 of ADC 1"]
pub type EnblReadGroup3ofAdc1R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfADC1` writer - Enable Read Group #3 of ADC 1"]
pub type EnblReadGroup3ofAdc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfADC1` reader - Enable Read Group #4 of ADC 1"]
pub type EnblReadGroup4ofAdc1R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfADC1` writer - Enable Read Group #4 of ADC 1"]
pub type EnblReadGroup4ofAdc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfADC1` reader - Enable Read Group #5 of ADC 1"]
pub type EnblReadGroup5ofAdc1R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfADC1` writer - Enable Read Group #5 of ADC 1"]
pub type EnblReadGroup5ofAdc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1320PRIC1_320\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1320pric13200500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1320pric13200500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1320pric13200500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1320PRIC13200500` reader - Enable Reset Tolerance of PRIC1320PRIC1_320\\[05:00\\]"]
pub type EnblRstToleranceOfPric1320pric13200500R =
    crate::BitReader<EnblRstToleranceOfPric1320pric13200500>;
impl EnblRstToleranceOfPric1320pric13200500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1320pric13200500 {
        match self.bits {
            false => EnblRstToleranceOfPric1320pric13200500::ResetBySrst,
            true => EnblRstToleranceOfPric1320pric13200500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1320pric13200500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1320pric13200500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1320PRIC13200500` writer - Enable Reset Tolerance of PRIC1320PRIC1_320\\[05:00\\]"]
pub type EnblRstToleranceOfPric1320pric13200500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1320pric13200500>;
impl<'a, REG> EnblRstToleranceOfPric1320pric13200500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1320pric13200500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1320pric13200500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1320PRIC13200600` reader - Enable Write Protection of PRIC1320PRIC1_320\\[06:00\\]"]
pub type EnblWrProtOfPric1320pric13200600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1320PRIC13200600` writer - Enable Write Protection of PRIC1320PRIC1_320\\[06:00\\]"]
pub type EnblWrProtOfPric1320pric13200600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfJTAG0` reader - Enable Read Group #0 of JTAG0"]
pub type EnblReadGroup0ofJtag0R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfJTAG0` writer - Enable Read Group #0 of JTAG0"]
pub type EnblReadGroup0ofJtag0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfJTAG0` reader - Enable Read Group #1 of JTAG0"]
pub type EnblReadGroup1ofJtag0R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfJTAG0` writer - Enable Read Group #1 of JTAG0"]
pub type EnblReadGroup1ofJtag0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfJTAG0` reader - Enable Read Group #2 of JTAG0"]
pub type EnblReadGroup2ofJtag0R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfJTAG0` writer - Enable Read Group #2 of JTAG0"]
pub type EnblReadGroup2ofJtag0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfJTAG0` reader - Enable Read Group #3 of JTAG0"]
pub type EnblReadGroup3ofJtag0R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfJTAG0` writer - Enable Read Group #3 of JTAG0"]
pub type EnblReadGroup3ofJtag0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfJTAG0` reader - Enable Read Group #4 of JTAG0"]
pub type EnblReadGroup4ofJtag0R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfJTAG0` writer - Enable Read Group #4 of JTAG0"]
pub type EnblReadGroup4ofJtag0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfJTAG0` reader - Enable Read Group #5 of JTAG0"]
pub type EnblReadGroup5ofJtag0R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfJTAG0` writer - Enable Read Group #5 of JTAG0"]
pub type EnblReadGroup5ofJtag0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1320PRIC1_320\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1320pric13201308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1320pric13201308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1320pric13201308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1320PRIC13201308` reader - Enable Reset Tolerance of PRIC1320PRIC1_320\\[13:08\\]"]
pub type EnblRstToleranceOfPric1320pric13201308R =
    crate::BitReader<EnblRstToleranceOfPric1320pric13201308>;
impl EnblRstToleranceOfPric1320pric13201308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1320pric13201308 {
        match self.bits {
            false => EnblRstToleranceOfPric1320pric13201308::ResetBySrst,
            true => EnblRstToleranceOfPric1320pric13201308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1320pric13201308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1320pric13201308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1320PRIC13201308` writer - Enable Reset Tolerance of PRIC1320PRIC1_320\\[13:08\\]"]
pub type EnblRstToleranceOfPric1320pric13201308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1320pric13201308>;
impl<'a, REG> EnblRstToleranceOfPric1320pric13201308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1320pric13201308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1320pric13201308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1320PRIC13201408` reader - Enable Write Protection of PRIC1320PRIC1_320\\[14:08\\]"]
pub type EnblWrProtOfPric1320pric13201408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1320PRIC13201408` writer - Enable Write Protection of PRIC1320PRIC1_320\\[14:08\\]"]
pub type EnblWrProtOfPric1320pric13201408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfSCU` reader - Enable Read Group #0 of SCU"]
pub type EnblReadGroup0ofScuR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfSCU` writer - Enable Read Group #0 of SCU"]
pub type EnblReadGroup0ofScuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfSCU` reader - Enable Read Group #1 of SCU"]
pub type EnblReadGroup1ofScuR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfSCU` writer - Enable Read Group #1 of SCU"]
pub type EnblReadGroup1ofScuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfSCU` reader - Enable Read Group #2 of SCU"]
pub type EnblReadGroup2ofScuR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfSCU` writer - Enable Read Group #2 of SCU"]
pub type EnblReadGroup2ofScuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfSCU` reader - Enable Read Group #3 of SCU"]
pub type EnblReadGroup3ofScuR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfSCU` writer - Enable Read Group #3 of SCU"]
pub type EnblReadGroup3ofScuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfSCU` reader - Enable Read Group #4 of SCU"]
pub type EnblReadGroup4ofScuR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfSCU` writer - Enable Read Group #4 of SCU"]
pub type EnblReadGroup4ofScuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfSCU` reader - Enable Read Group #5 of SCU"]
pub type EnblReadGroup5ofScuR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfSCU` writer - Enable Read Group #5 of SCU"]
pub type EnblReadGroup5ofScuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1320PRIC1_320\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1320pric13202116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1320pric13202116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1320pric13202116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1320PRIC13202116` reader - Enable Reset Tolerance of PRIC1320PRIC1_320\\[21:16\\]"]
pub type EnblRstToleranceOfPric1320pric13202116R =
    crate::BitReader<EnblRstToleranceOfPric1320pric13202116>;
impl EnblRstToleranceOfPric1320pric13202116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1320pric13202116 {
        match self.bits {
            false => EnblRstToleranceOfPric1320pric13202116::ResetBySrst,
            true => EnblRstToleranceOfPric1320pric13202116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1320pric13202116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1320pric13202116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1320PRIC13202116` writer - Enable Reset Tolerance of PRIC1320PRIC1_320\\[21:16\\]"]
pub type EnblRstToleranceOfPric1320pric13202116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1320pric13202116>;
impl<'a, REG> EnblRstToleranceOfPric1320pric13202116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1320pric13202116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1320pric13202116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1320PRIC13202216` reader - Enable Write Protection of PRIC1320PRIC1_320\\[22:16\\]"]
pub type EnblWrProtOfPric1320pric13202216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1320PRIC13202216` writer - Enable Write Protection of PRIC1320PRIC1_320\\[22:16\\]"]
pub type EnblWrProtOfPric1320pric13202216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of ADC 1"]
    #[inline(always)]
    pub fn enbl_read_group0of_adc1(&self) -> EnblReadGroup0ofAdc1R {
        EnblReadGroup0ofAdc1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of ADC 1"]
    #[inline(always)]
    pub fn enbl_read_group1of_adc1(&self) -> EnblReadGroup1ofAdc1R {
        EnblReadGroup1ofAdc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of ADC 1"]
    #[inline(always)]
    pub fn enbl_read_group2of_adc1(&self) -> EnblReadGroup2ofAdc1R {
        EnblReadGroup2ofAdc1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of ADC 1"]
    #[inline(always)]
    pub fn enbl_read_group3of_adc1(&self) -> EnblReadGroup3ofAdc1R {
        EnblReadGroup3ofAdc1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of ADC 1"]
    #[inline(always)]
    pub fn enbl_read_group4of_adc1(&self) -> EnblReadGroup4ofAdc1R {
        EnblReadGroup4ofAdc1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of ADC 1"]
    #[inline(always)]
    pub fn enbl_read_group5of_adc1(&self) -> EnblReadGroup5ofAdc1R {
        EnblReadGroup5ofAdc1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1320PRIC1_320\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1320pric13200500(
        &self,
    ) -> EnblRstToleranceOfPric1320pric13200500R {
        EnblRstToleranceOfPric1320pric13200500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1320PRIC1_320\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1320pric13200600(&self) -> EnblWrProtOfPric1320pric13200600R {
        EnblWrProtOfPric1320pric13200600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of JTAG0"]
    #[inline(always)]
    pub fn enbl_read_group0of_jtag0(&self) -> EnblReadGroup0ofJtag0R {
        EnblReadGroup0ofJtag0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of JTAG0"]
    #[inline(always)]
    pub fn enbl_read_group1of_jtag0(&self) -> EnblReadGroup1ofJtag0R {
        EnblReadGroup1ofJtag0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of JTAG0"]
    #[inline(always)]
    pub fn enbl_read_group2of_jtag0(&self) -> EnblReadGroup2ofJtag0R {
        EnblReadGroup2ofJtag0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of JTAG0"]
    #[inline(always)]
    pub fn enbl_read_group3of_jtag0(&self) -> EnblReadGroup3ofJtag0R {
        EnblReadGroup3ofJtag0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of JTAG0"]
    #[inline(always)]
    pub fn enbl_read_group4of_jtag0(&self) -> EnblReadGroup4ofJtag0R {
        EnblReadGroup4ofJtag0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of JTAG0"]
    #[inline(always)]
    pub fn enbl_read_group5of_jtag0(&self) -> EnblReadGroup5ofJtag0R {
        EnblReadGroup5ofJtag0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1320PRIC1_320\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1320pric13201308(
        &self,
    ) -> EnblRstToleranceOfPric1320pric13201308R {
        EnblRstToleranceOfPric1320pric13201308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1320PRIC1_320\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1320pric13201408(&self) -> EnblWrProtOfPric1320pric13201408R {
        EnblWrProtOfPric1320pric13201408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of SCU"]
    #[inline(always)]
    pub fn enbl_read_group0of_scu(&self) -> EnblReadGroup0ofScuR {
        EnblReadGroup0ofScuR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of SCU"]
    #[inline(always)]
    pub fn enbl_read_group1of_scu(&self) -> EnblReadGroup1ofScuR {
        EnblReadGroup1ofScuR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of SCU"]
    #[inline(always)]
    pub fn enbl_read_group2of_scu(&self) -> EnblReadGroup2ofScuR {
        EnblReadGroup2ofScuR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of SCU"]
    #[inline(always)]
    pub fn enbl_read_group3of_scu(&self) -> EnblReadGroup3ofScuR {
        EnblReadGroup3ofScuR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of SCU"]
    #[inline(always)]
    pub fn enbl_read_group4of_scu(&self) -> EnblReadGroup4ofScuR {
        EnblReadGroup4ofScuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of SCU"]
    #[inline(always)]
    pub fn enbl_read_group5of_scu(&self) -> EnblReadGroup5ofScuR {
        EnblReadGroup5ofScuR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1320PRIC1_320\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1320pric13202116(
        &self,
    ) -> EnblRstToleranceOfPric1320pric13202116R {
        EnblRstToleranceOfPric1320pric13202116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1320PRIC1_320\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1320pric13202216(&self) -> EnblWrProtOfPric1320pric13202216R {
        EnblWrProtOfPric1320pric13202216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of ADC 1"]
    #[inline(always)]
    pub fn enbl_read_group0of_adc1(&mut self) -> EnblReadGroup0ofAdc1W<PricIo320Spec> {
        EnblReadGroup0ofAdc1W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of ADC 1"]
    #[inline(always)]
    pub fn enbl_read_group1of_adc1(&mut self) -> EnblReadGroup1ofAdc1W<PricIo320Spec> {
        EnblReadGroup1ofAdc1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of ADC 1"]
    #[inline(always)]
    pub fn enbl_read_group2of_adc1(&mut self) -> EnblReadGroup2ofAdc1W<PricIo320Spec> {
        EnblReadGroup2ofAdc1W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of ADC 1"]
    #[inline(always)]
    pub fn enbl_read_group3of_adc1(&mut self) -> EnblReadGroup3ofAdc1W<PricIo320Spec> {
        EnblReadGroup3ofAdc1W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of ADC 1"]
    #[inline(always)]
    pub fn enbl_read_group4of_adc1(&mut self) -> EnblReadGroup4ofAdc1W<PricIo320Spec> {
        EnblReadGroup4ofAdc1W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of ADC 1"]
    #[inline(always)]
    pub fn enbl_read_group5of_adc1(&mut self) -> EnblReadGroup5ofAdc1W<PricIo320Spec> {
        EnblReadGroup5ofAdc1W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1320PRIC1_320\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1320pric13200500(
        &mut self,
    ) -> EnblRstToleranceOfPric1320pric13200500W<PricIo320Spec> {
        EnblRstToleranceOfPric1320pric13200500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1320PRIC1_320\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1320pric13200600(
        &mut self,
    ) -> EnblWrProtOfPric1320pric13200600W<PricIo320Spec> {
        EnblWrProtOfPric1320pric13200600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of JTAG0"]
    #[inline(always)]
    pub fn enbl_read_group0of_jtag0(&mut self) -> EnblReadGroup0ofJtag0W<PricIo320Spec> {
        EnblReadGroup0ofJtag0W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of JTAG0"]
    #[inline(always)]
    pub fn enbl_read_group1of_jtag0(&mut self) -> EnblReadGroup1ofJtag0W<PricIo320Spec> {
        EnblReadGroup1ofJtag0W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of JTAG0"]
    #[inline(always)]
    pub fn enbl_read_group2of_jtag0(&mut self) -> EnblReadGroup2ofJtag0W<PricIo320Spec> {
        EnblReadGroup2ofJtag0W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of JTAG0"]
    #[inline(always)]
    pub fn enbl_read_group3of_jtag0(&mut self) -> EnblReadGroup3ofJtag0W<PricIo320Spec> {
        EnblReadGroup3ofJtag0W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of JTAG0"]
    #[inline(always)]
    pub fn enbl_read_group4of_jtag0(&mut self) -> EnblReadGroup4ofJtag0W<PricIo320Spec> {
        EnblReadGroup4ofJtag0W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of JTAG0"]
    #[inline(always)]
    pub fn enbl_read_group5of_jtag0(&mut self) -> EnblReadGroup5ofJtag0W<PricIo320Spec> {
        EnblReadGroup5ofJtag0W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1320PRIC1_320\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1320pric13201308(
        &mut self,
    ) -> EnblRstToleranceOfPric1320pric13201308W<PricIo320Spec> {
        EnblRstToleranceOfPric1320pric13201308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1320PRIC1_320\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1320pric13201408(
        &mut self,
    ) -> EnblWrProtOfPric1320pric13201408W<PricIo320Spec> {
        EnblWrProtOfPric1320pric13201408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of SCU"]
    #[inline(always)]
    pub fn enbl_read_group0of_scu(&mut self) -> EnblReadGroup0ofScuW<PricIo320Spec> {
        EnblReadGroup0ofScuW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of SCU"]
    #[inline(always)]
    pub fn enbl_read_group1of_scu(&mut self) -> EnblReadGroup1ofScuW<PricIo320Spec> {
        EnblReadGroup1ofScuW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of SCU"]
    #[inline(always)]
    pub fn enbl_read_group2of_scu(&mut self) -> EnblReadGroup2ofScuW<PricIo320Spec> {
        EnblReadGroup2ofScuW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of SCU"]
    #[inline(always)]
    pub fn enbl_read_group3of_scu(&mut self) -> EnblReadGroup3ofScuW<PricIo320Spec> {
        EnblReadGroup3ofScuW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of SCU"]
    #[inline(always)]
    pub fn enbl_read_group4of_scu(&mut self) -> EnblReadGroup4ofScuW<PricIo320Spec> {
        EnblReadGroup4ofScuW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of SCU"]
    #[inline(always)]
    pub fn enbl_read_group5of_scu(&mut self) -> EnblReadGroup5ofScuW<PricIo320Spec> {
        EnblReadGroup5ofScuW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1320PRIC1_320\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1320pric13202116(
        &mut self,
    ) -> EnblRstToleranceOfPric1320pric13202116W<PricIo320Spec> {
        EnblRstToleranceOfPric1320pric13202116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1320PRIC1_320\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1320pric13202216(
        &mut self,
    ) -> EnblWrProtOfPric1320pric13202216W<PricIo320Spec> {
        EnblWrProtOfPric1320pric13202216W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo320Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Slave Read Group Setting Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io320::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io320::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo320Spec;
impl crate::RegisterSpec for PricIo320Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io320::R`](R) reader structure"]
impl crate::Readable for PricIo320Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io320::W`](W) writer structure"]
impl crate::Writable for PricIo320Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO320 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo320Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
