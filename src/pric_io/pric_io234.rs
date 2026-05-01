#[doc = "Register `PRIC_IO234` reader"]
pub type R = crate::R<PricIo234Spec>;
#[doc = "Register `PRIC_IO234` writer"]
pub type W = crate::W<PricIo234Spec>;
#[doc = "Field `EnblWrGroup0OfSGPIOMaster1` reader - Enable Write Group #0 of SGPIO Master 1"]
pub type EnblWrGroup0ofSgpiomaster1R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfSGPIOMaster1` writer - Enable Write Group #0 of SGPIO Master 1"]
pub type EnblWrGroup0ofSgpiomaster1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfSGPIOMaster1` reader - Enable Write Group #1 of SGPIO Master 1"]
pub type EnblWrGroup1ofSgpiomaster1R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfSGPIOMaster1` writer - Enable Write Group #1 of SGPIO Master 1"]
pub type EnblWrGroup1ofSgpiomaster1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfSGPIOMaster1` reader - Enable Write Group #2 of SGPIO Master 1"]
pub type EnblWrGroup2ofSgpiomaster1R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfSGPIOMaster1` writer - Enable Write Group #2 of SGPIO Master 1"]
pub type EnblWrGroup2ofSgpiomaster1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfSGPIOMaster1` reader - Enable Write Group #3 of SGPIO Master 1"]
pub type EnblWrGroup3ofSgpiomaster1R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfSGPIOMaster1` writer - Enable Write Group #3 of SGPIO Master 1"]
pub type EnblWrGroup3ofSgpiomaster1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfSGPIOMaster1` reader - Enable Write Group #4 of SGPIO Master 1"]
pub type EnblWrGroup4ofSgpiomaster1R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfSGPIOMaster1` writer - Enable Write Group #4 of SGPIO Master 1"]
pub type EnblWrGroup4ofSgpiomaster1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfSGPIOMaster1` reader - Enable Write Group #5 of SGPIO Master 1"]
pub type EnblWrGroup5ofSgpiomaster1R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfSGPIOMaster1` writer - Enable Write Group #5 of SGPIO Master 1"]
pub type EnblWrGroup5ofSgpiomaster1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1234PRIC1_234\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1234pric12340500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1234pric12340500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1234pric12340500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1234PRIC12340500` reader - Enable Reset Tolerance of PRIC1234PRIC1_234\\[05:00\\]"]
pub type EnblRstToleranceOfPric1234pric12340500R =
    crate::BitReader<EnblRstToleranceOfPric1234pric12340500>;
impl EnblRstToleranceOfPric1234pric12340500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1234pric12340500 {
        match self.bits {
            false => EnblRstToleranceOfPric1234pric12340500::ResetBySrst,
            true => EnblRstToleranceOfPric1234pric12340500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1234pric12340500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1234pric12340500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1234PRIC12340500` writer - Enable Reset Tolerance of PRIC1234PRIC1_234\\[05:00\\]"]
pub type EnblRstToleranceOfPric1234pric12340500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1234pric12340500>;
impl<'a, REG> EnblRstToleranceOfPric1234pric12340500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1234pric12340500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1234pric12340500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1234PRIC12340600` reader - Enable Write Protection of PRIC1234PRIC1_234\\[06:00\\]"]
pub type EnblWrProtOfPric1234pric12340600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1234PRIC12340600` writer - Enable Write Protection of PRIC1234PRIC1_234\\[06:00\\]"]
pub type EnblWrProtOfPric1234pric12340600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfSGPIOMaster1Privilege` reader - Enable Write Group #0 of SGPIO Master 1 Privilege"]
pub type EnblWrGroup0ofSgpiomaster1privilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfSGPIOMaster1Privilege` writer - Enable Write Group #0 of SGPIO Master 1 Privilege"]
pub type EnblWrGroup0ofSgpiomaster1privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfSGPIOMaster1Privilege` reader - Enable Write Group #1 of SGPIO Master 1 Privilege"]
pub type EnblWrGroup1ofSgpiomaster1privilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfSGPIOMaster1Privilege` writer - Enable Write Group #1 of SGPIO Master 1 Privilege"]
pub type EnblWrGroup1ofSgpiomaster1privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfSGPIOMaster1Privilege` reader - Enable Write Group #2 of SGPIO Master 1 Privilege"]
pub type EnblWrGroup2ofSgpiomaster1privilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfSGPIOMaster1Privilege` writer - Enable Write Group #2 of SGPIO Master 1 Privilege"]
pub type EnblWrGroup2ofSgpiomaster1privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfSGPIOMaster1Privilege` reader - Enable Write Group #3 of SGPIO Master 1 Privilege"]
pub type EnblWrGroup3ofSgpiomaster1privilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfSGPIOMaster1Privilege` writer - Enable Write Group #3 of SGPIO Master 1 Privilege"]
pub type EnblWrGroup3ofSgpiomaster1privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfSGPIOMaster1Privilege` reader - Enable Write Group #4 of SGPIO Master 1 Privilege"]
pub type EnblWrGroup4ofSgpiomaster1privilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfSGPIOMaster1Privilege` writer - Enable Write Group #4 of SGPIO Master 1 Privilege"]
pub type EnblWrGroup4ofSgpiomaster1privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfSGPIOMaster1Privilege` reader - Enable Write Group #5 of SGPIO Master 1 Privilege"]
pub type EnblWrGroup5ofSgpiomaster1privilegeR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfSGPIOMaster1Privilege` writer - Enable Write Group #5 of SGPIO Master 1 Privilege"]
pub type EnblWrGroup5ofSgpiomaster1privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1234PRIC1_234\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1234pric12341308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1234pric12341308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1234pric12341308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1234PRIC12341308` reader - Enable Reset Tolerance of PRIC1234PRIC1_234\\[13:08\\]"]
pub type EnblRstToleranceOfPric1234pric12341308R =
    crate::BitReader<EnblRstToleranceOfPric1234pric12341308>;
impl EnblRstToleranceOfPric1234pric12341308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1234pric12341308 {
        match self.bits {
            false => EnblRstToleranceOfPric1234pric12341308::ResetBySrst,
            true => EnblRstToleranceOfPric1234pric12341308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1234pric12341308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1234pric12341308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1234PRIC12341308` writer - Enable Reset Tolerance of PRIC1234PRIC1_234\\[13:08\\]"]
pub type EnblRstToleranceOfPric1234pric12341308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1234pric12341308>;
impl<'a, REG> EnblRstToleranceOfPric1234pric12341308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1234pric12341308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1234pric12341308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1234PRIC12341408` reader - Enable Write Protection of PRIC1234PRIC1_234\\[14:08\\]"]
pub type EnblWrProtOfPric1234pric12341408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1234PRIC12341408` writer - Enable Write Protection of PRIC1234PRIC1_234\\[14:08\\]"]
pub type EnblWrProtOfPric1234pric12341408W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `EnblWrGroup0OfI2CGlobal` reader - Enable Write Group #0 of I2C Global"]
pub type EnblWrGroup0ofI2cglobalR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfI2CGlobal` writer - Enable Write Group #0 of I2C Global"]
pub type EnblWrGroup0ofI2cglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfI2CGlobal` reader - Enable Write Group #1 of I2C Global"]
pub type EnblWrGroup1ofI2cglobalR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfI2CGlobal` writer - Enable Write Group #1 of I2C Global"]
pub type EnblWrGroup1ofI2cglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfI2CGlobal` reader - Enable Write Group #2 of I2C Global"]
pub type EnblWrGroup2ofI2cglobalR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfI2CGlobal` writer - Enable Write Group #2 of I2C Global"]
pub type EnblWrGroup2ofI2cglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfI2CGlobal` reader - Enable Write Group #3 of I2C Global"]
pub type EnblWrGroup3ofI2cglobalR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfI2CGlobal` writer - Enable Write Group #3 of I2C Global"]
pub type EnblWrGroup3ofI2cglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfI2CGlobal` reader - Enable Write Group #4 of I2C Global"]
pub type EnblWrGroup4ofI2cglobalR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfI2CGlobal` writer - Enable Write Group #4 of I2C Global"]
pub type EnblWrGroup4ofI2cglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfI2CGlobal` reader - Enable Write Group #5 of I2C Global"]
pub type EnblWrGroup5ofI2cglobalR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfI2CGlobal` writer - Enable Write Group #5 of I2C Global"]
pub type EnblWrGroup5ofI2cglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1234PRIC1_234\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1234pric12342924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1234pric12342924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1234pric12342924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1234PRIC12342924` reader - Enable Reset Tolerance of PRIC1234PRIC1_234\\[29:24\\]"]
pub type EnblRstToleranceOfPric1234pric12342924R =
    crate::BitReader<EnblRstToleranceOfPric1234pric12342924>;
impl EnblRstToleranceOfPric1234pric12342924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1234pric12342924 {
        match self.bits {
            false => EnblRstToleranceOfPric1234pric12342924::ResetBySrst,
            true => EnblRstToleranceOfPric1234pric12342924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1234pric12342924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1234pric12342924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1234PRIC12342924` writer - Enable Reset Tolerance of PRIC1234PRIC1_234\\[29:24\\]"]
pub type EnblRstToleranceOfPric1234pric12342924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1234pric12342924>;
impl<'a, REG> EnblRstToleranceOfPric1234pric12342924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1234pric12342924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1234pric12342924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1234PRIC12343024` reader - Enable Write Protection of PRIC1234PRIC1_234\\[30:24\\]"]
pub type EnblWrProtOfPric1234pric12343024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1234PRIC12343024` writer - Enable Write Protection of PRIC1234PRIC1_234\\[30:24\\]"]
pub type EnblWrProtOfPric1234pric12343024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_sgpiomaster1(&self) -> EnblWrGroup0ofSgpiomaster1R {
        EnblWrGroup0ofSgpiomaster1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_sgpiomaster1(&self) -> EnblWrGroup1ofSgpiomaster1R {
        EnblWrGroup1ofSgpiomaster1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_sgpiomaster1(&self) -> EnblWrGroup2ofSgpiomaster1R {
        EnblWrGroup2ofSgpiomaster1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_sgpiomaster1(&self) -> EnblWrGroup3ofSgpiomaster1R {
        EnblWrGroup3ofSgpiomaster1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_sgpiomaster1(&self) -> EnblWrGroup4ofSgpiomaster1R {
        EnblWrGroup4ofSgpiomaster1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_sgpiomaster1(&self) -> EnblWrGroup5ofSgpiomaster1R {
        EnblWrGroup5ofSgpiomaster1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1234PRIC1_234\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1234pric12340500(
        &self,
    ) -> EnblRstToleranceOfPric1234pric12340500R {
        EnblRstToleranceOfPric1234pric12340500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1234PRIC1_234\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1234pric12340600(&self) -> EnblWrProtOfPric1234pric12340600R {
        EnblWrProtOfPric1234pric12340600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group0of_sgpiomaster1privilege(&self) -> EnblWrGroup0ofSgpiomaster1privilegeR {
        EnblWrGroup0ofSgpiomaster1privilegeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group1of_sgpiomaster1privilege(&self) -> EnblWrGroup1ofSgpiomaster1privilegeR {
        EnblWrGroup1ofSgpiomaster1privilegeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group2of_sgpiomaster1privilege(&self) -> EnblWrGroup2ofSgpiomaster1privilegeR {
        EnblWrGroup2ofSgpiomaster1privilegeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group3of_sgpiomaster1privilege(&self) -> EnblWrGroup3ofSgpiomaster1privilegeR {
        EnblWrGroup3ofSgpiomaster1privilegeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group4of_sgpiomaster1privilege(&self) -> EnblWrGroup4ofSgpiomaster1privilegeR {
        EnblWrGroup4ofSgpiomaster1privilegeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group5of_sgpiomaster1privilege(&self) -> EnblWrGroup5ofSgpiomaster1privilegeR {
        EnblWrGroup5ofSgpiomaster1privilegeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1234PRIC1_234\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1234pric12341308(
        &self,
    ) -> EnblRstToleranceOfPric1234pric12341308R {
        EnblRstToleranceOfPric1234pric12341308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1234PRIC1_234\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1234pric12341408(&self) -> EnblWrProtOfPric1234pric12341408R {
        EnblWrProtOfPric1234pric12341408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of I2C Global"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2cglobal(&self) -> EnblWrGroup0ofI2cglobalR {
        EnblWrGroup0ofI2cglobalR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of I2C Global"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2cglobal(&self) -> EnblWrGroup1ofI2cglobalR {
        EnblWrGroup1ofI2cglobalR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of I2C Global"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2cglobal(&self) -> EnblWrGroup2ofI2cglobalR {
        EnblWrGroup2ofI2cglobalR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of I2C Global"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2cglobal(&self) -> EnblWrGroup3ofI2cglobalR {
        EnblWrGroup3ofI2cglobalR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of I2C Global"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2cglobal(&self) -> EnblWrGroup4ofI2cglobalR {
        EnblWrGroup4ofI2cglobalR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of I2C Global"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2cglobal(&self) -> EnblWrGroup5ofI2cglobalR {
        EnblWrGroup5ofI2cglobalR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1234PRIC1_234\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1234pric12342924(
        &self,
    ) -> EnblRstToleranceOfPric1234pric12342924R {
        EnblRstToleranceOfPric1234pric12342924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1234PRIC1_234\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1234pric12343024(&self) -> EnblWrProtOfPric1234pric12343024R {
        EnblWrProtOfPric1234pric12343024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_sgpiomaster1(&mut self) -> EnblWrGroup0ofSgpiomaster1W<PricIo234Spec> {
        EnblWrGroup0ofSgpiomaster1W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_sgpiomaster1(&mut self) -> EnblWrGroup1ofSgpiomaster1W<PricIo234Spec> {
        EnblWrGroup1ofSgpiomaster1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_sgpiomaster1(&mut self) -> EnblWrGroup2ofSgpiomaster1W<PricIo234Spec> {
        EnblWrGroup2ofSgpiomaster1W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_sgpiomaster1(&mut self) -> EnblWrGroup3ofSgpiomaster1W<PricIo234Spec> {
        EnblWrGroup3ofSgpiomaster1W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_sgpiomaster1(&mut self) -> EnblWrGroup4ofSgpiomaster1W<PricIo234Spec> {
        EnblWrGroup4ofSgpiomaster1W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_sgpiomaster1(&mut self) -> EnblWrGroup5ofSgpiomaster1W<PricIo234Spec> {
        EnblWrGroup5ofSgpiomaster1W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1234PRIC1_234\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1234pric12340500(
        &mut self,
    ) -> EnblRstToleranceOfPric1234pric12340500W<PricIo234Spec> {
        EnblRstToleranceOfPric1234pric12340500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1234PRIC1_234\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1234pric12340600(
        &mut self,
    ) -> EnblWrProtOfPric1234pric12340600W<PricIo234Spec> {
        EnblWrProtOfPric1234pric12340600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group0of_sgpiomaster1privilege(
        &mut self,
    ) -> EnblWrGroup0ofSgpiomaster1privilegeW<PricIo234Spec> {
        EnblWrGroup0ofSgpiomaster1privilegeW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group1of_sgpiomaster1privilege(
        &mut self,
    ) -> EnblWrGroup1ofSgpiomaster1privilegeW<PricIo234Spec> {
        EnblWrGroup1ofSgpiomaster1privilegeW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group2of_sgpiomaster1privilege(
        &mut self,
    ) -> EnblWrGroup2ofSgpiomaster1privilegeW<PricIo234Spec> {
        EnblWrGroup2ofSgpiomaster1privilegeW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group3of_sgpiomaster1privilege(
        &mut self,
    ) -> EnblWrGroup3ofSgpiomaster1privilegeW<PricIo234Spec> {
        EnblWrGroup3ofSgpiomaster1privilegeW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group4of_sgpiomaster1privilege(
        &mut self,
    ) -> EnblWrGroup4ofSgpiomaster1privilegeW<PricIo234Spec> {
        EnblWrGroup4ofSgpiomaster1privilegeW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_wr_group5of_sgpiomaster1privilege(
        &mut self,
    ) -> EnblWrGroup5ofSgpiomaster1privilegeW<PricIo234Spec> {
        EnblWrGroup5ofSgpiomaster1privilegeW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1234PRIC1_234\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1234pric12341308(
        &mut self,
    ) -> EnblRstToleranceOfPric1234pric12341308W<PricIo234Spec> {
        EnblRstToleranceOfPric1234pric12341308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1234PRIC1_234\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1234pric12341408(
        &mut self,
    ) -> EnblWrProtOfPric1234pric12341408W<PricIo234Spec> {
        EnblWrProtOfPric1234pric12341408W::new(self, 15)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo234Spec> {
        Reserved7W::new(self, 16)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo234Spec> {
        Reserved6W::new(self, 17)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo234Spec> {
        Reserved5W::new(self, 18)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo234Spec> {
        Reserved4W::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo234Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo234Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo234Spec> {
        Reserved1W::new(self, 22)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of I2C Global"]
    #[inline(always)]
    pub fn enbl_wr_group0of_i2cglobal(&mut self) -> EnblWrGroup0ofI2cglobalW<PricIo234Spec> {
        EnblWrGroup0ofI2cglobalW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of I2C Global"]
    #[inline(always)]
    pub fn enbl_wr_group1of_i2cglobal(&mut self) -> EnblWrGroup1ofI2cglobalW<PricIo234Spec> {
        EnblWrGroup1ofI2cglobalW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of I2C Global"]
    #[inline(always)]
    pub fn enbl_wr_group2of_i2cglobal(&mut self) -> EnblWrGroup2ofI2cglobalW<PricIo234Spec> {
        EnblWrGroup2ofI2cglobalW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of I2C Global"]
    #[inline(always)]
    pub fn enbl_wr_group3of_i2cglobal(&mut self) -> EnblWrGroup3ofI2cglobalW<PricIo234Spec> {
        EnblWrGroup3ofI2cglobalW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of I2C Global"]
    #[inline(always)]
    pub fn enbl_wr_group4of_i2cglobal(&mut self) -> EnblWrGroup4ofI2cglobalW<PricIo234Spec> {
        EnblWrGroup4ofI2cglobalW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of I2C Global"]
    #[inline(always)]
    pub fn enbl_wr_group5of_i2cglobal(&mut self) -> EnblWrGroup5ofI2cglobalW<PricIo234Spec> {
        EnblWrGroup5ofI2cglobalW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1234PRIC1_234\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1234pric12342924(
        &mut self,
    ) -> EnblRstToleranceOfPric1234pric12342924W<PricIo234Spec> {
        EnblRstToleranceOfPric1234pric12342924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1234PRIC1_234\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1234pric12343024(
        &mut self,
    ) -> EnblWrProtOfPric1234pric12343024W<PricIo234Spec> {
        EnblWrProtOfPric1234pric12343024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io234::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io234::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo234Spec;
impl crate::RegisterSpec for PricIo234Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io234::R`](R) reader structure"]
impl crate::Readable for PricIo234Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io234::W`](W) writer structure"]
impl crate::Writable for PricIo234Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO234 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo234Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
