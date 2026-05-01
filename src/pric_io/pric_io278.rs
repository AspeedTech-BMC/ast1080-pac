#[doc = "Register `PRIC_IO278` reader"]
pub type R = crate::R<PricIo278Spec>;
#[doc = "Register `PRIC_IO278` writer"]
pub type W = crate::W<PricIo278Spec>;
#[doc = "Field `EnblWrGroup0OfUART11` reader - Enable Write Group #0 of UART11"]
pub type EnblWrGroup0ofUart11R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUART11` writer - Enable Write Group #0 of UART11"]
pub type EnblWrGroup0ofUart11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUART11` reader - Enable Write Group #1 of UART11"]
pub type EnblWrGroup1ofUart11R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUART11` writer - Enable Write Group #1 of UART11"]
pub type EnblWrGroup1ofUart11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUART11` reader - Enable Write Group #2 of UART11"]
pub type EnblWrGroup2ofUart11R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUART11` writer - Enable Write Group #2 of UART11"]
pub type EnblWrGroup2ofUart11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUART11` reader - Enable Write Group #3 of UART11"]
pub type EnblWrGroup3ofUart11R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUART11` writer - Enable Write Group #3 of UART11"]
pub type EnblWrGroup3ofUart11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUART11` reader - Enable Write Group #4 of UART11"]
pub type EnblWrGroup4ofUart11R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUART11` writer - Enable Write Group #4 of UART11"]
pub type EnblWrGroup4ofUart11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUART11` reader - Enable Write Group #5 of UART11"]
pub type EnblWrGroup5ofUart11R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUART11` writer - Enable Write Group #5 of UART11"]
pub type EnblWrGroup5ofUart11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1278PRIC1_278\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1278pric12780500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1278pric12780500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1278pric12780500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1278PRIC12780500` reader - Enable Reset Tolerance of PRIC1278PRIC1_278\\[05:00\\]"]
pub type EnblRstToleranceOfPric1278pric12780500R =
    crate::BitReader<EnblRstToleranceOfPric1278pric12780500>;
impl EnblRstToleranceOfPric1278pric12780500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1278pric12780500 {
        match self.bits {
            false => EnblRstToleranceOfPric1278pric12780500::ResetBySrst,
            true => EnblRstToleranceOfPric1278pric12780500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1278pric12780500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1278pric12780500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1278PRIC12780500` writer - Enable Reset Tolerance of PRIC1278PRIC1_278\\[05:00\\]"]
pub type EnblRstToleranceOfPric1278pric12780500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1278pric12780500>;
impl<'a, REG> EnblRstToleranceOfPric1278pric12780500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1278pric12780500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1278pric12780500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1278PRIC12780600` reader - Enable Write Protection of PRIC1278PRIC1_278\\[06:00\\]"]
pub type EnblWrProtOfPric1278pric12780600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1278PRIC12780600` writer - Enable Write Protection of PRIC1278PRIC1_278\\[06:00\\]"]
pub type EnblWrProtOfPric1278pric12780600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfBMCUART` reader - Enable Write Group #0 of BMC UART"]
pub type EnblWrGroup0ofBmcuartR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfBMCUART` writer - Enable Write Group #0 of BMC UART"]
pub type EnblWrGroup0ofBmcuartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfBMCUART` reader - Enable Write Group #1 of BMC UART"]
pub type EnblWrGroup1ofBmcuartR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfBMCUART` writer - Enable Write Group #1 of BMC UART"]
pub type EnblWrGroup1ofBmcuartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfBMCUART` reader - Enable Write Group #2 of BMC UART"]
pub type EnblWrGroup2ofBmcuartR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfBMCUART` writer - Enable Write Group #2 of BMC UART"]
pub type EnblWrGroup2ofBmcuartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfBMCUART` reader - Enable Write Group #3 of BMC UART"]
pub type EnblWrGroup3ofBmcuartR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfBMCUART` writer - Enable Write Group #3 of BMC UART"]
pub type EnblWrGroup3ofBmcuartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfBMCUART` reader - Enable Write Group #4 of BMC UART"]
pub type EnblWrGroup4ofBmcuartR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfBMCUART` writer - Enable Write Group #4 of BMC UART"]
pub type EnblWrGroup4ofBmcuartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfBMCUART` reader - Enable Write Group #5 of BMC UART"]
pub type EnblWrGroup5ofBmcuartR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfBMCUART` writer - Enable Write Group #5 of BMC UART"]
pub type EnblWrGroup5ofBmcuartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1278PRIC1_278\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1278pric12781308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1278pric12781308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1278pric12781308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1278PRIC12781308` reader - Enable Reset Tolerance of PRIC1278PRIC1_278\\[13:08\\]"]
pub type EnblRstToleranceOfPric1278pric12781308R =
    crate::BitReader<EnblRstToleranceOfPric1278pric12781308>;
impl EnblRstToleranceOfPric1278pric12781308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1278pric12781308 {
        match self.bits {
            false => EnblRstToleranceOfPric1278pric12781308::ResetBySrst,
            true => EnblRstToleranceOfPric1278pric12781308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1278pric12781308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1278pric12781308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1278PRIC12781308` writer - Enable Reset Tolerance of PRIC1278PRIC1_278\\[13:08\\]"]
pub type EnblRstToleranceOfPric1278pric12781308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1278pric12781308>;
impl<'a, REG> EnblRstToleranceOfPric1278pric12781308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1278pric12781308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1278pric12781308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1278PRIC12781408` reader - Enable Write Protection of PRIC1278PRIC1_278\\[14:08\\]"]
pub type EnblWrProtOfPric1278pric12781408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1278PRIC12781408` writer - Enable Write Protection of PRIC1278PRIC1_278\\[14:08\\]"]
pub type EnblWrProtOfPric1278pric12781408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfRTC` reader - Enable Write Group #0 of RTC"]
pub type EnblWrGroup0ofRtcR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfRTC` writer - Enable Write Group #0 of RTC"]
pub type EnblWrGroup0ofRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfRTC` reader - Enable Write Group #1 of RTC"]
pub type EnblWrGroup1ofRtcR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfRTC` writer - Enable Write Group #1 of RTC"]
pub type EnblWrGroup1ofRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfRTC` reader - Enable Write Group #2 of RTC"]
pub type EnblWrGroup2ofRtcR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfRTC` writer - Enable Write Group #2 of RTC"]
pub type EnblWrGroup2ofRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfRTC` reader - Enable Write Group #3 of RTC"]
pub type EnblWrGroup3ofRtcR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfRTC` writer - Enable Write Group #3 of RTC"]
pub type EnblWrGroup3ofRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfRTC` reader - Enable Write Group #4 of RTC"]
pub type EnblWrGroup4ofRtcR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfRTC` writer - Enable Write Group #4 of RTC"]
pub type EnblWrGroup4ofRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfRTC` reader - Enable Write Group #5 of RTC"]
pub type EnblWrGroup5ofRtcR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfRTC` writer - Enable Write Group #5 of RTC"]
pub type EnblWrGroup5ofRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1278PRIC1_278\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1278pric12782116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1278pric12782116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1278pric12782116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1278PRIC12782116` reader - Enable Reset Tolerance of PRIC1278PRIC1_278\\[21:16\\]"]
pub type EnblRstToleranceOfPric1278pric12782116R =
    crate::BitReader<EnblRstToleranceOfPric1278pric12782116>;
impl EnblRstToleranceOfPric1278pric12782116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1278pric12782116 {
        match self.bits {
            false => EnblRstToleranceOfPric1278pric12782116::ResetBySrst,
            true => EnblRstToleranceOfPric1278pric12782116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1278pric12782116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1278pric12782116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1278PRIC12782116` writer - Enable Reset Tolerance of PRIC1278PRIC1_278\\[21:16\\]"]
pub type EnblRstToleranceOfPric1278pric12782116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1278pric12782116>;
impl<'a, REG> EnblRstToleranceOfPric1278pric12782116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1278pric12782116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1278pric12782116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1278PRIC12782216` reader - Enable Write Protection of PRIC1278PRIC1_278\\[22:16\\]"]
pub type EnblWrProtOfPric1278pric12782216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1278PRIC12782216` writer - Enable Write Protection of PRIC1278PRIC1_278\\[22:16\\]"]
pub type EnblWrProtOfPric1278pric12782216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfAntiTamper` reader - Enable Write Group #0 of Anti Tamper"]
pub type EnblWrGroup0ofAntiTamperR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfAntiTamper` writer - Enable Write Group #0 of Anti Tamper"]
pub type EnblWrGroup0ofAntiTamperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfAntiTamper` reader - Enable Write Group #1 of Anti Tamper"]
pub type EnblWrGroup1ofAntiTamperR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfAntiTamper` writer - Enable Write Group #1 of Anti Tamper"]
pub type EnblWrGroup1ofAntiTamperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfAntiTamper` reader - Enable Write Group #2 of Anti Tamper"]
pub type EnblWrGroup2ofAntiTamperR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfAntiTamper` writer - Enable Write Group #2 of Anti Tamper"]
pub type EnblWrGroup2ofAntiTamperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfAntiTamper` reader - Enable Write Group #3 of Anti Tamper"]
pub type EnblWrGroup3ofAntiTamperR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfAntiTamper` writer - Enable Write Group #3 of Anti Tamper"]
pub type EnblWrGroup3ofAntiTamperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfAntiTamper` reader - Enable Write Group #4 of Anti Tamper"]
pub type EnblWrGroup4ofAntiTamperR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfAntiTamper` writer - Enable Write Group #4 of Anti Tamper"]
pub type EnblWrGroup4ofAntiTamperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfAntiTamper` reader - Enable Write Group #5 of Anti Tamper"]
pub type EnblWrGroup5ofAntiTamperR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfAntiTamper` writer - Enable Write Group #5 of Anti Tamper"]
pub type EnblWrGroup5ofAntiTamperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1278PRIC1_278\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1278pric12782924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1278pric12782924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1278pric12782924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1278PRIC12782924` reader - Enable Reset Tolerance of PRIC1278PRIC1_278\\[29:24\\]"]
pub type EnblRstToleranceOfPric1278pric12782924R =
    crate::BitReader<EnblRstToleranceOfPric1278pric12782924>;
impl EnblRstToleranceOfPric1278pric12782924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1278pric12782924 {
        match self.bits {
            false => EnblRstToleranceOfPric1278pric12782924::ResetBySrst,
            true => EnblRstToleranceOfPric1278pric12782924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1278pric12782924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1278pric12782924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1278PRIC12782924` writer - Enable Reset Tolerance of PRIC1278PRIC1_278\\[29:24\\]"]
pub type EnblRstToleranceOfPric1278pric12782924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1278pric12782924>;
impl<'a, REG> EnblRstToleranceOfPric1278pric12782924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1278pric12782924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1278pric12782924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1278PRIC12783024` reader - Enable Write Protection of PRIC1278PRIC1_278\\[30:24\\]"]
pub type EnblWrProtOfPric1278pric12783024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1278PRIC12783024` writer - Enable Write Protection of PRIC1278PRIC1_278\\[30:24\\]"]
pub type EnblWrProtOfPric1278pric12783024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of UART11"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart11(&self) -> EnblWrGroup0ofUart11R {
        EnblWrGroup0ofUart11R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of UART11"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart11(&self) -> EnblWrGroup1ofUart11R {
        EnblWrGroup1ofUart11R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of UART11"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart11(&self) -> EnblWrGroup2ofUart11R {
        EnblWrGroup2ofUart11R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of UART11"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart11(&self) -> EnblWrGroup3ofUart11R {
        EnblWrGroup3ofUart11R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of UART11"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart11(&self) -> EnblWrGroup4ofUart11R {
        EnblWrGroup4ofUart11R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of UART11"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart11(&self) -> EnblWrGroup5ofUart11R {
        EnblWrGroup5ofUart11R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1278PRIC1_278\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1278pric12780500(
        &self,
    ) -> EnblRstToleranceOfPric1278pric12780500R {
        EnblRstToleranceOfPric1278pric12780500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1278PRIC1_278\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1278pric12780600(&self) -> EnblWrProtOfPric1278pric12780600R {
        EnblWrProtOfPric1278pric12780600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of BMC UART"]
    #[inline(always)]
    pub fn enbl_wr_group0of_bmcuart(&self) -> EnblWrGroup0ofBmcuartR {
        EnblWrGroup0ofBmcuartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of BMC UART"]
    #[inline(always)]
    pub fn enbl_wr_group1of_bmcuart(&self) -> EnblWrGroup1ofBmcuartR {
        EnblWrGroup1ofBmcuartR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of BMC UART"]
    #[inline(always)]
    pub fn enbl_wr_group2of_bmcuart(&self) -> EnblWrGroup2ofBmcuartR {
        EnblWrGroup2ofBmcuartR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of BMC UART"]
    #[inline(always)]
    pub fn enbl_wr_group3of_bmcuart(&self) -> EnblWrGroup3ofBmcuartR {
        EnblWrGroup3ofBmcuartR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of BMC UART"]
    #[inline(always)]
    pub fn enbl_wr_group4of_bmcuart(&self) -> EnblWrGroup4ofBmcuartR {
        EnblWrGroup4ofBmcuartR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of BMC UART"]
    #[inline(always)]
    pub fn enbl_wr_group5of_bmcuart(&self) -> EnblWrGroup5ofBmcuartR {
        EnblWrGroup5ofBmcuartR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1278PRIC1_278\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1278pric12781308(
        &self,
    ) -> EnblRstToleranceOfPric1278pric12781308R {
        EnblRstToleranceOfPric1278pric12781308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1278PRIC1_278\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1278pric12781408(&self) -> EnblWrProtOfPric1278pric12781408R {
        EnblWrProtOfPric1278pric12781408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of RTC"]
    #[inline(always)]
    pub fn enbl_wr_group0of_rtc(&self) -> EnblWrGroup0ofRtcR {
        EnblWrGroup0ofRtcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of RTC"]
    #[inline(always)]
    pub fn enbl_wr_group1of_rtc(&self) -> EnblWrGroup1ofRtcR {
        EnblWrGroup1ofRtcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of RTC"]
    #[inline(always)]
    pub fn enbl_wr_group2of_rtc(&self) -> EnblWrGroup2ofRtcR {
        EnblWrGroup2ofRtcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of RTC"]
    #[inline(always)]
    pub fn enbl_wr_group3of_rtc(&self) -> EnblWrGroup3ofRtcR {
        EnblWrGroup3ofRtcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of RTC"]
    #[inline(always)]
    pub fn enbl_wr_group4of_rtc(&self) -> EnblWrGroup4ofRtcR {
        EnblWrGroup4ofRtcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of RTC"]
    #[inline(always)]
    pub fn enbl_wr_group5of_rtc(&self) -> EnblWrGroup5ofRtcR {
        EnblWrGroup5ofRtcR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1278PRIC1_278\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1278pric12782116(
        &self,
    ) -> EnblRstToleranceOfPric1278pric12782116R {
        EnblRstToleranceOfPric1278pric12782116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1278PRIC1_278\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1278pric12782216(&self) -> EnblWrProtOfPric1278pric12782216R {
        EnblWrProtOfPric1278pric12782216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_wr_group0of_anti_tamper(&self) -> EnblWrGroup0ofAntiTamperR {
        EnblWrGroup0ofAntiTamperR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_wr_group1of_anti_tamper(&self) -> EnblWrGroup1ofAntiTamperR {
        EnblWrGroup1ofAntiTamperR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_wr_group2of_anti_tamper(&self) -> EnblWrGroup2ofAntiTamperR {
        EnblWrGroup2ofAntiTamperR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_wr_group3of_anti_tamper(&self) -> EnblWrGroup3ofAntiTamperR {
        EnblWrGroup3ofAntiTamperR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_wr_group4of_anti_tamper(&self) -> EnblWrGroup4ofAntiTamperR {
        EnblWrGroup4ofAntiTamperR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_wr_group5of_anti_tamper(&self) -> EnblWrGroup5ofAntiTamperR {
        EnblWrGroup5ofAntiTamperR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1278PRIC1_278\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1278pric12782924(
        &self,
    ) -> EnblRstToleranceOfPric1278pric12782924R {
        EnblRstToleranceOfPric1278pric12782924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1278PRIC1_278\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1278pric12783024(&self) -> EnblWrProtOfPric1278pric12783024R {
        EnblWrProtOfPric1278pric12783024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of UART11"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart11(&mut self) -> EnblWrGroup0ofUart11W<PricIo278Spec> {
        EnblWrGroup0ofUart11W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of UART11"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart11(&mut self) -> EnblWrGroup1ofUart11W<PricIo278Spec> {
        EnblWrGroup1ofUart11W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of UART11"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart11(&mut self) -> EnblWrGroup2ofUart11W<PricIo278Spec> {
        EnblWrGroup2ofUart11W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of UART11"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart11(&mut self) -> EnblWrGroup3ofUart11W<PricIo278Spec> {
        EnblWrGroup3ofUart11W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of UART11"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart11(&mut self) -> EnblWrGroup4ofUart11W<PricIo278Spec> {
        EnblWrGroup4ofUart11W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of UART11"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart11(&mut self) -> EnblWrGroup5ofUart11W<PricIo278Spec> {
        EnblWrGroup5ofUart11W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1278PRIC1_278\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1278pric12780500(
        &mut self,
    ) -> EnblRstToleranceOfPric1278pric12780500W<PricIo278Spec> {
        EnblRstToleranceOfPric1278pric12780500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1278PRIC1_278\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1278pric12780600(
        &mut self,
    ) -> EnblWrProtOfPric1278pric12780600W<PricIo278Spec> {
        EnblWrProtOfPric1278pric12780600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of BMC UART"]
    #[inline(always)]
    pub fn enbl_wr_group0of_bmcuart(&mut self) -> EnblWrGroup0ofBmcuartW<PricIo278Spec> {
        EnblWrGroup0ofBmcuartW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of BMC UART"]
    #[inline(always)]
    pub fn enbl_wr_group1of_bmcuart(&mut self) -> EnblWrGroup1ofBmcuartW<PricIo278Spec> {
        EnblWrGroup1ofBmcuartW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of BMC UART"]
    #[inline(always)]
    pub fn enbl_wr_group2of_bmcuart(&mut self) -> EnblWrGroup2ofBmcuartW<PricIo278Spec> {
        EnblWrGroup2ofBmcuartW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of BMC UART"]
    #[inline(always)]
    pub fn enbl_wr_group3of_bmcuart(&mut self) -> EnblWrGroup3ofBmcuartW<PricIo278Spec> {
        EnblWrGroup3ofBmcuartW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of BMC UART"]
    #[inline(always)]
    pub fn enbl_wr_group4of_bmcuart(&mut self) -> EnblWrGroup4ofBmcuartW<PricIo278Spec> {
        EnblWrGroup4ofBmcuartW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of BMC UART"]
    #[inline(always)]
    pub fn enbl_wr_group5of_bmcuart(&mut self) -> EnblWrGroup5ofBmcuartW<PricIo278Spec> {
        EnblWrGroup5ofBmcuartW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1278PRIC1_278\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1278pric12781308(
        &mut self,
    ) -> EnblRstToleranceOfPric1278pric12781308W<PricIo278Spec> {
        EnblRstToleranceOfPric1278pric12781308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1278PRIC1_278\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1278pric12781408(
        &mut self,
    ) -> EnblWrProtOfPric1278pric12781408W<PricIo278Spec> {
        EnblWrProtOfPric1278pric12781408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of RTC"]
    #[inline(always)]
    pub fn enbl_wr_group0of_rtc(&mut self) -> EnblWrGroup0ofRtcW<PricIo278Spec> {
        EnblWrGroup0ofRtcW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of RTC"]
    #[inline(always)]
    pub fn enbl_wr_group1of_rtc(&mut self) -> EnblWrGroup1ofRtcW<PricIo278Spec> {
        EnblWrGroup1ofRtcW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of RTC"]
    #[inline(always)]
    pub fn enbl_wr_group2of_rtc(&mut self) -> EnblWrGroup2ofRtcW<PricIo278Spec> {
        EnblWrGroup2ofRtcW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of RTC"]
    #[inline(always)]
    pub fn enbl_wr_group3of_rtc(&mut self) -> EnblWrGroup3ofRtcW<PricIo278Spec> {
        EnblWrGroup3ofRtcW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of RTC"]
    #[inline(always)]
    pub fn enbl_wr_group4of_rtc(&mut self) -> EnblWrGroup4ofRtcW<PricIo278Spec> {
        EnblWrGroup4ofRtcW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of RTC"]
    #[inline(always)]
    pub fn enbl_wr_group5of_rtc(&mut self) -> EnblWrGroup5ofRtcW<PricIo278Spec> {
        EnblWrGroup5ofRtcW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1278PRIC1_278\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1278pric12782116(
        &mut self,
    ) -> EnblRstToleranceOfPric1278pric12782116W<PricIo278Spec> {
        EnblRstToleranceOfPric1278pric12782116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1278PRIC1_278\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1278pric12782216(
        &mut self,
    ) -> EnblWrProtOfPric1278pric12782216W<PricIo278Spec> {
        EnblWrProtOfPric1278pric12782216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_wr_group0of_anti_tamper(&mut self) -> EnblWrGroup0ofAntiTamperW<PricIo278Spec> {
        EnblWrGroup0ofAntiTamperW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_wr_group1of_anti_tamper(&mut self) -> EnblWrGroup1ofAntiTamperW<PricIo278Spec> {
        EnblWrGroup1ofAntiTamperW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_wr_group2of_anti_tamper(&mut self) -> EnblWrGroup2ofAntiTamperW<PricIo278Spec> {
        EnblWrGroup2ofAntiTamperW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_wr_group3of_anti_tamper(&mut self) -> EnblWrGroup3ofAntiTamperW<PricIo278Spec> {
        EnblWrGroup3ofAntiTamperW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_wr_group4of_anti_tamper(&mut self) -> EnblWrGroup4ofAntiTamperW<PricIo278Spec> {
        EnblWrGroup4ofAntiTamperW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_wr_group5of_anti_tamper(&mut self) -> EnblWrGroup5ofAntiTamperW<PricIo278Spec> {
        EnblWrGroup5ofAntiTamperW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1278PRIC1_278\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1278pric12782924(
        &mut self,
    ) -> EnblRstToleranceOfPric1278pric12782924W<PricIo278Spec> {
        EnblRstToleranceOfPric1278pric12782924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1278PRIC1_278\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1278pric12783024(
        &mut self,
    ) -> EnblWrProtOfPric1278pric12783024W<PricIo278Spec> {
        EnblWrProtOfPric1278pric12783024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#30\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io278::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io278::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo278Spec;
impl crate::RegisterSpec for PricIo278Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io278::R`](R) reader structure"]
impl crate::Readable for PricIo278Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io278::W`](W) writer structure"]
impl crate::Writable for PricIo278Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO278 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo278Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
