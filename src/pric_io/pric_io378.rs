#[doc = "Register `PRIC_IO378` reader"]
pub type R = crate::R<PricIo378Spec>;
#[doc = "Register `PRIC_IO378` writer"]
pub type W = crate::W<PricIo378Spec>;
#[doc = "Field `EnblReadGroup0OfUART11` reader - Enable Read Group #0 of UART11"]
pub type EnblReadGroup0ofUart11R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUART11` writer - Enable Read Group #0 of UART11"]
pub type EnblReadGroup0ofUart11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUART11` reader - Enable Read Group #1 of UART11"]
pub type EnblReadGroup1ofUart11R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUART11` writer - Enable Read Group #1 of UART11"]
pub type EnblReadGroup1ofUart11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUART11` reader - Enable Read Group #2 of UART11"]
pub type EnblReadGroup2ofUart11R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUART11` writer - Enable Read Group #2 of UART11"]
pub type EnblReadGroup2ofUart11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUART11` reader - Enable Read Group #3 of UART11"]
pub type EnblReadGroup3ofUart11R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUART11` writer - Enable Read Group #3 of UART11"]
pub type EnblReadGroup3ofUart11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUART11` reader - Enable Read Group #4 of UART11"]
pub type EnblReadGroup4ofUart11R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUART11` writer - Enable Read Group #4 of UART11"]
pub type EnblReadGroup4ofUart11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUART11` reader - Enable Read Group #5 of UART11"]
pub type EnblReadGroup5ofUart11R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUART11` writer - Enable Read Group #5 of UART11"]
pub type EnblReadGroup5ofUart11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1378PRIC1_378\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1378pric13780500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1378pric13780500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1378pric13780500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1378PRIC13780500` reader - Enable Reset Tolerance of PRIC1378PRIC1_378\\[05:00\\]"]
pub type EnblRstToleranceOfPric1378pric13780500R =
    crate::BitReader<EnblRstToleranceOfPric1378pric13780500>;
impl EnblRstToleranceOfPric1378pric13780500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1378pric13780500 {
        match self.bits {
            false => EnblRstToleranceOfPric1378pric13780500::ResetBySrst,
            true => EnblRstToleranceOfPric1378pric13780500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1378pric13780500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1378pric13780500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1378PRIC13780500` writer - Enable Reset Tolerance of PRIC1378PRIC1_378\\[05:00\\]"]
pub type EnblRstToleranceOfPric1378pric13780500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1378pric13780500>;
impl<'a, REG> EnblRstToleranceOfPric1378pric13780500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1378pric13780500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1378pric13780500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1378PRIC13780600` reader - Enable Write Protection of PRIC1378PRIC1_378\\[06:00\\]"]
pub type EnblWrProtOfPric1378pric13780600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1378PRIC13780600` writer - Enable Write Protection of PRIC1378PRIC1_378\\[06:00\\]"]
pub type EnblWrProtOfPric1378pric13780600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfBMCUART` reader - Enable Read Group #0 of BMC UART"]
pub type EnblReadGroup0ofBmcuartR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfBMCUART` writer - Enable Read Group #0 of BMC UART"]
pub type EnblReadGroup0ofBmcuartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfBMCUART` reader - Enable Read Group #1 of BMC UART"]
pub type EnblReadGroup1ofBmcuartR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfBMCUART` writer - Enable Read Group #1 of BMC UART"]
pub type EnblReadGroup1ofBmcuartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfBMCUART` reader - Enable Read Group #2 of BMC UART"]
pub type EnblReadGroup2ofBmcuartR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfBMCUART` writer - Enable Read Group #2 of BMC UART"]
pub type EnblReadGroup2ofBmcuartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfBMCUART` reader - Enable Read Group #3 of BMC UART"]
pub type EnblReadGroup3ofBmcuartR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfBMCUART` writer - Enable Read Group #3 of BMC UART"]
pub type EnblReadGroup3ofBmcuartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfBMCUART` reader - Enable Read Group #4 of BMC UART"]
pub type EnblReadGroup4ofBmcuartR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfBMCUART` writer - Enable Read Group #4 of BMC UART"]
pub type EnblReadGroup4ofBmcuartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfBMCUART` reader - Enable Read Group #5 of BMC UART"]
pub type EnblReadGroup5ofBmcuartR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfBMCUART` writer - Enable Read Group #5 of BMC UART"]
pub type EnblReadGroup5ofBmcuartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1378PRIC1_378\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1378pric13781308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1378pric13781308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1378pric13781308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1378PRIC13781308` reader - Enable Reset Tolerance of PRIC1378PRIC1_378\\[13:08\\]"]
pub type EnblRstToleranceOfPric1378pric13781308R =
    crate::BitReader<EnblRstToleranceOfPric1378pric13781308>;
impl EnblRstToleranceOfPric1378pric13781308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1378pric13781308 {
        match self.bits {
            false => EnblRstToleranceOfPric1378pric13781308::ResetBySrst,
            true => EnblRstToleranceOfPric1378pric13781308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1378pric13781308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1378pric13781308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1378PRIC13781308` writer - Enable Reset Tolerance of PRIC1378PRIC1_378\\[13:08\\]"]
pub type EnblRstToleranceOfPric1378pric13781308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1378pric13781308>;
impl<'a, REG> EnblRstToleranceOfPric1378pric13781308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1378pric13781308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1378pric13781308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1378PRIC13781408` reader - Enable Write Protection of PRIC1378PRIC1_378\\[14:08\\]"]
pub type EnblWrProtOfPric1378pric13781408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1378PRIC13781408` writer - Enable Write Protection of PRIC1378PRIC1_378\\[14:08\\]"]
pub type EnblWrProtOfPric1378pric13781408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfRTC` reader - Enable Read Group #0 of RTC"]
pub type EnblReadGroup0ofRtcR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfRTC` writer - Enable Read Group #0 of RTC"]
pub type EnblReadGroup0ofRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfRTC` reader - Enable Read Group #1 of RTC"]
pub type EnblReadGroup1ofRtcR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfRTC` writer - Enable Read Group #1 of RTC"]
pub type EnblReadGroup1ofRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfRTC` reader - Enable Read Group #2 of RTC"]
pub type EnblReadGroup2ofRtcR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfRTC` writer - Enable Read Group #2 of RTC"]
pub type EnblReadGroup2ofRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfRTC` reader - Enable Read Group #3 of RTC"]
pub type EnblReadGroup3ofRtcR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfRTC` writer - Enable Read Group #3 of RTC"]
pub type EnblReadGroup3ofRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfRTC` reader - Enable Read Group #4 of RTC"]
pub type EnblReadGroup4ofRtcR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfRTC` writer - Enable Read Group #4 of RTC"]
pub type EnblReadGroup4ofRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfRTC` reader - Enable Read Group #5 of RTC"]
pub type EnblReadGroup5ofRtcR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfRTC` writer - Enable Read Group #5 of RTC"]
pub type EnblReadGroup5ofRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1378PRIC1_378\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1378pric13782116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1378pric13782116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1378pric13782116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1378PRIC13782116` reader - Enable Reset Tolerance of PRIC1378PRIC1_378\\[21:16\\]"]
pub type EnblRstToleranceOfPric1378pric13782116R =
    crate::BitReader<EnblRstToleranceOfPric1378pric13782116>;
impl EnblRstToleranceOfPric1378pric13782116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1378pric13782116 {
        match self.bits {
            false => EnblRstToleranceOfPric1378pric13782116::ResetBySrst,
            true => EnblRstToleranceOfPric1378pric13782116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1378pric13782116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1378pric13782116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1378PRIC13782116` writer - Enable Reset Tolerance of PRIC1378PRIC1_378\\[21:16\\]"]
pub type EnblRstToleranceOfPric1378pric13782116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1378pric13782116>;
impl<'a, REG> EnblRstToleranceOfPric1378pric13782116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1378pric13782116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1378pric13782116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1378PRIC13782216` reader - Enable Write Protection of PRIC1378PRIC1_378\\[22:16\\]"]
pub type EnblWrProtOfPric1378pric13782216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1378PRIC13782216` writer - Enable Write Protection of PRIC1378PRIC1_378\\[22:16\\]"]
pub type EnblWrProtOfPric1378pric13782216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfAntiTamper` reader - Enable Read Group #0 of Anti Tamper"]
pub type EnblReadGroup0ofAntiTamperR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfAntiTamper` writer - Enable Read Group #0 of Anti Tamper"]
pub type EnblReadGroup0ofAntiTamperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfAntiTamper` reader - Enable Read Group #1 of Anti Tamper"]
pub type EnblReadGroup1ofAntiTamperR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfAntiTamper` writer - Enable Read Group #1 of Anti Tamper"]
pub type EnblReadGroup1ofAntiTamperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfAntiTamper` reader - Enable Read Group #2 of Anti Tamper"]
pub type EnblReadGroup2ofAntiTamperR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfAntiTamper` writer - Enable Read Group #2 of Anti Tamper"]
pub type EnblReadGroup2ofAntiTamperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfAntiTamper` reader - Enable Read Group #3 of Anti Tamper"]
pub type EnblReadGroup3ofAntiTamperR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfAntiTamper` writer - Enable Read Group #3 of Anti Tamper"]
pub type EnblReadGroup3ofAntiTamperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfAntiTamper` reader - Enable Read Group #4 of Anti Tamper"]
pub type EnblReadGroup4ofAntiTamperR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfAntiTamper` writer - Enable Read Group #4 of Anti Tamper"]
pub type EnblReadGroup4ofAntiTamperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfAntiTamper` reader - Enable Read Group #5 of Anti Tamper"]
pub type EnblReadGroup5ofAntiTamperR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfAntiTamper` writer - Enable Read Group #5 of Anti Tamper"]
pub type EnblReadGroup5ofAntiTamperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1378PRIC1_378\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1378pric13782924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1378pric13782924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1378pric13782924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1378PRIC13782924` reader - Enable Reset Tolerance of PRIC1378PRIC1_378\\[29:24\\]"]
pub type EnblRstToleranceOfPric1378pric13782924R =
    crate::BitReader<EnblRstToleranceOfPric1378pric13782924>;
impl EnblRstToleranceOfPric1378pric13782924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1378pric13782924 {
        match self.bits {
            false => EnblRstToleranceOfPric1378pric13782924::ResetBySrst,
            true => EnblRstToleranceOfPric1378pric13782924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1378pric13782924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1378pric13782924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1378PRIC13782924` writer - Enable Reset Tolerance of PRIC1378PRIC1_378\\[29:24\\]"]
pub type EnblRstToleranceOfPric1378pric13782924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1378pric13782924>;
impl<'a, REG> EnblRstToleranceOfPric1378pric13782924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1378pric13782924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1378pric13782924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1378PRIC13783024` reader - Enable Write Protection of PRIC1378PRIC1_378\\[30:24\\]"]
pub type EnblWrProtOfPric1378pric13783024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1378PRIC13783024` writer - Enable Write Protection of PRIC1378PRIC1_378\\[30:24\\]"]
pub type EnblWrProtOfPric1378pric13783024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of UART11"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart11(&self) -> EnblReadGroup0ofUart11R {
        EnblReadGroup0ofUart11R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of UART11"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart11(&self) -> EnblReadGroup1ofUart11R {
        EnblReadGroup1ofUart11R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of UART11"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart11(&self) -> EnblReadGroup2ofUart11R {
        EnblReadGroup2ofUart11R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of UART11"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart11(&self) -> EnblReadGroup3ofUart11R {
        EnblReadGroup3ofUart11R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of UART11"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart11(&self) -> EnblReadGroup4ofUart11R {
        EnblReadGroup4ofUart11R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of UART11"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart11(&self) -> EnblReadGroup5ofUart11R {
        EnblReadGroup5ofUart11R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1378PRIC1_378\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1378pric13780500(
        &self,
    ) -> EnblRstToleranceOfPric1378pric13780500R {
        EnblRstToleranceOfPric1378pric13780500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1378PRIC1_378\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1378pric13780600(&self) -> EnblWrProtOfPric1378pric13780600R {
        EnblWrProtOfPric1378pric13780600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of BMC UART"]
    #[inline(always)]
    pub fn enbl_read_group0of_bmcuart(&self) -> EnblReadGroup0ofBmcuartR {
        EnblReadGroup0ofBmcuartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of BMC UART"]
    #[inline(always)]
    pub fn enbl_read_group1of_bmcuart(&self) -> EnblReadGroup1ofBmcuartR {
        EnblReadGroup1ofBmcuartR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of BMC UART"]
    #[inline(always)]
    pub fn enbl_read_group2of_bmcuart(&self) -> EnblReadGroup2ofBmcuartR {
        EnblReadGroup2ofBmcuartR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of BMC UART"]
    #[inline(always)]
    pub fn enbl_read_group3of_bmcuart(&self) -> EnblReadGroup3ofBmcuartR {
        EnblReadGroup3ofBmcuartR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of BMC UART"]
    #[inline(always)]
    pub fn enbl_read_group4of_bmcuart(&self) -> EnblReadGroup4ofBmcuartR {
        EnblReadGroup4ofBmcuartR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of BMC UART"]
    #[inline(always)]
    pub fn enbl_read_group5of_bmcuart(&self) -> EnblReadGroup5ofBmcuartR {
        EnblReadGroup5ofBmcuartR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1378PRIC1_378\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1378pric13781308(
        &self,
    ) -> EnblRstToleranceOfPric1378pric13781308R {
        EnblRstToleranceOfPric1378pric13781308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1378PRIC1_378\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1378pric13781408(&self) -> EnblWrProtOfPric1378pric13781408R {
        EnblWrProtOfPric1378pric13781408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of RTC"]
    #[inline(always)]
    pub fn enbl_read_group0of_rtc(&self) -> EnblReadGroup0ofRtcR {
        EnblReadGroup0ofRtcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of RTC"]
    #[inline(always)]
    pub fn enbl_read_group1of_rtc(&self) -> EnblReadGroup1ofRtcR {
        EnblReadGroup1ofRtcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of RTC"]
    #[inline(always)]
    pub fn enbl_read_group2of_rtc(&self) -> EnblReadGroup2ofRtcR {
        EnblReadGroup2ofRtcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of RTC"]
    #[inline(always)]
    pub fn enbl_read_group3of_rtc(&self) -> EnblReadGroup3ofRtcR {
        EnblReadGroup3ofRtcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of RTC"]
    #[inline(always)]
    pub fn enbl_read_group4of_rtc(&self) -> EnblReadGroup4ofRtcR {
        EnblReadGroup4ofRtcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of RTC"]
    #[inline(always)]
    pub fn enbl_read_group5of_rtc(&self) -> EnblReadGroup5ofRtcR {
        EnblReadGroup5ofRtcR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1378PRIC1_378\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1378pric13782116(
        &self,
    ) -> EnblRstToleranceOfPric1378pric13782116R {
        EnblRstToleranceOfPric1378pric13782116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1378PRIC1_378\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1378pric13782216(&self) -> EnblWrProtOfPric1378pric13782216R {
        EnblWrProtOfPric1378pric13782216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_read_group0of_anti_tamper(&self) -> EnblReadGroup0ofAntiTamperR {
        EnblReadGroup0ofAntiTamperR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_read_group1of_anti_tamper(&self) -> EnblReadGroup1ofAntiTamperR {
        EnblReadGroup1ofAntiTamperR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_read_group2of_anti_tamper(&self) -> EnblReadGroup2ofAntiTamperR {
        EnblReadGroup2ofAntiTamperR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_read_group3of_anti_tamper(&self) -> EnblReadGroup3ofAntiTamperR {
        EnblReadGroup3ofAntiTamperR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_read_group4of_anti_tamper(&self) -> EnblReadGroup4ofAntiTamperR {
        EnblReadGroup4ofAntiTamperR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_read_group5of_anti_tamper(&self) -> EnblReadGroup5ofAntiTamperR {
        EnblReadGroup5ofAntiTamperR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1378PRIC1_378\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1378pric13782924(
        &self,
    ) -> EnblRstToleranceOfPric1378pric13782924R {
        EnblRstToleranceOfPric1378pric13782924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1378PRIC1_378\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1378pric13783024(&self) -> EnblWrProtOfPric1378pric13783024R {
        EnblWrProtOfPric1378pric13783024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of UART11"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart11(&mut self) -> EnblReadGroup0ofUart11W<PricIo378Spec> {
        EnblReadGroup0ofUart11W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of UART11"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart11(&mut self) -> EnblReadGroup1ofUart11W<PricIo378Spec> {
        EnblReadGroup1ofUart11W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of UART11"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart11(&mut self) -> EnblReadGroup2ofUart11W<PricIo378Spec> {
        EnblReadGroup2ofUart11W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of UART11"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart11(&mut self) -> EnblReadGroup3ofUart11W<PricIo378Spec> {
        EnblReadGroup3ofUart11W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of UART11"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart11(&mut self) -> EnblReadGroup4ofUart11W<PricIo378Spec> {
        EnblReadGroup4ofUart11W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of UART11"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart11(&mut self) -> EnblReadGroup5ofUart11W<PricIo378Spec> {
        EnblReadGroup5ofUart11W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1378PRIC1_378\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1378pric13780500(
        &mut self,
    ) -> EnblRstToleranceOfPric1378pric13780500W<PricIo378Spec> {
        EnblRstToleranceOfPric1378pric13780500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1378PRIC1_378\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1378pric13780600(
        &mut self,
    ) -> EnblWrProtOfPric1378pric13780600W<PricIo378Spec> {
        EnblWrProtOfPric1378pric13780600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of BMC UART"]
    #[inline(always)]
    pub fn enbl_read_group0of_bmcuart(&mut self) -> EnblReadGroup0ofBmcuartW<PricIo378Spec> {
        EnblReadGroup0ofBmcuartW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of BMC UART"]
    #[inline(always)]
    pub fn enbl_read_group1of_bmcuart(&mut self) -> EnblReadGroup1ofBmcuartW<PricIo378Spec> {
        EnblReadGroup1ofBmcuartW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of BMC UART"]
    #[inline(always)]
    pub fn enbl_read_group2of_bmcuart(&mut self) -> EnblReadGroup2ofBmcuartW<PricIo378Spec> {
        EnblReadGroup2ofBmcuartW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of BMC UART"]
    #[inline(always)]
    pub fn enbl_read_group3of_bmcuart(&mut self) -> EnblReadGroup3ofBmcuartW<PricIo378Spec> {
        EnblReadGroup3ofBmcuartW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of BMC UART"]
    #[inline(always)]
    pub fn enbl_read_group4of_bmcuart(&mut self) -> EnblReadGroup4ofBmcuartW<PricIo378Spec> {
        EnblReadGroup4ofBmcuartW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of BMC UART"]
    #[inline(always)]
    pub fn enbl_read_group5of_bmcuart(&mut self) -> EnblReadGroup5ofBmcuartW<PricIo378Spec> {
        EnblReadGroup5ofBmcuartW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1378PRIC1_378\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1378pric13781308(
        &mut self,
    ) -> EnblRstToleranceOfPric1378pric13781308W<PricIo378Spec> {
        EnblRstToleranceOfPric1378pric13781308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1378PRIC1_378\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1378pric13781408(
        &mut self,
    ) -> EnblWrProtOfPric1378pric13781408W<PricIo378Spec> {
        EnblWrProtOfPric1378pric13781408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of RTC"]
    #[inline(always)]
    pub fn enbl_read_group0of_rtc(&mut self) -> EnblReadGroup0ofRtcW<PricIo378Spec> {
        EnblReadGroup0ofRtcW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of RTC"]
    #[inline(always)]
    pub fn enbl_read_group1of_rtc(&mut self) -> EnblReadGroup1ofRtcW<PricIo378Spec> {
        EnblReadGroup1ofRtcW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of RTC"]
    #[inline(always)]
    pub fn enbl_read_group2of_rtc(&mut self) -> EnblReadGroup2ofRtcW<PricIo378Spec> {
        EnblReadGroup2ofRtcW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of RTC"]
    #[inline(always)]
    pub fn enbl_read_group3of_rtc(&mut self) -> EnblReadGroup3ofRtcW<PricIo378Spec> {
        EnblReadGroup3ofRtcW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of RTC"]
    #[inline(always)]
    pub fn enbl_read_group4of_rtc(&mut self) -> EnblReadGroup4ofRtcW<PricIo378Spec> {
        EnblReadGroup4ofRtcW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of RTC"]
    #[inline(always)]
    pub fn enbl_read_group5of_rtc(&mut self) -> EnblReadGroup5ofRtcW<PricIo378Spec> {
        EnblReadGroup5ofRtcW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1378PRIC1_378\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1378pric13782116(
        &mut self,
    ) -> EnblRstToleranceOfPric1378pric13782116W<PricIo378Spec> {
        EnblRstToleranceOfPric1378pric13782116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1378PRIC1_378\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1378pric13782216(
        &mut self,
    ) -> EnblWrProtOfPric1378pric13782216W<PricIo378Spec> {
        EnblWrProtOfPric1378pric13782216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_read_group0of_anti_tamper(&mut self) -> EnblReadGroup0ofAntiTamperW<PricIo378Spec> {
        EnblReadGroup0ofAntiTamperW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_read_group1of_anti_tamper(&mut self) -> EnblReadGroup1ofAntiTamperW<PricIo378Spec> {
        EnblReadGroup1ofAntiTamperW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_read_group2of_anti_tamper(&mut self) -> EnblReadGroup2ofAntiTamperW<PricIo378Spec> {
        EnblReadGroup2ofAntiTamperW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_read_group3of_anti_tamper(&mut self) -> EnblReadGroup3ofAntiTamperW<PricIo378Spec> {
        EnblReadGroup3ofAntiTamperW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_read_group4of_anti_tamper(&mut self) -> EnblReadGroup4ofAntiTamperW<PricIo378Spec> {
        EnblReadGroup4ofAntiTamperW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of Anti Tamper"]
    #[inline(always)]
    pub fn enbl_read_group5of_anti_tamper(&mut self) -> EnblReadGroup5ofAntiTamperW<PricIo378Spec> {
        EnblReadGroup5ofAntiTamperW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1378PRIC1_378\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1378pric13782924(
        &mut self,
    ) -> EnblRstToleranceOfPric1378pric13782924W<PricIo378Spec> {
        EnblRstToleranceOfPric1378pric13782924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1378PRIC1_378\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1378pric13783024(
        &mut self,
    ) -> EnblWrProtOfPric1378pric13783024W<PricIo378Spec> {
        EnblWrProtOfPric1378pric13783024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#30\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io378::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io378::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo378Spec;
impl crate::RegisterSpec for PricIo378Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io378::R`](R) reader structure"]
impl crate::Readable for PricIo378Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io378::W`](W) writer structure"]
impl crate::Writable for PricIo378Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO378 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo378Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
