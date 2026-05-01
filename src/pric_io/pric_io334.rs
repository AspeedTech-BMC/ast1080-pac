#[doc = "Register `PRIC_IO334` reader"]
pub type R = crate::R<PricIo334Spec>;
#[doc = "Register `PRIC_IO334` writer"]
pub type W = crate::W<PricIo334Spec>;
#[doc = "Field `EnblReadGroup0OfSGPIOMaster1` reader - Enable Read Group #0 of SGPIO Master 1"]
pub type EnblReadGroup0ofSgpiomaster1R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfSGPIOMaster1` writer - Enable Read Group #0 of SGPIO Master 1"]
pub type EnblReadGroup0ofSgpiomaster1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfSGPIOMaster1` reader - Enable Read Group #1 of SGPIO Master 1"]
pub type EnblReadGroup1ofSgpiomaster1R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfSGPIOMaster1` writer - Enable Read Group #1 of SGPIO Master 1"]
pub type EnblReadGroup1ofSgpiomaster1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfSGPIOMaster1` reader - Enable Read Group #2 of SGPIO Master 1"]
pub type EnblReadGroup2ofSgpiomaster1R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfSGPIOMaster1` writer - Enable Read Group #2 of SGPIO Master 1"]
pub type EnblReadGroup2ofSgpiomaster1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfSGPIOMaster1` reader - Enable Read Group #3 of SGPIO Master 1"]
pub type EnblReadGroup3ofSgpiomaster1R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfSGPIOMaster1` writer - Enable Read Group #3 of SGPIO Master 1"]
pub type EnblReadGroup3ofSgpiomaster1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfSGPIOMaster1` reader - Enable Read Group #4 of SGPIO Master 1"]
pub type EnblReadGroup4ofSgpiomaster1R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfSGPIOMaster1` writer - Enable Read Group #4 of SGPIO Master 1"]
pub type EnblReadGroup4ofSgpiomaster1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfSGPIOMaster1` reader - Enable Read Group #5 of SGPIO Master 1"]
pub type EnblReadGroup5ofSgpiomaster1R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfSGPIOMaster1` writer - Enable Read Group #5 of SGPIO Master 1"]
pub type EnblReadGroup5ofSgpiomaster1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1334PRIC1_334\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1334pric13340500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1334pric13340500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1334pric13340500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1334PRIC13340500` reader - Enable Reset Tolerance of PRIC1334PRIC1_334\\[05:00\\]"]
pub type EnblRstToleranceOfPric1334pric13340500R =
    crate::BitReader<EnblRstToleranceOfPric1334pric13340500>;
impl EnblRstToleranceOfPric1334pric13340500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1334pric13340500 {
        match self.bits {
            false => EnblRstToleranceOfPric1334pric13340500::ResetBySrst,
            true => EnblRstToleranceOfPric1334pric13340500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1334pric13340500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1334pric13340500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1334PRIC13340500` writer - Enable Reset Tolerance of PRIC1334PRIC1_334\\[05:00\\]"]
pub type EnblRstToleranceOfPric1334pric13340500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1334pric13340500>;
impl<'a, REG> EnblRstToleranceOfPric1334pric13340500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1334pric13340500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1334pric13340500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1334PRIC13340600` reader - Enable Write Protection of PRIC1334PRIC1_334\\[06:00\\]"]
pub type EnblWrProtOfPric1334pric13340600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1334PRIC13340600` writer - Enable Write Protection of PRIC1334PRIC1_334\\[06:00\\]"]
pub type EnblWrProtOfPric1334pric13340600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfSGPIOMaster1Privilege` reader - Enable Read Group #0 of SGPIO Master 1 Privilege"]
pub type EnblReadGroup0ofSgpiomaster1privilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfSGPIOMaster1Privilege` writer - Enable Read Group #0 of SGPIO Master 1 Privilege"]
pub type EnblReadGroup0ofSgpiomaster1privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfSGPIOMaster1Privilege` reader - Enable Read Group #1 of SGPIO Master 1 Privilege"]
pub type EnblReadGroup1ofSgpiomaster1privilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfSGPIOMaster1Privilege` writer - Enable Read Group #1 of SGPIO Master 1 Privilege"]
pub type EnblReadGroup1ofSgpiomaster1privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfSGPIOMaster1Privilege` reader - Enable Read Group #2 of SGPIO Master 1 Privilege"]
pub type EnblReadGroup2ofSgpiomaster1privilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfSGPIOMaster1Privilege` writer - Enable Read Group #2 of SGPIO Master 1 Privilege"]
pub type EnblReadGroup2ofSgpiomaster1privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfSGPIOMaster1Privilege` reader - Enable Read Group #3 of SGPIO Master 1 Privilege"]
pub type EnblReadGroup3ofSgpiomaster1privilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfSGPIOMaster1Privilege` writer - Enable Read Group #3 of SGPIO Master 1 Privilege"]
pub type EnblReadGroup3ofSgpiomaster1privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfSGPIOMaster1Privilege` reader - Enable Read Group #4 of SGPIO Master 1 Privilege"]
pub type EnblReadGroup4ofSgpiomaster1privilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfSGPIOMaster1Privilege` writer - Enable Read Group #4 of SGPIO Master 1 Privilege"]
pub type EnblReadGroup4ofSgpiomaster1privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfSGPIOMaster1Privilege` reader - Enable Read Group #5 of SGPIO Master 1 Privilege"]
pub type EnblReadGroup5ofSgpiomaster1privilegeR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfSGPIOMaster1Privilege` writer - Enable Read Group #5 of SGPIO Master 1 Privilege"]
pub type EnblReadGroup5ofSgpiomaster1privilegeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1334PRIC1_334\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1334pric13341308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1334pric13341308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1334pric13341308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1334PRIC13341308` reader - Enable Reset Tolerance of PRIC1334PRIC1_334\\[13:08\\]"]
pub type EnblRstToleranceOfPric1334pric13341308R =
    crate::BitReader<EnblRstToleranceOfPric1334pric13341308>;
impl EnblRstToleranceOfPric1334pric13341308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1334pric13341308 {
        match self.bits {
            false => EnblRstToleranceOfPric1334pric13341308::ResetBySrst,
            true => EnblRstToleranceOfPric1334pric13341308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1334pric13341308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1334pric13341308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1334PRIC13341308` writer - Enable Reset Tolerance of PRIC1334PRIC1_334\\[13:08\\]"]
pub type EnblRstToleranceOfPric1334pric13341308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1334pric13341308>;
impl<'a, REG> EnblRstToleranceOfPric1334pric13341308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1334pric13341308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1334pric13341308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1334PRIC13341408` reader - Enable Write Protection of PRIC1334PRIC1_334\\[14:08\\]"]
pub type EnblWrProtOfPric1334pric13341408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1334PRIC13341408` writer - Enable Write Protection of PRIC1334PRIC1_334\\[14:08\\]"]
pub type EnblWrProtOfPric1334pric13341408W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `EnblReadGroup0OfI2CGlobal` reader - Enable Read Group #0 of I2C Global"]
pub type EnblReadGroup0ofI2cglobalR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfI2CGlobal` writer - Enable Read Group #0 of I2C Global"]
pub type EnblReadGroup0ofI2cglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfI2CGlobal` reader - Enable Read Group #1 of I2C Global"]
pub type EnblReadGroup1ofI2cglobalR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfI2CGlobal` writer - Enable Read Group #1 of I2C Global"]
pub type EnblReadGroup1ofI2cglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfI2CGlobal` reader - Enable Read Group #2 of I2C Global"]
pub type EnblReadGroup2ofI2cglobalR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfI2CGlobal` writer - Enable Read Group #2 of I2C Global"]
pub type EnblReadGroup2ofI2cglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfI2CGlobal` reader - Enable Read Group #3 of I2C Global"]
pub type EnblReadGroup3ofI2cglobalR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfI2CGlobal` writer - Enable Read Group #3 of I2C Global"]
pub type EnblReadGroup3ofI2cglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfI2CGlobal` reader - Enable Read Group #4 of I2C Global"]
pub type EnblReadGroup4ofI2cglobalR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfI2CGlobal` writer - Enable Read Group #4 of I2C Global"]
pub type EnblReadGroup4ofI2cglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfI2CGlobal` reader - Enable Read Group #5 of I2C Global"]
pub type EnblReadGroup5ofI2cglobalR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfI2CGlobal` writer - Enable Read Group #5 of I2C Global"]
pub type EnblReadGroup5ofI2cglobalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1334PRIC1_334\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1334pric13342924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1334pric13342924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1334pric13342924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1334PRIC13342924` reader - Enable Reset Tolerance of PRIC1334PRIC1_334\\[29:24\\]"]
pub type EnblRstToleranceOfPric1334pric13342924R =
    crate::BitReader<EnblRstToleranceOfPric1334pric13342924>;
impl EnblRstToleranceOfPric1334pric13342924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1334pric13342924 {
        match self.bits {
            false => EnblRstToleranceOfPric1334pric13342924::ResetBySrst,
            true => EnblRstToleranceOfPric1334pric13342924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1334pric13342924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1334pric13342924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1334PRIC13342924` writer - Enable Reset Tolerance of PRIC1334PRIC1_334\\[29:24\\]"]
pub type EnblRstToleranceOfPric1334pric13342924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1334pric13342924>;
impl<'a, REG> EnblRstToleranceOfPric1334pric13342924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1334pric13342924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1334pric13342924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1334PRIC13343024` reader - Enable Write Protection of PRIC1334PRIC1_334\\[30:24\\]"]
pub type EnblWrProtOfPric1334pric13343024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1334PRIC13343024` writer - Enable Write Protection of PRIC1334PRIC1_334\\[30:24\\]"]
pub type EnblWrProtOfPric1334pric13343024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_read_group0of_sgpiomaster1(&self) -> EnblReadGroup0ofSgpiomaster1R {
        EnblReadGroup0ofSgpiomaster1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_read_group1of_sgpiomaster1(&self) -> EnblReadGroup1ofSgpiomaster1R {
        EnblReadGroup1ofSgpiomaster1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_read_group2of_sgpiomaster1(&self) -> EnblReadGroup2ofSgpiomaster1R {
        EnblReadGroup2ofSgpiomaster1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_read_group3of_sgpiomaster1(&self) -> EnblReadGroup3ofSgpiomaster1R {
        EnblReadGroup3ofSgpiomaster1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_read_group4of_sgpiomaster1(&self) -> EnblReadGroup4ofSgpiomaster1R {
        EnblReadGroup4ofSgpiomaster1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_read_group5of_sgpiomaster1(&self) -> EnblReadGroup5ofSgpiomaster1R {
        EnblReadGroup5ofSgpiomaster1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1334PRIC1_334\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1334pric13340500(
        &self,
    ) -> EnblRstToleranceOfPric1334pric13340500R {
        EnblRstToleranceOfPric1334pric13340500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1334PRIC1_334\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1334pric13340600(&self) -> EnblWrProtOfPric1334pric13340600R {
        EnblWrProtOfPric1334pric13340600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group0of_sgpiomaster1privilege(
        &self,
    ) -> EnblReadGroup0ofSgpiomaster1privilegeR {
        EnblReadGroup0ofSgpiomaster1privilegeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group1of_sgpiomaster1privilege(
        &self,
    ) -> EnblReadGroup1ofSgpiomaster1privilegeR {
        EnblReadGroup1ofSgpiomaster1privilegeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group2of_sgpiomaster1privilege(
        &self,
    ) -> EnblReadGroup2ofSgpiomaster1privilegeR {
        EnblReadGroup2ofSgpiomaster1privilegeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group3of_sgpiomaster1privilege(
        &self,
    ) -> EnblReadGroup3ofSgpiomaster1privilegeR {
        EnblReadGroup3ofSgpiomaster1privilegeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group4of_sgpiomaster1privilege(
        &self,
    ) -> EnblReadGroup4ofSgpiomaster1privilegeR {
        EnblReadGroup4ofSgpiomaster1privilegeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group5of_sgpiomaster1privilege(
        &self,
    ) -> EnblReadGroup5ofSgpiomaster1privilegeR {
        EnblReadGroup5ofSgpiomaster1privilegeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1334PRIC1_334\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1334pric13341308(
        &self,
    ) -> EnblRstToleranceOfPric1334pric13341308R {
        EnblRstToleranceOfPric1334pric13341308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1334PRIC1_334\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1334pric13341408(&self) -> EnblWrProtOfPric1334pric13341408R {
        EnblWrProtOfPric1334pric13341408R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 24 - Enable Read Group #0 of I2C Global"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2cglobal(&self) -> EnblReadGroup0ofI2cglobalR {
        EnblReadGroup0ofI2cglobalR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of I2C Global"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2cglobal(&self) -> EnblReadGroup1ofI2cglobalR {
        EnblReadGroup1ofI2cglobalR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of I2C Global"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2cglobal(&self) -> EnblReadGroup2ofI2cglobalR {
        EnblReadGroup2ofI2cglobalR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of I2C Global"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2cglobal(&self) -> EnblReadGroup3ofI2cglobalR {
        EnblReadGroup3ofI2cglobalR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of I2C Global"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2cglobal(&self) -> EnblReadGroup4ofI2cglobalR {
        EnblReadGroup4ofI2cglobalR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of I2C Global"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2cglobal(&self) -> EnblReadGroup5ofI2cglobalR {
        EnblReadGroup5ofI2cglobalR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1334PRIC1_334\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1334pric13342924(
        &self,
    ) -> EnblRstToleranceOfPric1334pric13342924R {
        EnblRstToleranceOfPric1334pric13342924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1334PRIC1_334\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1334pric13343024(&self) -> EnblWrProtOfPric1334pric13343024R {
        EnblWrProtOfPric1334pric13343024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_read_group0of_sgpiomaster1(
        &mut self,
    ) -> EnblReadGroup0ofSgpiomaster1W<PricIo334Spec> {
        EnblReadGroup0ofSgpiomaster1W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_read_group1of_sgpiomaster1(
        &mut self,
    ) -> EnblReadGroup1ofSgpiomaster1W<PricIo334Spec> {
        EnblReadGroup1ofSgpiomaster1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_read_group2of_sgpiomaster1(
        &mut self,
    ) -> EnblReadGroup2ofSgpiomaster1W<PricIo334Spec> {
        EnblReadGroup2ofSgpiomaster1W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_read_group3of_sgpiomaster1(
        &mut self,
    ) -> EnblReadGroup3ofSgpiomaster1W<PricIo334Spec> {
        EnblReadGroup3ofSgpiomaster1W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_read_group4of_sgpiomaster1(
        &mut self,
    ) -> EnblReadGroup4ofSgpiomaster1W<PricIo334Spec> {
        EnblReadGroup4ofSgpiomaster1W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of SGPIO Master 1"]
    #[inline(always)]
    pub fn enbl_read_group5of_sgpiomaster1(
        &mut self,
    ) -> EnblReadGroup5ofSgpiomaster1W<PricIo334Spec> {
        EnblReadGroup5ofSgpiomaster1W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1334PRIC1_334\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1334pric13340500(
        &mut self,
    ) -> EnblRstToleranceOfPric1334pric13340500W<PricIo334Spec> {
        EnblRstToleranceOfPric1334pric13340500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1334PRIC1_334\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1334pric13340600(
        &mut self,
    ) -> EnblWrProtOfPric1334pric13340600W<PricIo334Spec> {
        EnblWrProtOfPric1334pric13340600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group0of_sgpiomaster1privilege(
        &mut self,
    ) -> EnblReadGroup0ofSgpiomaster1privilegeW<PricIo334Spec> {
        EnblReadGroup0ofSgpiomaster1privilegeW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group1of_sgpiomaster1privilege(
        &mut self,
    ) -> EnblReadGroup1ofSgpiomaster1privilegeW<PricIo334Spec> {
        EnblReadGroup1ofSgpiomaster1privilegeW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group2of_sgpiomaster1privilege(
        &mut self,
    ) -> EnblReadGroup2ofSgpiomaster1privilegeW<PricIo334Spec> {
        EnblReadGroup2ofSgpiomaster1privilegeW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group3of_sgpiomaster1privilege(
        &mut self,
    ) -> EnblReadGroup3ofSgpiomaster1privilegeW<PricIo334Spec> {
        EnblReadGroup3ofSgpiomaster1privilegeW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group4of_sgpiomaster1privilege(
        &mut self,
    ) -> EnblReadGroup4ofSgpiomaster1privilegeW<PricIo334Spec> {
        EnblReadGroup4ofSgpiomaster1privilegeW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of SGPIO Master 1 Privilege"]
    #[inline(always)]
    pub fn enbl_read_group5of_sgpiomaster1privilege(
        &mut self,
    ) -> EnblReadGroup5ofSgpiomaster1privilegeW<PricIo334Spec> {
        EnblReadGroup5ofSgpiomaster1privilegeW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1334PRIC1_334\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1334pric13341308(
        &mut self,
    ) -> EnblRstToleranceOfPric1334pric13341308W<PricIo334Spec> {
        EnblRstToleranceOfPric1334pric13341308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1334PRIC1_334\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1334pric13341408(
        &mut self,
    ) -> EnblWrProtOfPric1334pric13341408W<PricIo334Spec> {
        EnblWrProtOfPric1334pric13341408W::new(self, 15)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo334Spec> {
        Reserved7W::new(self, 16)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo334Spec> {
        Reserved6W::new(self, 17)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo334Spec> {
        Reserved5W::new(self, 18)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo334Spec> {
        Reserved4W::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo334Spec> {
        Reserved3W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo334Spec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo334Spec> {
        Reserved1W::new(self, 22)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of I2C Global"]
    #[inline(always)]
    pub fn enbl_read_group0of_i2cglobal(&mut self) -> EnblReadGroup0ofI2cglobalW<PricIo334Spec> {
        EnblReadGroup0ofI2cglobalW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of I2C Global"]
    #[inline(always)]
    pub fn enbl_read_group1of_i2cglobal(&mut self) -> EnblReadGroup1ofI2cglobalW<PricIo334Spec> {
        EnblReadGroup1ofI2cglobalW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of I2C Global"]
    #[inline(always)]
    pub fn enbl_read_group2of_i2cglobal(&mut self) -> EnblReadGroup2ofI2cglobalW<PricIo334Spec> {
        EnblReadGroup2ofI2cglobalW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of I2C Global"]
    #[inline(always)]
    pub fn enbl_read_group3of_i2cglobal(&mut self) -> EnblReadGroup3ofI2cglobalW<PricIo334Spec> {
        EnblReadGroup3ofI2cglobalW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of I2C Global"]
    #[inline(always)]
    pub fn enbl_read_group4of_i2cglobal(&mut self) -> EnblReadGroup4ofI2cglobalW<PricIo334Spec> {
        EnblReadGroup4ofI2cglobalW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of I2C Global"]
    #[inline(always)]
    pub fn enbl_read_group5of_i2cglobal(&mut self) -> EnblReadGroup5ofI2cglobalW<PricIo334Spec> {
        EnblReadGroup5ofI2cglobalW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1334PRIC1_334\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1334pric13342924(
        &mut self,
    ) -> EnblRstToleranceOfPric1334pric13342924W<PricIo334Spec> {
        EnblRstToleranceOfPric1334pric13342924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1334PRIC1_334\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1334pric13343024(
        &mut self,
    ) -> EnblWrProtOfPric1334pric13343024W<PricIo334Spec> {
        EnblWrProtOfPric1334pric13343024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io334::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io334::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo334Spec;
impl crate::RegisterSpec for PricIo334Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io334::R`](R) reader structure"]
impl crate::Readable for PricIo334Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io334::W`](W) writer structure"]
impl crate::Writable for PricIo334Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO334 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo334Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
