#[doc = "Register `PRIC_IO230` reader"]
pub type R = crate::R<PricIo230Spec>;
#[doc = "Register `PRIC_IO230` writer"]
pub type W = crate::W<PricIo230Spec>;
#[doc = "Field `EnblWrGroup0OfGPIO` reader - Enable Write Group #0 of GPIO"]
pub type EnblWrGroup0ofGpioR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfGPIO` writer - Enable Write Group #0 of GPIO"]
pub type EnblWrGroup0ofGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfGPIO` reader - Enable Write Group #1 of GPIO"]
pub type EnblWrGroup1ofGpioR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfGPIO` writer - Enable Write Group #1 of GPIO"]
pub type EnblWrGroup1ofGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfGPIO` reader - Enable Write Group #2 of GPIO"]
pub type EnblWrGroup2ofGpioR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfGPIO` writer - Enable Write Group #2 of GPIO"]
pub type EnblWrGroup2ofGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfGPIO` reader - Enable Write Group #3 of GPIO"]
pub type EnblWrGroup3ofGpioR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfGPIO` writer - Enable Write Group #3 of GPIO"]
pub type EnblWrGroup3ofGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfGPIO` reader - Enable Write Group #4 of GPIO"]
pub type EnblWrGroup4ofGpioR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfGPIO` writer - Enable Write Group #4 of GPIO"]
pub type EnblWrGroup4ofGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfGPIO` reader - Enable Write Group #5 of GPIO"]
pub type EnblWrGroup5ofGpioR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfGPIO` writer - Enable Write Group #5 of GPIO"]
pub type EnblWrGroup5ofGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1230PRIC1_230\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1230pric12300500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1230pric12300500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1230pric12300500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1230PRIC12300500` reader - Enable Reset Tolerance of PRIC1230PRIC1_230\\[05:00\\]"]
pub type EnblRstToleranceOfPric1230pric12300500R =
    crate::BitReader<EnblRstToleranceOfPric1230pric12300500>;
impl EnblRstToleranceOfPric1230pric12300500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1230pric12300500 {
        match self.bits {
            false => EnblRstToleranceOfPric1230pric12300500::ResetBySrst,
            true => EnblRstToleranceOfPric1230pric12300500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1230pric12300500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1230pric12300500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1230PRIC12300500` writer - Enable Reset Tolerance of PRIC1230PRIC1_230\\[05:00\\]"]
pub type EnblRstToleranceOfPric1230pric12300500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1230pric12300500>;
impl<'a, REG> EnblRstToleranceOfPric1230pric12300500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1230pric12300500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1230pric12300500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1230PRIC12300600` reader - Enable Write Protection of PRIC1230PRIC1_230\\[06:00\\]"]
pub type EnblWrProtOfPric1230pric12300600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1230PRIC12300600` writer - Enable Write Protection of PRIC1230PRIC1_230\\[06:00\\]"]
pub type EnblWrProtOfPric1230pric12300600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfGPIOPrivilege` reader - Enable Write Group #0 of GPIO Privilege"]
pub type EnblWrGroup0ofGpioprivilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfGPIOPrivilege` writer - Enable Write Group #0 of GPIO Privilege"]
pub type EnblWrGroup0ofGpioprivilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfGPIOPrivilege` reader - Enable Write Group #1 of GPIO Privilege"]
pub type EnblWrGroup1ofGpioprivilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfGPIOPrivilege` writer - Enable Write Group #1 of GPIO Privilege"]
pub type EnblWrGroup1ofGpioprivilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfGPIOPrivilege` reader - Enable Write Group #2 of GPIO Privilege"]
pub type EnblWrGroup2ofGpioprivilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfGPIOPrivilege` writer - Enable Write Group #2 of GPIO Privilege"]
pub type EnblWrGroup2ofGpioprivilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfGPIOPrivilege` reader - Enable Write Group #3 of GPIO Privilege"]
pub type EnblWrGroup3ofGpioprivilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfGPIOPrivilege` writer - Enable Write Group #3 of GPIO Privilege"]
pub type EnblWrGroup3ofGpioprivilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfGPIOPrivilege` reader - Enable Write Group #4 of GPIO Privilege"]
pub type EnblWrGroup4ofGpioprivilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfGPIOPrivilege` writer - Enable Write Group #4 of GPIO Privilege"]
pub type EnblWrGroup4ofGpioprivilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfGPIOPrivilege` reader - Enable Write Group #5 of GPIO Privilege"]
pub type EnblWrGroup5ofGpioprivilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfGPIOPrivilege` writer - Enable Write Group #5 of GPIO Privilege"]
pub type EnblWrGroup5ofGpioprivilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1230PRIC1_230\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1230pric12301308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1230pric12301308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1230pric12301308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1230PRIC12301308` reader - Enable Reset Tolerance of PRIC1230PRIC1_230\\[13:08\\]"]
pub type EnblRstToleranceOfPric1230pric12301308R =
    crate::BitReader<EnblRstToleranceOfPric1230pric12301308>;
impl EnblRstToleranceOfPric1230pric12301308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1230pric12301308 {
        match self.bits {
            false => EnblRstToleranceOfPric1230pric12301308::ResetBySrst,
            true => EnblRstToleranceOfPric1230pric12301308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1230pric12301308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1230pric12301308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1230PRIC12301308` writer - Enable Reset Tolerance of PRIC1230PRIC1_230\\[13:08\\]"]
pub type EnblRstToleranceOfPric1230pric12301308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1230pric12301308>;
impl<'a, REG> EnblRstToleranceOfPric1230pric12301308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1230pric12301308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1230pric12301308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1230PRIC12301408` reader - Enable Write Protection of PRIC1230PRIC1_230\\[14:08\\]"]
pub type EnblWrProtOfPric1230pric12301408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1230PRIC12301408` writer - Enable Write Protection of PRIC1230PRIC1_230\\[14:08\\]"]
pub type EnblWrProtOfPric1230pric12301408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfSGPIOMaster0` reader - Enable Write Group #0 of SGPIO Master 0"]
pub type EnblWrGroup0ofSgpiomaster0R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfSGPIOMaster0` writer - Enable Write Group #0 of SGPIO Master 0"]
pub type EnblWrGroup0ofSgpiomaster0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfSGPIOMaster0` reader - Enable Write Group #1 of SGPIO Master 0"]
pub type EnblWrGroup1ofSgpiomaster0R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfSGPIOMaster0` writer - Enable Write Group #1 of SGPIO Master 0"]
pub type EnblWrGroup1ofSgpiomaster0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfSGPIOMaster0` reader - Enable Write Group #2 of SGPIO Master 0"]
pub type EnblWrGroup2ofSgpiomaster0R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfSGPIOMaster0` writer - Enable Write Group #2 of SGPIO Master 0"]
pub type EnblWrGroup2ofSgpiomaster0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfSGPIOMaster0` reader - Enable Write Group #3 of SGPIO Master 0"]
pub type EnblWrGroup3ofSgpiomaster0R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfSGPIOMaster0` writer - Enable Write Group #3 of SGPIO Master 0"]
pub type EnblWrGroup3ofSgpiomaster0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfSGPIOMaster0` reader - Enable Write Group #4 of SGPIO Master 0"]
pub type EnblWrGroup4ofSgpiomaster0R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfSGPIOMaster0` writer - Enable Write Group #4 of SGPIO Master 0"]
pub type EnblWrGroup4ofSgpiomaster0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfSGPIOMaster0` reader - Enable Write Group #5 of SGPIO Master 0"]
pub type EnblWrGroup5ofSgpiomaster0R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfSGPIOMaster0` writer - Enable Write Group #5 of SGPIO Master 0"]
pub type EnblWrGroup5ofSgpiomaster0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1230PRIC1_230\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1230pric12302116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1230pric12302116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1230pric12302116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1230PRIC12302116` reader - Enable Reset Tolerance of PRIC1230PRIC1_230\\[21:16\\]"]
pub type EnblRstToleranceOfPric1230pric12302116R =
    crate::BitReader<EnblRstToleranceOfPric1230pric12302116>;
impl EnblRstToleranceOfPric1230pric12302116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1230pric12302116 {
        match self.bits {
            false => EnblRstToleranceOfPric1230pric12302116::ResetBySrst,
            true => EnblRstToleranceOfPric1230pric12302116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1230pric12302116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1230pric12302116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1230PRIC12302116` writer - Enable Reset Tolerance of PRIC1230PRIC1_230\\[21:16\\]"]
pub type EnblRstToleranceOfPric1230pric12302116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1230pric12302116>;
impl<'a, REG> EnblRstToleranceOfPric1230pric12302116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1230pric12302116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1230pric12302116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1230PRIC12302216` reader - Enable Write Protection of PRIC1230PRIC1_230\\[22:16\\]"]
pub type EnblWrProtOfPric1230pric12302216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1230PRIC12302216` writer - Enable Write Protection of PRIC1230PRIC1_230\\[22:16\\]"]
pub type EnblWrProtOfPric1230pric12302216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfSGPIOMaster0Privilege` reader - Enable Write Group #0 of SGPIO Master 0 Privilege"]
pub type EnblWrGroup0ofSgpiomaster0privilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfSGPIOMaster0Privilege` writer - Enable Write Group #0 of SGPIO Master 0 Privilege"]
pub type EnblWrGroup0ofSgpiomaster0privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfSGPIOMaster0Privilege` reader - Enable Write Group #1 of SGPIO Master 0 Privilege"]
pub type EnblWrGroup1ofSgpiomaster0privilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfSGPIOMaster0Privilege` writer - Enable Write Group #1 of SGPIO Master 0 Privilege"]
pub type EnblWrGroup1ofSgpiomaster0privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfSGPIOMaster0Privilege` reader - Enable Write Group #2 of SGPIO Master 0 Privilege"]
pub type EnblWrGroup2ofSgpiomaster0privilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfSGPIOMaster0Privilege` writer - Enable Write Group #2 of SGPIO Master 0 Privilege"]
pub type EnblWrGroup2ofSgpiomaster0privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfSGPIOMaster0Privilege` reader - Enable Write Group #3 of SGPIO Master 0 Privilege"]
pub type EnblWrGroup3ofSgpiomaster0privilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfSGPIOMaster0Privilege` writer - Enable Write Group #3 of SGPIO Master 0 Privilege"]
pub type EnblWrGroup3ofSgpiomaster0privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfSGPIOMaster0Privilege` reader - Enable Write Group #4 of SGPIO Master 0 Privilege"]
pub type EnblWrGroup4ofSgpiomaster0privilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfSGPIOMaster0Privilege` writer - Enable Write Group #4 of SGPIO Master 0 Privilege"]
pub type EnblWrGroup4ofSgpiomaster0privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfSGPIOMaster0Privilege` reader - Enable Write Group #5 of SGPIO Master 0 Privilege"]
pub type EnblWrGroup5ofSgpiomaster0privilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfSGPIOMaster0Privilege` writer - Enable Write Group #5 of SGPIO Master 0 Privilege"]
pub type EnblWrGroup5ofSgpiomaster0privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1230PRIC1_230\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1230pric12302924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1230pric12302924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1230pric12302924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1230PRIC12302924` reader - Enable Reset Tolerance of PRIC1230PRIC1_230\\[29:24\\]"]
pub type EnblRstToleranceOfPric1230pric12302924R =
    crate::BitReader<EnblRstToleranceOfPric1230pric12302924>;
impl EnblRstToleranceOfPric1230pric12302924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1230pric12302924 {
        match self.bits {
            false => EnblRstToleranceOfPric1230pric12302924::ResetBySrst,
            true => EnblRstToleranceOfPric1230pric12302924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1230pric12302924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1230pric12302924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1230PRIC12302924` writer - Enable Reset Tolerance of PRIC1230PRIC1_230\\[29:24\\]"]
pub type EnblRstToleranceOfPric1230pric12302924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1230pric12302924>;
impl<'a, REG> EnblRstToleranceOfPric1230pric12302924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1230pric12302924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1230pric12302924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1230PRIC12303024` reader - Enable Write Protection of PRIC1230PRIC1_230\\[30:24\\]"]
pub type EnblWrProtOfPric1230pric12303024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1230PRIC12303024` writer - Enable Write Protection of PRIC1230PRIC1_230\\[30:24\\]"]
pub type EnblWrProtOfPric1230pric12303024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of GPIO"]
    #[inline(always)]
    pub fn enbl_wr_group0of_gpio(&self) -> EnblWrGroup0ofGpioR {
        EnblWrGroup0ofGpioR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of GPIO"]
    #[inline(always)]
    pub fn enbl_wr_group1of_gpio(&self) -> EnblWrGroup1ofGpioR {
        EnblWrGroup1ofGpioR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of GPIO"]
    #[inline(always)]
    pub fn enbl_wr_group2of_gpio(&self) -> EnblWrGroup2ofGpioR {
        EnblWrGroup2ofGpioR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of GPIO"]
    #[inline(always)]
    pub fn enbl_wr_group3of_gpio(&self) -> EnblWrGroup3ofGpioR {
        EnblWrGroup3ofGpioR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of GPIO"]
    #[inline(always)]
    pub fn enbl_wr_group4of_gpio(&self) -> EnblWrGroup4ofGpioR {
        EnblWrGroup4ofGpioR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of GPIO"]
    #[inline(always)]
    pub fn enbl_wr_group5of_gpio(&self) -> EnblWrGroup5ofGpioR {
        EnblWrGroup5ofGpioR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1230PRIC1_230\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1230pric12300500(
        &self,
    ) -> EnblRstToleranceOfPric1230pric12300500R {
        EnblRstToleranceOfPric1230pric12300500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1230PRIC1_230\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1230pric12300600(&self) -> EnblWrProtOfPric1230pric12300600R {
        EnblWrProtOfPric1230pric12300600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group0of_gpioprivilege(&self) -> EnblWrGroup0ofGpioprivilegeR {
        EnblWrGroup0ofGpioprivilegeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group1of_gpioprivilege(&self) -> EnblWrGroup1ofGpioprivilegeR {
        EnblWrGroup1ofGpioprivilegeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group2of_gpioprivilege(&self) -> EnblWrGroup2ofGpioprivilegeR {
        EnblWrGroup2ofGpioprivilegeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group3of_gpioprivilege(&self) -> EnblWrGroup3ofGpioprivilegeR {
        EnblWrGroup3ofGpioprivilegeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group4of_gpioprivilege(&self) -> EnblWrGroup4ofGpioprivilegeR {
        EnblWrGroup4ofGpioprivilegeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group5of_gpioprivilege(&self) -> EnblWrGroup5ofGpioprivilegeR {
        EnblWrGroup5ofGpioprivilegeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1230PRIC1_230\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1230pric12301308(
        &self,
    ) -> EnblRstToleranceOfPric1230pric12301308R {
        EnblRstToleranceOfPric1230pric12301308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1230PRIC1_230\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1230pric12301408(&self) -> EnblWrProtOfPric1230pric12301408R {
        EnblWrProtOfPric1230pric12301408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_wr_group0of_sgpiomaster0(&self) -> EnblWrGroup0ofSgpiomaster0R {
        EnblWrGroup0ofSgpiomaster0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_wr_group1of_sgpiomaster0(&self) -> EnblWrGroup1ofSgpiomaster0R {
        EnblWrGroup1ofSgpiomaster0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_wr_group2of_sgpiomaster0(&self) -> EnblWrGroup2ofSgpiomaster0R {
        EnblWrGroup2ofSgpiomaster0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_wr_group3of_sgpiomaster0(&self) -> EnblWrGroup3ofSgpiomaster0R {
        EnblWrGroup3ofSgpiomaster0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_wr_group4of_sgpiomaster0(&self) -> EnblWrGroup4ofSgpiomaster0R {
        EnblWrGroup4ofSgpiomaster0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_wr_group5of_sgpiomaster0(&self) -> EnblWrGroup5ofSgpiomaster0R {
        EnblWrGroup5ofSgpiomaster0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1230PRIC1_230\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1230pric12302116(
        &self,
    ) -> EnblRstToleranceOfPric1230pric12302116R {
        EnblRstToleranceOfPric1230pric12302116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1230PRIC1_230\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1230pric12302216(&self) -> EnblWrProtOfPric1230pric12302216R {
        EnblWrProtOfPric1230pric12302216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group0of_sgpiomaster0privilege(&self) -> EnblWrGroup0ofSgpiomaster0privilegeR {
        EnblWrGroup0ofSgpiomaster0privilegeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group1of_sgpiomaster0privilege(&self) -> EnblWrGroup1ofSgpiomaster0privilegeR {
        EnblWrGroup1ofSgpiomaster0privilegeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group2of_sgpiomaster0privilege(&self) -> EnblWrGroup2ofSgpiomaster0privilegeR {
        EnblWrGroup2ofSgpiomaster0privilegeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group3of_sgpiomaster0privilege(&self) -> EnblWrGroup3ofSgpiomaster0privilegeR {
        EnblWrGroup3ofSgpiomaster0privilegeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group4of_sgpiomaster0privilege(&self) -> EnblWrGroup4ofSgpiomaster0privilegeR {
        EnblWrGroup4ofSgpiomaster0privilegeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group5of_sgpiomaster0privilege(&self) -> EnblWrGroup5ofSgpiomaster0privilegeR {
        EnblWrGroup5ofSgpiomaster0privilegeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1230PRIC1_230\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1230pric12302924(
        &self,
    ) -> EnblRstToleranceOfPric1230pric12302924R {
        EnblRstToleranceOfPric1230pric12302924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1230PRIC1_230\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1230pric12303024(&self) -> EnblWrProtOfPric1230pric12303024R {
        EnblWrProtOfPric1230pric12303024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of GPIO"]
    #[inline(always)]
    pub fn enbl_wr_group0of_gpio(&mut self) -> EnblWrGroup0ofGpioW<PricIo230Spec> {
        EnblWrGroup0ofGpioW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of GPIO"]
    #[inline(always)]
    pub fn enbl_wr_group1of_gpio(&mut self) -> EnblWrGroup1ofGpioW<PricIo230Spec> {
        EnblWrGroup1ofGpioW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of GPIO"]
    #[inline(always)]
    pub fn enbl_wr_group2of_gpio(&mut self) -> EnblWrGroup2ofGpioW<PricIo230Spec> {
        EnblWrGroup2ofGpioW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of GPIO"]
    #[inline(always)]
    pub fn enbl_wr_group3of_gpio(&mut self) -> EnblWrGroup3ofGpioW<PricIo230Spec> {
        EnblWrGroup3ofGpioW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of GPIO"]
    #[inline(always)]
    pub fn enbl_wr_group4of_gpio(&mut self) -> EnblWrGroup4ofGpioW<PricIo230Spec> {
        EnblWrGroup4ofGpioW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of GPIO"]
    #[inline(always)]
    pub fn enbl_wr_group5of_gpio(&mut self) -> EnblWrGroup5ofGpioW<PricIo230Spec> {
        EnblWrGroup5ofGpioW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1230PRIC1_230\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1230pric12300500(
        &mut self,
    ) -> EnblRstToleranceOfPric1230pric12300500W<PricIo230Spec> {
        EnblRstToleranceOfPric1230pric12300500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1230PRIC1_230\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1230pric12300600(
        &mut self,
    ) -> EnblWrProtOfPric1230pric12300600W<PricIo230Spec> {
        EnblWrProtOfPric1230pric12300600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group0of_gpioprivilege(
        &mut self,
    ) -> EnblWrGroup0ofGpioprivilegeW<PricIo230Spec> {
        EnblWrGroup0ofGpioprivilegeW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group1of_gpioprivilege(
        &mut self,
    ) -> EnblWrGroup1ofGpioprivilegeW<PricIo230Spec> {
        EnblWrGroup1ofGpioprivilegeW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group2of_gpioprivilege(
        &mut self,
    ) -> EnblWrGroup2ofGpioprivilegeW<PricIo230Spec> {
        EnblWrGroup2ofGpioprivilegeW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group3of_gpioprivilege(
        &mut self,
    ) -> EnblWrGroup3ofGpioprivilegeW<PricIo230Spec> {
        EnblWrGroup3ofGpioprivilegeW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group4of_gpioprivilege(
        &mut self,
    ) -> EnblWrGroup4ofGpioprivilegeW<PricIo230Spec> {
        EnblWrGroup4ofGpioprivilegeW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group5of_gpioprivilege(
        &mut self,
    ) -> EnblWrGroup5ofGpioprivilegeW<PricIo230Spec> {
        EnblWrGroup5ofGpioprivilegeW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1230PRIC1_230\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1230pric12301308(
        &mut self,
    ) -> EnblRstToleranceOfPric1230pric12301308W<PricIo230Spec> {
        EnblRstToleranceOfPric1230pric12301308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1230PRIC1_230\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1230pric12301408(
        &mut self,
    ) -> EnblWrProtOfPric1230pric12301408W<PricIo230Spec> {
        EnblWrProtOfPric1230pric12301408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_wr_group0of_sgpiomaster0(&mut self) -> EnblWrGroup0ofSgpiomaster0W<PricIo230Spec> {
        EnblWrGroup0ofSgpiomaster0W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_wr_group1of_sgpiomaster0(&mut self) -> EnblWrGroup1ofSgpiomaster0W<PricIo230Spec> {
        EnblWrGroup1ofSgpiomaster0W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_wr_group2of_sgpiomaster0(&mut self) -> EnblWrGroup2ofSgpiomaster0W<PricIo230Spec> {
        EnblWrGroup2ofSgpiomaster0W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_wr_group3of_sgpiomaster0(&mut self) -> EnblWrGroup3ofSgpiomaster0W<PricIo230Spec> {
        EnblWrGroup3ofSgpiomaster0W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_wr_group4of_sgpiomaster0(&mut self) -> EnblWrGroup4ofSgpiomaster0W<PricIo230Spec> {
        EnblWrGroup4ofSgpiomaster0W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_wr_group5of_sgpiomaster0(&mut self) -> EnblWrGroup5ofSgpiomaster0W<PricIo230Spec> {
        EnblWrGroup5ofSgpiomaster0W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1230PRIC1_230\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1230pric12302116(
        &mut self,
    ) -> EnblRstToleranceOfPric1230pric12302116W<PricIo230Spec> {
        EnblRstToleranceOfPric1230pric12302116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1230PRIC1_230\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1230pric12302216(
        &mut self,
    ) -> EnblWrProtOfPric1230pric12302216W<PricIo230Spec> {
        EnblWrProtOfPric1230pric12302216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group0of_sgpiomaster0privilege(
        &mut self,
    ) -> EnblWrGroup0ofSgpiomaster0privilegeW<PricIo230Spec> {
        EnblWrGroup0ofSgpiomaster0privilegeW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group1of_sgpiomaster0privilege(
        &mut self,
    ) -> EnblWrGroup1ofSgpiomaster0privilegeW<PricIo230Spec> {
        EnblWrGroup1ofSgpiomaster0privilegeW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group2of_sgpiomaster0privilege(
        &mut self,
    ) -> EnblWrGroup2ofSgpiomaster0privilegeW<PricIo230Spec> {
        EnblWrGroup2ofSgpiomaster0privilegeW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group3of_sgpiomaster0privilege(
        &mut self,
    ) -> EnblWrGroup3ofSgpiomaster0privilegeW<PricIo230Spec> {
        EnblWrGroup3ofSgpiomaster0privilegeW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group4of_sgpiomaster0privilege(
        &mut self,
    ) -> EnblWrGroup4ofSgpiomaster0privilegeW<PricIo230Spec> {
        EnblWrGroup4ofSgpiomaster0privilegeW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group5of_sgpiomaster0privilege(
        &mut self,
    ) -> EnblWrGroup5ofSgpiomaster0privilegeW<PricIo230Spec> {
        EnblWrGroup5ofSgpiomaster0privilegeW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1230PRIC1_230\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1230pric12302924(
        &mut self,
    ) -> EnblRstToleranceOfPric1230pric12302924W<PricIo230Spec> {
        EnblRstToleranceOfPric1230pric12302924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1230PRIC1_230\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1230pric12303024(
        &mut self,
    ) -> EnblWrProtOfPric1230pric12303024W<PricIo230Spec> {
        EnblWrProtOfPric1230pric12303024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#12\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io230::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io230::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo230Spec;
impl crate::RegisterSpec for PricIo230Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io230::R`](R) reader structure"]
impl crate::Readable for PricIo230Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io230::W`](W) writer structure"]
impl crate::Writable for PricIo230Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO230 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo230Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
