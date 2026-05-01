#[doc = "Register `PRIC_IO374` reader"]
pub type R = crate::R<PricIo374Spec>;
#[doc = "Register `PRIC_IO374` writer"]
pub type W = crate::W<PricIo374Spec>;
#[doc = "Field `EnblReadGroup0OfUART7` reader - Enable Read Group #0 of UART7"]
pub type EnblReadGroup0ofUart7R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUART7` writer - Enable Read Group #0 of UART7"]
pub type EnblReadGroup0ofUart7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUART7` reader - Enable Read Group #1 of UART7"]
pub type EnblReadGroup1ofUart7R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUART7` writer - Enable Read Group #1 of UART7"]
pub type EnblReadGroup1ofUart7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUART7` reader - Enable Read Group #2 of UART7"]
pub type EnblReadGroup2ofUart7R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUART7` writer - Enable Read Group #2 of UART7"]
pub type EnblReadGroup2ofUart7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUART7` reader - Enable Read Group #3 of UART7"]
pub type EnblReadGroup3ofUart7R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUART7` writer - Enable Read Group #3 of UART7"]
pub type EnblReadGroup3ofUart7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUART7` reader - Enable Read Group #4 of UART7"]
pub type EnblReadGroup4ofUart7R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUART7` writer - Enable Read Group #4 of UART7"]
pub type EnblReadGroup4ofUart7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUART7` reader - Enable Read Group #5 of UART7"]
pub type EnblReadGroup5ofUart7R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUART7` writer - Enable Read Group #5 of UART7"]
pub type EnblReadGroup5ofUart7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1374PRIC1_374\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1374pric13740500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1374pric13740500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1374pric13740500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1374PRIC13740500` reader - Enable Reset Tolerance of PRIC1374PRIC1_374\\[05:00\\]"]
pub type EnblRstToleranceOfPric1374pric13740500R =
    crate::BitReader<EnblRstToleranceOfPric1374pric13740500>;
impl EnblRstToleranceOfPric1374pric13740500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1374pric13740500 {
        match self.bits {
            false => EnblRstToleranceOfPric1374pric13740500::ResetBySrst,
            true => EnblRstToleranceOfPric1374pric13740500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1374pric13740500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1374pric13740500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1374PRIC13740500` writer - Enable Reset Tolerance of PRIC1374PRIC1_374\\[05:00\\]"]
pub type EnblRstToleranceOfPric1374pric13740500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1374pric13740500>;
impl<'a, REG> EnblRstToleranceOfPric1374pric13740500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1374pric13740500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1374pric13740500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1374PRIC13740600` reader - Enable Write Protection of PRIC1374PRIC1_374\\[06:00\\]"]
pub type EnblWrProtOfPric1374pric13740600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1374PRIC13740600` writer - Enable Write Protection of PRIC1374PRIC1_374\\[06:00\\]"]
pub type EnblWrProtOfPric1374pric13740600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfUART8` reader - Enable Read Group #0 of UART8"]
pub type EnblReadGroup0ofUart8R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUART8` writer - Enable Read Group #0 of UART8"]
pub type EnblReadGroup0ofUart8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUART8` reader - Enable Read Group #1 of UART8"]
pub type EnblReadGroup1ofUart8R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUART8` writer - Enable Read Group #1 of UART8"]
pub type EnblReadGroup1ofUart8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUART8` reader - Enable Read Group #2 of UART8"]
pub type EnblReadGroup2ofUart8R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUART8` writer - Enable Read Group #2 of UART8"]
pub type EnblReadGroup2ofUart8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUART8` reader - Enable Read Group #3 of UART8"]
pub type EnblReadGroup3ofUart8R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUART8` writer - Enable Read Group #3 of UART8"]
pub type EnblReadGroup3ofUart8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUART8` reader - Enable Read Group #4 of UART8"]
pub type EnblReadGroup4ofUart8R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUART8` writer - Enable Read Group #4 of UART8"]
pub type EnblReadGroup4ofUart8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUART8` reader - Enable Read Group #5 of UART8"]
pub type EnblReadGroup5ofUart8R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUART8` writer - Enable Read Group #5 of UART8"]
pub type EnblReadGroup5ofUart8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1374PRIC1_374\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1374pric13741308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1374pric13741308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1374pric13741308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1374PRIC13741308` reader - Enable Reset Tolerance of PRIC1374PRIC1_374\\[13:08\\]"]
pub type EnblRstToleranceOfPric1374pric13741308R =
    crate::BitReader<EnblRstToleranceOfPric1374pric13741308>;
impl EnblRstToleranceOfPric1374pric13741308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1374pric13741308 {
        match self.bits {
            false => EnblRstToleranceOfPric1374pric13741308::ResetBySrst,
            true => EnblRstToleranceOfPric1374pric13741308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1374pric13741308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1374pric13741308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1374PRIC13741308` writer - Enable Reset Tolerance of PRIC1374PRIC1_374\\[13:08\\]"]
pub type EnblRstToleranceOfPric1374pric13741308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1374pric13741308>;
impl<'a, REG> EnblRstToleranceOfPric1374pric13741308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1374pric13741308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1374pric13741308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1374PRIC13741408` reader - Enable Write Protection of PRIC1374PRIC1_374\\[14:08\\]"]
pub type EnblWrProtOfPric1374pric13741408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1374PRIC13741408` writer - Enable Write Protection of PRIC1374PRIC1_374\\[14:08\\]"]
pub type EnblWrProtOfPric1374pric13741408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfUART9` reader - Enable Read Group #0 of UART9"]
pub type EnblReadGroup0ofUart9R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUART9` writer - Enable Read Group #0 of UART9"]
pub type EnblReadGroup0ofUart9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUART9` reader - Enable Read Group #1 of UART9"]
pub type EnblReadGroup1ofUart9R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUART9` writer - Enable Read Group #1 of UART9"]
pub type EnblReadGroup1ofUart9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUART9` reader - Enable Read Group #2 of UART9"]
pub type EnblReadGroup2ofUart9R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUART9` writer - Enable Read Group #2 of UART9"]
pub type EnblReadGroup2ofUart9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUART9` reader - Enable Read Group #3 of UART9"]
pub type EnblReadGroup3ofUart9R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUART9` writer - Enable Read Group #3 of UART9"]
pub type EnblReadGroup3ofUart9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUART9` reader - Enable Read Group #4 of UART9"]
pub type EnblReadGroup4ofUart9R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUART9` writer - Enable Read Group #4 of UART9"]
pub type EnblReadGroup4ofUart9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUART9` reader - Enable Read Group #5 of UART9"]
pub type EnblReadGroup5ofUart9R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUART9` writer - Enable Read Group #5 of UART9"]
pub type EnblReadGroup5ofUart9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1374PRIC1_374\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1374pric13742116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1374pric13742116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1374pric13742116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1374PRIC13742116` reader - Enable Reset Tolerance of PRIC1374PRIC1_374\\[21:16\\]"]
pub type EnblRstToleranceOfPric1374pric13742116R =
    crate::BitReader<EnblRstToleranceOfPric1374pric13742116>;
impl EnblRstToleranceOfPric1374pric13742116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1374pric13742116 {
        match self.bits {
            false => EnblRstToleranceOfPric1374pric13742116::ResetBySrst,
            true => EnblRstToleranceOfPric1374pric13742116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1374pric13742116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1374pric13742116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1374PRIC13742116` writer - Enable Reset Tolerance of PRIC1374PRIC1_374\\[21:16\\]"]
pub type EnblRstToleranceOfPric1374pric13742116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1374pric13742116>;
impl<'a, REG> EnblRstToleranceOfPric1374pric13742116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1374pric13742116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1374pric13742116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1374PRIC13742216` reader - Enable Write Protection of PRIC1374PRIC1_374\\[22:16\\]"]
pub type EnblWrProtOfPric1374pric13742216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1374PRIC13742216` writer - Enable Write Protection of PRIC1374PRIC1_374\\[22:16\\]"]
pub type EnblWrProtOfPric1374pric13742216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfUART10` reader - Enable Read Group #0 of UART10"]
pub type EnblReadGroup0ofUart10R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUART10` writer - Enable Read Group #0 of UART10"]
pub type EnblReadGroup0ofUart10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUART10` reader - Enable Read Group #1 of UART10"]
pub type EnblReadGroup1ofUart10R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUART10` writer - Enable Read Group #1 of UART10"]
pub type EnblReadGroup1ofUart10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUART10` reader - Enable Read Group #2 of UART10"]
pub type EnblReadGroup2ofUart10R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUART10` writer - Enable Read Group #2 of UART10"]
pub type EnblReadGroup2ofUart10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUART10` reader - Enable Read Group #3 of UART10"]
pub type EnblReadGroup3ofUart10R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUART10` writer - Enable Read Group #3 of UART10"]
pub type EnblReadGroup3ofUart10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUART10` reader - Enable Read Group #4 of UART10"]
pub type EnblReadGroup4ofUart10R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUART10` writer - Enable Read Group #4 of UART10"]
pub type EnblReadGroup4ofUart10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUART10` reader - Enable Read Group #5 of UART10"]
pub type EnblReadGroup5ofUart10R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUART10` writer - Enable Read Group #5 of UART10"]
pub type EnblReadGroup5ofUart10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1374PRIC1_374\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1374pric13742924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1374pric13742924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1374pric13742924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1374PRIC13742924` reader - Enable Reset Tolerance of PRIC1374PRIC1_374\\[29:24\\]"]
pub type EnblRstToleranceOfPric1374pric13742924R =
    crate::BitReader<EnblRstToleranceOfPric1374pric13742924>;
impl EnblRstToleranceOfPric1374pric13742924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1374pric13742924 {
        match self.bits {
            false => EnblRstToleranceOfPric1374pric13742924::ResetBySrst,
            true => EnblRstToleranceOfPric1374pric13742924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1374pric13742924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1374pric13742924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1374PRIC13742924` writer - Enable Reset Tolerance of PRIC1374PRIC1_374\\[29:24\\]"]
pub type EnblRstToleranceOfPric1374pric13742924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1374pric13742924>;
impl<'a, REG> EnblRstToleranceOfPric1374pric13742924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1374pric13742924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1374pric13742924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1374PRIC13743024` reader - Enable Write Protection of PRIC1374PRIC1_374\\[30:24\\]"]
pub type EnblWrProtOfPric1374pric13743024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1374PRIC13743024` writer - Enable Write Protection of PRIC1374PRIC1_374\\[30:24\\]"]
pub type EnblWrProtOfPric1374pric13743024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of UART7"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart7(&self) -> EnblReadGroup0ofUart7R {
        EnblReadGroup0ofUart7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of UART7"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart7(&self) -> EnblReadGroup1ofUart7R {
        EnblReadGroup1ofUart7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of UART7"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart7(&self) -> EnblReadGroup2ofUart7R {
        EnblReadGroup2ofUart7R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of UART7"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart7(&self) -> EnblReadGroup3ofUart7R {
        EnblReadGroup3ofUart7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of UART7"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart7(&self) -> EnblReadGroup4ofUart7R {
        EnblReadGroup4ofUart7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of UART7"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart7(&self) -> EnblReadGroup5ofUart7R {
        EnblReadGroup5ofUart7R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1374PRIC1_374\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1374pric13740500(
        &self,
    ) -> EnblRstToleranceOfPric1374pric13740500R {
        EnblRstToleranceOfPric1374pric13740500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1374PRIC1_374\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1374pric13740600(&self) -> EnblWrProtOfPric1374pric13740600R {
        EnblWrProtOfPric1374pric13740600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of UART8"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart8(&self) -> EnblReadGroup0ofUart8R {
        EnblReadGroup0ofUart8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of UART8"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart8(&self) -> EnblReadGroup1ofUart8R {
        EnblReadGroup1ofUart8R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of UART8"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart8(&self) -> EnblReadGroup2ofUart8R {
        EnblReadGroup2ofUart8R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of UART8"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart8(&self) -> EnblReadGroup3ofUart8R {
        EnblReadGroup3ofUart8R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of UART8"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart8(&self) -> EnblReadGroup4ofUart8R {
        EnblReadGroup4ofUart8R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of UART8"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart8(&self) -> EnblReadGroup5ofUart8R {
        EnblReadGroup5ofUart8R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1374PRIC1_374\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1374pric13741308(
        &self,
    ) -> EnblRstToleranceOfPric1374pric13741308R {
        EnblRstToleranceOfPric1374pric13741308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1374PRIC1_374\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1374pric13741408(&self) -> EnblWrProtOfPric1374pric13741408R {
        EnblWrProtOfPric1374pric13741408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of UART9"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart9(&self) -> EnblReadGroup0ofUart9R {
        EnblReadGroup0ofUart9R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of UART9"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart9(&self) -> EnblReadGroup1ofUart9R {
        EnblReadGroup1ofUart9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of UART9"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart9(&self) -> EnblReadGroup2ofUart9R {
        EnblReadGroup2ofUart9R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of UART9"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart9(&self) -> EnblReadGroup3ofUart9R {
        EnblReadGroup3ofUart9R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of UART9"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart9(&self) -> EnblReadGroup4ofUart9R {
        EnblReadGroup4ofUart9R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of UART9"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart9(&self) -> EnblReadGroup5ofUart9R {
        EnblReadGroup5ofUart9R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1374PRIC1_374\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1374pric13742116(
        &self,
    ) -> EnblRstToleranceOfPric1374pric13742116R {
        EnblRstToleranceOfPric1374pric13742116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1374PRIC1_374\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1374pric13742216(&self) -> EnblWrProtOfPric1374pric13742216R {
        EnblWrProtOfPric1374pric13742216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of UART10"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart10(&self) -> EnblReadGroup0ofUart10R {
        EnblReadGroup0ofUart10R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of UART10"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart10(&self) -> EnblReadGroup1ofUart10R {
        EnblReadGroup1ofUart10R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of UART10"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart10(&self) -> EnblReadGroup2ofUart10R {
        EnblReadGroup2ofUart10R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of UART10"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart10(&self) -> EnblReadGroup3ofUart10R {
        EnblReadGroup3ofUart10R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of UART10"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart10(&self) -> EnblReadGroup4ofUart10R {
        EnblReadGroup4ofUart10R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of UART10"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart10(&self) -> EnblReadGroup5ofUart10R {
        EnblReadGroup5ofUart10R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1374PRIC1_374\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1374pric13742924(
        &self,
    ) -> EnblRstToleranceOfPric1374pric13742924R {
        EnblRstToleranceOfPric1374pric13742924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1374PRIC1_374\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1374pric13743024(&self) -> EnblWrProtOfPric1374pric13743024R {
        EnblWrProtOfPric1374pric13743024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of UART7"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart7(&mut self) -> EnblReadGroup0ofUart7W<PricIo374Spec> {
        EnblReadGroup0ofUart7W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of UART7"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart7(&mut self) -> EnblReadGroup1ofUart7W<PricIo374Spec> {
        EnblReadGroup1ofUart7W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of UART7"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart7(&mut self) -> EnblReadGroup2ofUart7W<PricIo374Spec> {
        EnblReadGroup2ofUart7W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of UART7"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart7(&mut self) -> EnblReadGroup3ofUart7W<PricIo374Spec> {
        EnblReadGroup3ofUart7W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of UART7"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart7(&mut self) -> EnblReadGroup4ofUart7W<PricIo374Spec> {
        EnblReadGroup4ofUart7W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of UART7"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart7(&mut self) -> EnblReadGroup5ofUart7W<PricIo374Spec> {
        EnblReadGroup5ofUart7W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1374PRIC1_374\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1374pric13740500(
        &mut self,
    ) -> EnblRstToleranceOfPric1374pric13740500W<PricIo374Spec> {
        EnblRstToleranceOfPric1374pric13740500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1374PRIC1_374\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1374pric13740600(
        &mut self,
    ) -> EnblWrProtOfPric1374pric13740600W<PricIo374Spec> {
        EnblWrProtOfPric1374pric13740600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of UART8"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart8(&mut self) -> EnblReadGroup0ofUart8W<PricIo374Spec> {
        EnblReadGroup0ofUart8W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of UART8"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart8(&mut self) -> EnblReadGroup1ofUart8W<PricIo374Spec> {
        EnblReadGroup1ofUart8W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of UART8"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart8(&mut self) -> EnblReadGroup2ofUart8W<PricIo374Spec> {
        EnblReadGroup2ofUart8W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of UART8"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart8(&mut self) -> EnblReadGroup3ofUart8W<PricIo374Spec> {
        EnblReadGroup3ofUart8W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of UART8"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart8(&mut self) -> EnblReadGroup4ofUart8W<PricIo374Spec> {
        EnblReadGroup4ofUart8W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of UART8"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart8(&mut self) -> EnblReadGroup5ofUart8W<PricIo374Spec> {
        EnblReadGroup5ofUart8W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1374PRIC1_374\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1374pric13741308(
        &mut self,
    ) -> EnblRstToleranceOfPric1374pric13741308W<PricIo374Spec> {
        EnblRstToleranceOfPric1374pric13741308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1374PRIC1_374\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1374pric13741408(
        &mut self,
    ) -> EnblWrProtOfPric1374pric13741408W<PricIo374Spec> {
        EnblWrProtOfPric1374pric13741408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of UART9"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart9(&mut self) -> EnblReadGroup0ofUart9W<PricIo374Spec> {
        EnblReadGroup0ofUart9W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of UART9"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart9(&mut self) -> EnblReadGroup1ofUart9W<PricIo374Spec> {
        EnblReadGroup1ofUart9W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of UART9"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart9(&mut self) -> EnblReadGroup2ofUart9W<PricIo374Spec> {
        EnblReadGroup2ofUart9W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of UART9"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart9(&mut self) -> EnblReadGroup3ofUart9W<PricIo374Spec> {
        EnblReadGroup3ofUart9W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of UART9"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart9(&mut self) -> EnblReadGroup4ofUart9W<PricIo374Spec> {
        EnblReadGroup4ofUart9W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of UART9"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart9(&mut self) -> EnblReadGroup5ofUart9W<PricIo374Spec> {
        EnblReadGroup5ofUart9W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1374PRIC1_374\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1374pric13742116(
        &mut self,
    ) -> EnblRstToleranceOfPric1374pric13742116W<PricIo374Spec> {
        EnblRstToleranceOfPric1374pric13742116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1374PRIC1_374\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1374pric13742216(
        &mut self,
    ) -> EnblWrProtOfPric1374pric13742216W<PricIo374Spec> {
        EnblWrProtOfPric1374pric13742216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of UART10"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart10(&mut self) -> EnblReadGroup0ofUart10W<PricIo374Spec> {
        EnblReadGroup0ofUart10W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of UART10"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart10(&mut self) -> EnblReadGroup1ofUart10W<PricIo374Spec> {
        EnblReadGroup1ofUart10W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of UART10"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart10(&mut self) -> EnblReadGroup2ofUart10W<PricIo374Spec> {
        EnblReadGroup2ofUart10W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of UART10"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart10(&mut self) -> EnblReadGroup3ofUart10W<PricIo374Spec> {
        EnblReadGroup3ofUart10W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of UART10"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart10(&mut self) -> EnblReadGroup4ofUart10W<PricIo374Spec> {
        EnblReadGroup4ofUart10W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of UART10"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart10(&mut self) -> EnblReadGroup5ofUart10W<PricIo374Spec> {
        EnblReadGroup5ofUart10W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1374PRIC1_374\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1374pric13742924(
        &mut self,
    ) -> EnblRstToleranceOfPric1374pric13742924W<PricIo374Spec> {
        EnblRstToleranceOfPric1374pric13742924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1374PRIC1_374\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1374pric13743024(
        &mut self,
    ) -> EnblWrProtOfPric1374pric13743024W<PricIo374Spec> {
        EnblWrProtOfPric1374pric13743024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#29\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io374::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io374::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo374Spec;
impl crate::RegisterSpec for PricIo374Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io374::R`](R) reader structure"]
impl crate::Readable for PricIo374Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io374::W`](W) writer structure"]
impl crate::Writable for PricIo374Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO374 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo374Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
