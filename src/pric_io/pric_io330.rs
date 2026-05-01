#[doc = "Register `PRIC_IO330` reader"]
pub type R = crate::R<PricIo330Spec>;
#[doc = "Register `PRIC_IO330` writer"]
pub type W = crate::W<PricIo330Spec>;
#[doc = "Field `EnblReadGroup0OfGPIO` reader - Enable Read Group #0 of GPIO"]
pub type EnblReadGroup0ofGpioR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfGPIO` writer - Enable Read Group #0 of GPIO"]
pub type EnblReadGroup0ofGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfGPIO` reader - Enable Read Group #1 of GPIO"]
pub type EnblReadGroup1ofGpioR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfGPIO` writer - Enable Read Group #1 of GPIO"]
pub type EnblReadGroup1ofGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfGPIO` reader - Enable Read Group #2 of GPIO"]
pub type EnblReadGroup2ofGpioR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfGPIO` writer - Enable Read Group #2 of GPIO"]
pub type EnblReadGroup2ofGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfGPIO` reader - Enable Read Group #3 of GPIO"]
pub type EnblReadGroup3ofGpioR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfGPIO` writer - Enable Read Group #3 of GPIO"]
pub type EnblReadGroup3ofGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfGPIO` reader - Enable Read Group #4 of GPIO"]
pub type EnblReadGroup4ofGpioR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfGPIO` writer - Enable Read Group #4 of GPIO"]
pub type EnblReadGroup4ofGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfGPIO` reader - Enable Read Group #5 of GPIO"]
pub type EnblReadGroup5ofGpioR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfGPIO` writer - Enable Read Group #5 of GPIO"]
pub type EnblReadGroup5ofGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1330PRIC1_330\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1330pric13300500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1330pric13300500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1330pric13300500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1330PRIC13300500` reader - Enable Reset Tolerance of PRIC1330PRIC1_330\\[05:00\\]"]
pub type EnblRstToleranceOfPric1330pric13300500R =
    crate::BitReader<EnblRstToleranceOfPric1330pric13300500>;
impl EnblRstToleranceOfPric1330pric13300500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1330pric13300500 {
        match self.bits {
            false => EnblRstToleranceOfPric1330pric13300500::ResetBySrst,
            true => EnblRstToleranceOfPric1330pric13300500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1330pric13300500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1330pric13300500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1330PRIC13300500` writer - Enable Reset Tolerance of PRIC1330PRIC1_330\\[05:00\\]"]
pub type EnblRstToleranceOfPric1330pric13300500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1330pric13300500>;
impl<'a, REG> EnblRstToleranceOfPric1330pric13300500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1330pric13300500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1330pric13300500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1330PRIC13300600` reader - Enable Write Protection of PRIC1330PRIC1_330\\[06:00\\]"]
pub type EnblWrProtOfPric1330pric13300600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1330PRIC13300600` writer - Enable Write Protection of PRIC1330PRIC1_330\\[06:00\\]"]
pub type EnblWrProtOfPric1330pric13300600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfGPIOPrivilege` reader - Enable Read Group #0 of GPIO Privilege"]
pub type EnblReadGroup0ofGpioprivilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfGPIOPrivilege` writer - Enable Read Group #0 of GPIO Privilege"]
pub type EnblReadGroup0ofGpioprivilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfGPIOPrivilege` reader - Enable Read Group #1 of GPIO Privilege"]
pub type EnblReadGroup1ofGpioprivilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfGPIOPrivilege` writer - Enable Read Group #1 of GPIO Privilege"]
pub type EnblReadGroup1ofGpioprivilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfGPIOPrivilege` reader - Enable Read Group #2 of GPIO Privilege"]
pub type EnblReadGroup2ofGpioprivilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfGPIOPrivilege` writer - Enable Read Group #2 of GPIO Privilege"]
pub type EnblReadGroup2ofGpioprivilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfGPIOPrivilege` reader - Enable Read Group #3 of GPIO Privilege"]
pub type EnblReadGroup3ofGpioprivilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfGPIOPrivilege` writer - Enable Read Group #3 of GPIO Privilege"]
pub type EnblReadGroup3ofGpioprivilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfGPIOPrivilege` reader - Enable Read Group #4 of GPIO Privilege"]
pub type EnblReadGroup4ofGpioprivilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfGPIOPrivilege` writer - Enable Read Group #4 of GPIO Privilege"]
pub type EnblReadGroup4ofGpioprivilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfGPIOPrivilege` reader - Enable Read Group #5 of GPIO Privilege"]
pub type EnblReadGroup5ofGpioprivilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfGPIOPrivilege` writer - Enable Read Group #5 of GPIO Privilege"]
pub type EnblReadGroup5ofGpioprivilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1330PRIC1_330\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1330pric13301308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1330pric13301308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1330pric13301308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1330PRIC13301308` reader - Enable Reset Tolerance of PRIC1330PRIC1_330\\[13:08\\]"]
pub type EnblRstToleranceOfPric1330pric13301308R =
    crate::BitReader<EnblRstToleranceOfPric1330pric13301308>;
impl EnblRstToleranceOfPric1330pric13301308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1330pric13301308 {
        match self.bits {
            false => EnblRstToleranceOfPric1330pric13301308::ResetBySrst,
            true => EnblRstToleranceOfPric1330pric13301308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1330pric13301308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1330pric13301308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1330PRIC13301308` writer - Enable Reset Tolerance of PRIC1330PRIC1_330\\[13:08\\]"]
pub type EnblRstToleranceOfPric1330pric13301308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1330pric13301308>;
impl<'a, REG> EnblRstToleranceOfPric1330pric13301308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1330pric13301308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1330pric13301308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1330PRIC13301408` reader - Enable Write Protection of PRIC1330PRIC1_330\\[14:08\\]"]
pub type EnblWrProtOfPric1330pric13301408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1330PRIC13301408` writer - Enable Write Protection of PRIC1330PRIC1_330\\[14:08\\]"]
pub type EnblWrProtOfPric1330pric13301408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfSGPIOMaster0` reader - Enable Read Group #0 of SGPIO Master 0"]
pub type EnblReadGroup0ofSgpiomaster0R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfSGPIOMaster0` writer - Enable Read Group #0 of SGPIO Master 0"]
pub type EnblReadGroup0ofSgpiomaster0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfSGPIOMaster0` reader - Enable Read Group #1 of SGPIO Master 0"]
pub type EnblReadGroup1ofSgpiomaster0R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfSGPIOMaster0` writer - Enable Read Group #1 of SGPIO Master 0"]
pub type EnblReadGroup1ofSgpiomaster0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfSGPIOMaster0` reader - Enable Read Group #2 of SGPIO Master 0"]
pub type EnblReadGroup2ofSgpiomaster0R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfSGPIOMaster0` writer - Enable Read Group #2 of SGPIO Master 0"]
pub type EnblReadGroup2ofSgpiomaster0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfSGPIOMaster0` reader - Enable Read Group #3 of SGPIO Master 0"]
pub type EnblReadGroup3ofSgpiomaster0R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfSGPIOMaster0` writer - Enable Read Group #3 of SGPIO Master 0"]
pub type EnblReadGroup3ofSgpiomaster0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfSGPIOMaster0` reader - Enable Read Group #4 of SGPIO Master 0"]
pub type EnblReadGroup4ofSgpiomaster0R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfSGPIOMaster0` writer - Enable Read Group #4 of SGPIO Master 0"]
pub type EnblReadGroup4ofSgpiomaster0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfSGPIOMaster0` reader - Enable Read Group #5 of SGPIO Master 0"]
pub type EnblReadGroup5ofSgpiomaster0R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfSGPIOMaster0` writer - Enable Read Group #5 of SGPIO Master 0"]
pub type EnblReadGroup5ofSgpiomaster0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1330PRIC1_330\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1330pric13302116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1330pric13302116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1330pric13302116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1330PRIC13302116` reader - Enable Reset Tolerance of PRIC1330PRIC1_330\\[21:16\\]"]
pub type EnblRstToleranceOfPric1330pric13302116R =
    crate::BitReader<EnblRstToleranceOfPric1330pric13302116>;
impl EnblRstToleranceOfPric1330pric13302116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1330pric13302116 {
        match self.bits {
            false => EnblRstToleranceOfPric1330pric13302116::ResetBySrst,
            true => EnblRstToleranceOfPric1330pric13302116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1330pric13302116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1330pric13302116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1330PRIC13302116` writer - Enable Reset Tolerance of PRIC1330PRIC1_330\\[21:16\\]"]
pub type EnblRstToleranceOfPric1330pric13302116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1330pric13302116>;
impl<'a, REG> EnblRstToleranceOfPric1330pric13302116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1330pric13302116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1330pric13302116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1330PRIC13302216` reader - Enable Write Protection of PRIC1330PRIC1_330\\[22:16\\]"]
pub type EnblWrProtOfPric1330pric13302216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1330PRIC13302216` writer - Enable Write Protection of PRIC1330PRIC1_330\\[22:16\\]"]
pub type EnblWrProtOfPric1330pric13302216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfSGPIOMaster0Privilege` reader - Enable Read Group #0 of SGPIO Master 0 Privilege"]
pub type EnblReadGroup0ofSgpiomaster0privilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfSGPIOMaster0Privilege` writer - Enable Read Group #0 of SGPIO Master 0 Privilege"]
pub type EnblReadGroup0ofSgpiomaster0privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfSGPIOMaster0Privilege` reader - Enable Read Group #1 of SGPIO Master 0 Privilege"]
pub type EnblReadGroup1ofSgpiomaster0privilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfSGPIOMaster0Privilege` writer - Enable Read Group #1 of SGPIO Master 0 Privilege"]
pub type EnblReadGroup1ofSgpiomaster0privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfSGPIOMaster0Privilege` reader - Enable Read Group #2 of SGPIO Master 0 Privilege"]
pub type EnblReadGroup2ofSgpiomaster0privilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfSGPIOMaster0Privilege` writer - Enable Read Group #2 of SGPIO Master 0 Privilege"]
pub type EnblReadGroup2ofSgpiomaster0privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfSGPIOMaster0Privilege` reader - Enable Read Group #3 of SGPIO Master 0 Privilege"]
pub type EnblReadGroup3ofSgpiomaster0privilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfSGPIOMaster0Privilege` writer - Enable Read Group #3 of SGPIO Master 0 Privilege"]
pub type EnblReadGroup3ofSgpiomaster0privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfSGPIOMaster0Privilege` reader - Enable Read Group #4 of SGPIO Master 0 Privilege"]
pub type EnblReadGroup4ofSgpiomaster0privilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfSGPIOMaster0Privilege` writer - Enable Read Group #4 of SGPIO Master 0 Privilege"]
pub type EnblReadGroup4ofSgpiomaster0privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfSGPIOMaster0Privilege` reader - Enable Read Group #5 of SGPIO Master 0 Privilege"]
pub type EnblReadGroup5ofSgpiomaster0privilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfSGPIOMaster0Privilege` writer - Enable Read Group #5 of SGPIO Master 0 Privilege"]
pub type EnblReadGroup5ofSgpiomaster0privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1330PRIC1_330\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1330pric13302924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1330pric13302924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1330pric13302924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1330PRIC13302924` reader - Enable Reset Tolerance of PRIC1330PRIC1_330\\[29:24\\]"]
pub type EnblRstToleranceOfPric1330pric13302924R =
    crate::BitReader<EnblRstToleranceOfPric1330pric13302924>;
impl EnblRstToleranceOfPric1330pric13302924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1330pric13302924 {
        match self.bits {
            false => EnblRstToleranceOfPric1330pric13302924::ResetBySrst,
            true => EnblRstToleranceOfPric1330pric13302924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1330pric13302924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1330pric13302924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1330PRIC13302924` writer - Enable Reset Tolerance of PRIC1330PRIC1_330\\[29:24\\]"]
pub type EnblRstToleranceOfPric1330pric13302924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1330pric13302924>;
impl<'a, REG> EnblRstToleranceOfPric1330pric13302924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1330pric13302924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1330pric13302924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1330PRIC13303024` reader - Enable Write Protection of PRIC1330PRIC1_330\\[30:24\\]"]
pub type EnblWrProtOfPric1330pric13303024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1330PRIC13303024` writer - Enable Write Protection of PRIC1330PRIC1_330\\[30:24\\]"]
pub type EnblWrProtOfPric1330pric13303024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of GPIO"]
    #[inline(always)]
    pub fn enbl_read_group0of_gpio(&self) -> EnblReadGroup0ofGpioR {
        EnblReadGroup0ofGpioR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of GPIO"]
    #[inline(always)]
    pub fn enbl_read_group1of_gpio(&self) -> EnblReadGroup1ofGpioR {
        EnblReadGroup1ofGpioR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of GPIO"]
    #[inline(always)]
    pub fn enbl_read_group2of_gpio(&self) -> EnblReadGroup2ofGpioR {
        EnblReadGroup2ofGpioR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of GPIO"]
    #[inline(always)]
    pub fn enbl_read_group3of_gpio(&self) -> EnblReadGroup3ofGpioR {
        EnblReadGroup3ofGpioR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of GPIO"]
    #[inline(always)]
    pub fn enbl_read_group4of_gpio(&self) -> EnblReadGroup4ofGpioR {
        EnblReadGroup4ofGpioR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of GPIO"]
    #[inline(always)]
    pub fn enbl_read_group5of_gpio(&self) -> EnblReadGroup5ofGpioR {
        EnblReadGroup5ofGpioR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1330PRIC1_330\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1330pric13300500(
        &self,
    ) -> EnblRstToleranceOfPric1330pric13300500R {
        EnblRstToleranceOfPric1330pric13300500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1330PRIC1_330\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1330pric13300600(&self) -> EnblWrProtOfPric1330pric13300600R {
        EnblWrProtOfPric1330pric13300600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_read_group0of_gpioprivilege(&self) -> EnblReadGroup0ofGpioprivilegeR {
        EnblReadGroup0ofGpioprivilegeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_read_group1of_gpioprivilege(&self) -> EnblReadGroup1ofGpioprivilegeR {
        EnblReadGroup1ofGpioprivilegeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_read_group2of_gpioprivilege(&self) -> EnblReadGroup2ofGpioprivilegeR {
        EnblReadGroup2ofGpioprivilegeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_read_group3of_gpioprivilege(&self) -> EnblReadGroup3ofGpioprivilegeR {
        EnblReadGroup3ofGpioprivilegeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_read_group4of_gpioprivilege(&self) -> EnblReadGroup4ofGpioprivilegeR {
        EnblReadGroup4ofGpioprivilegeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_read_group5of_gpioprivilege(&self) -> EnblReadGroup5ofGpioprivilegeR {
        EnblReadGroup5ofGpioprivilegeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1330PRIC1_330\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1330pric13301308(
        &self,
    ) -> EnblRstToleranceOfPric1330pric13301308R {
        EnblRstToleranceOfPric1330pric13301308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1330PRIC1_330\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1330pric13301408(&self) -> EnblWrProtOfPric1330pric13301408R {
        EnblWrProtOfPric1330pric13301408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_read_group0of_sgpiomaster0(&self) -> EnblReadGroup0ofSgpiomaster0R {
        EnblReadGroup0ofSgpiomaster0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_read_group1of_sgpiomaster0(&self) -> EnblReadGroup1ofSgpiomaster0R {
        EnblReadGroup1ofSgpiomaster0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_read_group2of_sgpiomaster0(&self) -> EnblReadGroup2ofSgpiomaster0R {
        EnblReadGroup2ofSgpiomaster0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_read_group3of_sgpiomaster0(&self) -> EnblReadGroup3ofSgpiomaster0R {
        EnblReadGroup3ofSgpiomaster0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_read_group4of_sgpiomaster0(&self) -> EnblReadGroup4ofSgpiomaster0R {
        EnblReadGroup4ofSgpiomaster0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_read_group5of_sgpiomaster0(&self) -> EnblReadGroup5ofSgpiomaster0R {
        EnblReadGroup5ofSgpiomaster0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1330PRIC1_330\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1330pric13302116(
        &self,
    ) -> EnblRstToleranceOfPric1330pric13302116R {
        EnblRstToleranceOfPric1330pric13302116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1330PRIC1_330\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1330pric13302216(&self) -> EnblWrProtOfPric1330pric13302216R {
        EnblWrProtOfPric1330pric13302216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group0of_sgpiomaster0privilege(
        &self,
    ) -> EnblReadGroup0ofSgpiomaster0privilegeR {
        EnblReadGroup0ofSgpiomaster0privilegeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group1of_sgpiomaster0privilege(
        &self,
    ) -> EnblReadGroup1ofSgpiomaster0privilegeR {
        EnblReadGroup1ofSgpiomaster0privilegeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group2of_sgpiomaster0privilege(
        &self,
    ) -> EnblReadGroup2ofSgpiomaster0privilegeR {
        EnblReadGroup2ofSgpiomaster0privilegeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group3of_sgpiomaster0privilege(
        &self,
    ) -> EnblReadGroup3ofSgpiomaster0privilegeR {
        EnblReadGroup3ofSgpiomaster0privilegeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group4of_sgpiomaster0privilege(
        &self,
    ) -> EnblReadGroup4ofSgpiomaster0privilegeR {
        EnblReadGroup4ofSgpiomaster0privilegeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group5of_sgpiomaster0privilege(
        &self,
    ) -> EnblReadGroup5ofSgpiomaster0privilegeR {
        EnblReadGroup5ofSgpiomaster0privilegeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1330PRIC1_330\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1330pric13302924(
        &self,
    ) -> EnblRstToleranceOfPric1330pric13302924R {
        EnblRstToleranceOfPric1330pric13302924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1330PRIC1_330\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1330pric13303024(&self) -> EnblWrProtOfPric1330pric13303024R {
        EnblWrProtOfPric1330pric13303024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of GPIO"]
    #[inline(always)]
    pub fn enbl_read_group0of_gpio(&mut self) -> EnblReadGroup0ofGpioW<PricIo330Spec> {
        EnblReadGroup0ofGpioW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of GPIO"]
    #[inline(always)]
    pub fn enbl_read_group1of_gpio(&mut self) -> EnblReadGroup1ofGpioW<PricIo330Spec> {
        EnblReadGroup1ofGpioW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of GPIO"]
    #[inline(always)]
    pub fn enbl_read_group2of_gpio(&mut self) -> EnblReadGroup2ofGpioW<PricIo330Spec> {
        EnblReadGroup2ofGpioW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of GPIO"]
    #[inline(always)]
    pub fn enbl_read_group3of_gpio(&mut self) -> EnblReadGroup3ofGpioW<PricIo330Spec> {
        EnblReadGroup3ofGpioW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of GPIO"]
    #[inline(always)]
    pub fn enbl_read_group4of_gpio(&mut self) -> EnblReadGroup4ofGpioW<PricIo330Spec> {
        EnblReadGroup4ofGpioW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of GPIO"]
    #[inline(always)]
    pub fn enbl_read_group5of_gpio(&mut self) -> EnblReadGroup5ofGpioW<PricIo330Spec> {
        EnblReadGroup5ofGpioW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1330PRIC1_330\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1330pric13300500(
        &mut self,
    ) -> EnblRstToleranceOfPric1330pric13300500W<PricIo330Spec> {
        EnblRstToleranceOfPric1330pric13300500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1330PRIC1_330\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1330pric13300600(
        &mut self,
    ) -> EnblWrProtOfPric1330pric13300600W<PricIo330Spec> {
        EnblWrProtOfPric1330pric13300600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_read_group0of_gpioprivilege(
        &mut self,
    ) -> EnblReadGroup0ofGpioprivilegeW<PricIo330Spec> {
        EnblReadGroup0ofGpioprivilegeW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_read_group1of_gpioprivilege(
        &mut self,
    ) -> EnblReadGroup1ofGpioprivilegeW<PricIo330Spec> {
        EnblReadGroup1ofGpioprivilegeW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_read_group2of_gpioprivilege(
        &mut self,
    ) -> EnblReadGroup2ofGpioprivilegeW<PricIo330Spec> {
        EnblReadGroup2ofGpioprivilegeW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_read_group3of_gpioprivilege(
        &mut self,
    ) -> EnblReadGroup3ofGpioprivilegeW<PricIo330Spec> {
        EnblReadGroup3ofGpioprivilegeW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_read_group4of_gpioprivilege(
        &mut self,
    ) -> EnblReadGroup4ofGpioprivilegeW<PricIo330Spec> {
        EnblReadGroup4ofGpioprivilegeW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of GPIO Privilege"]
    #[inline(always)]
    pub fn enbl_read_group5of_gpioprivilege(
        &mut self,
    ) -> EnblReadGroup5ofGpioprivilegeW<PricIo330Spec> {
        EnblReadGroup5ofGpioprivilegeW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1330PRIC1_330\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1330pric13301308(
        &mut self,
    ) -> EnblRstToleranceOfPric1330pric13301308W<PricIo330Spec> {
        EnblRstToleranceOfPric1330pric13301308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1330PRIC1_330\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1330pric13301408(
        &mut self,
    ) -> EnblWrProtOfPric1330pric13301408W<PricIo330Spec> {
        EnblWrProtOfPric1330pric13301408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_read_group0of_sgpiomaster0(
        &mut self,
    ) -> EnblReadGroup0ofSgpiomaster0W<PricIo330Spec> {
        EnblReadGroup0ofSgpiomaster0W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_read_group1of_sgpiomaster0(
        &mut self,
    ) -> EnblReadGroup1ofSgpiomaster0W<PricIo330Spec> {
        EnblReadGroup1ofSgpiomaster0W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_read_group2of_sgpiomaster0(
        &mut self,
    ) -> EnblReadGroup2ofSgpiomaster0W<PricIo330Spec> {
        EnblReadGroup2ofSgpiomaster0W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_read_group3of_sgpiomaster0(
        &mut self,
    ) -> EnblReadGroup3ofSgpiomaster0W<PricIo330Spec> {
        EnblReadGroup3ofSgpiomaster0W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_read_group4of_sgpiomaster0(
        &mut self,
    ) -> EnblReadGroup4ofSgpiomaster0W<PricIo330Spec> {
        EnblReadGroup4ofSgpiomaster0W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of SGPIO Master 0"]
    #[inline(always)]
    pub fn enbl_read_group5of_sgpiomaster0(
        &mut self,
    ) -> EnblReadGroup5ofSgpiomaster0W<PricIo330Spec> {
        EnblReadGroup5ofSgpiomaster0W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1330PRIC1_330\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1330pric13302116(
        &mut self,
    ) -> EnblRstToleranceOfPric1330pric13302116W<PricIo330Spec> {
        EnblRstToleranceOfPric1330pric13302116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1330PRIC1_330\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1330pric13302216(
        &mut self,
    ) -> EnblWrProtOfPric1330pric13302216W<PricIo330Spec> {
        EnblWrProtOfPric1330pric13302216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group0of_sgpiomaster0privilege(
        &mut self,
    ) -> EnblReadGroup0ofSgpiomaster0privilegeW<PricIo330Spec> {
        EnblReadGroup0ofSgpiomaster0privilegeW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group1of_sgpiomaster0privilege(
        &mut self,
    ) -> EnblReadGroup1ofSgpiomaster0privilegeW<PricIo330Spec> {
        EnblReadGroup1ofSgpiomaster0privilegeW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group2of_sgpiomaster0privilege(
        &mut self,
    ) -> EnblReadGroup2ofSgpiomaster0privilegeW<PricIo330Spec> {
        EnblReadGroup2ofSgpiomaster0privilegeW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group3of_sgpiomaster0privilege(
        &mut self,
    ) -> EnblReadGroup3ofSgpiomaster0privilegeW<PricIo330Spec> {
        EnblReadGroup3ofSgpiomaster0privilegeW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group4of_sgpiomaster0privilege(
        &mut self,
    ) -> EnblReadGroup4ofSgpiomaster0privilegeW<PricIo330Spec> {
        EnblReadGroup4ofSgpiomaster0privilegeW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of SGPIO Master 0 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group5of_sgpiomaster0privilege(
        &mut self,
    ) -> EnblReadGroup5ofSgpiomaster0privilegeW<PricIo330Spec> {
        EnblReadGroup5ofSgpiomaster0privilegeW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1330PRIC1_330\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1330pric13302924(
        &mut self,
    ) -> EnblRstToleranceOfPric1330pric13302924W<PricIo330Spec> {
        EnblRstToleranceOfPric1330pric13302924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1330PRIC1_330\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1330pric13303024(
        &mut self,
    ) -> EnblWrProtOfPric1330pric13303024W<PricIo330Spec> {
        EnblWrProtOfPric1330pric13303024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#12\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io330::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io330::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo330Spec;
impl crate::RegisterSpec for PricIo330Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io330::R`](R) reader structure"]
impl crate::Readable for PricIo330Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io330::W`](W) writer structure"]
impl crate::Writable for PricIo330Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO330 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo330Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
