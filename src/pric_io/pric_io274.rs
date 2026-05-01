#[doc = "Register `PRIC_IO274` reader"]
pub type R = crate::R<PricIo274Spec>;
#[doc = "Register `PRIC_IO274` writer"]
pub type W = crate::W<PricIo274Spec>;
#[doc = "Field `EnblWrGroup0OfUART7` reader - Enable Write Group #0 of UART7"]
pub type EnblWrGroup0ofUart7R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUART7` writer - Enable Write Group #0 of UART7"]
pub type EnblWrGroup0ofUart7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUART7` reader - Enable Write Group #1 of UART7"]
pub type EnblWrGroup1ofUart7R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUART7` writer - Enable Write Group #1 of UART7"]
pub type EnblWrGroup1ofUart7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUART7` reader - Enable Write Group #2 of UART7"]
pub type EnblWrGroup2ofUart7R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUART7` writer - Enable Write Group #2 of UART7"]
pub type EnblWrGroup2ofUart7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUART7` reader - Enable Write Group #3 of UART7"]
pub type EnblWrGroup3ofUart7R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUART7` writer - Enable Write Group #3 of UART7"]
pub type EnblWrGroup3ofUart7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUART7` reader - Enable Write Group #4 of UART7"]
pub type EnblWrGroup4ofUart7R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUART7` writer - Enable Write Group #4 of UART7"]
pub type EnblWrGroup4ofUart7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUART7` reader - Enable Write Group #5 of UART7"]
pub type EnblWrGroup5ofUart7R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUART7` writer - Enable Write Group #5 of UART7"]
pub type EnblWrGroup5ofUart7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1274PRIC1_274\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1274pric12740500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1274pric12740500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1274pric12740500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1274PRIC12740500` reader - Enable Reset Tolerance of PRIC1274PRIC1_274\\[05:00\\]"]
pub type EnblRstToleranceOfPric1274pric12740500R =
    crate::BitReader<EnblRstToleranceOfPric1274pric12740500>;
impl EnblRstToleranceOfPric1274pric12740500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1274pric12740500 {
        match self.bits {
            false => EnblRstToleranceOfPric1274pric12740500::ResetBySrst,
            true => EnblRstToleranceOfPric1274pric12740500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1274pric12740500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1274pric12740500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1274PRIC12740500` writer - Enable Reset Tolerance of PRIC1274PRIC1_274\\[05:00\\]"]
pub type EnblRstToleranceOfPric1274pric12740500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1274pric12740500>;
impl<'a, REG> EnblRstToleranceOfPric1274pric12740500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1274pric12740500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1274pric12740500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1274PRIC12740600` reader - Enable Write Protection of PRIC1274PRIC1_274\\[06:00\\]"]
pub type EnblWrProtOfPric1274pric12740600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1274PRIC12740600` writer - Enable Write Protection of PRIC1274PRIC1_274\\[06:00\\]"]
pub type EnblWrProtOfPric1274pric12740600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfUART8` reader - Enable Write Group #0 of UART8"]
pub type EnblWrGroup0ofUart8R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUART8` writer - Enable Write Group #0 of UART8"]
pub type EnblWrGroup0ofUart8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUART8` reader - Enable Write Group #1 of UART8"]
pub type EnblWrGroup1ofUart8R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUART8` writer - Enable Write Group #1 of UART8"]
pub type EnblWrGroup1ofUart8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUART8` reader - Enable Write Group #2 of UART8"]
pub type EnblWrGroup2ofUart8R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUART8` writer - Enable Write Group #2 of UART8"]
pub type EnblWrGroup2ofUart8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUART8` reader - Enable Write Group #3 of UART8"]
pub type EnblWrGroup3ofUart8R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUART8` writer - Enable Write Group #3 of UART8"]
pub type EnblWrGroup3ofUart8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUART8` reader - Enable Write Group #4 of UART8"]
pub type EnblWrGroup4ofUart8R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUART8` writer - Enable Write Group #4 of UART8"]
pub type EnblWrGroup4ofUart8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUART8` reader - Enable Write Group #5 of UART8"]
pub type EnblWrGroup5ofUart8R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUART8` writer - Enable Write Group #5 of UART8"]
pub type EnblWrGroup5ofUart8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1274PRIC1_274\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1274pric12741308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1274pric12741308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1274pric12741308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1274PRIC12741308` reader - Enable Reset Tolerance of PRIC1274PRIC1_274\\[13:08\\]"]
pub type EnblRstToleranceOfPric1274pric12741308R =
    crate::BitReader<EnblRstToleranceOfPric1274pric12741308>;
impl EnblRstToleranceOfPric1274pric12741308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1274pric12741308 {
        match self.bits {
            false => EnblRstToleranceOfPric1274pric12741308::ResetBySrst,
            true => EnblRstToleranceOfPric1274pric12741308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1274pric12741308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1274pric12741308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1274PRIC12741308` writer - Enable Reset Tolerance of PRIC1274PRIC1_274\\[13:08\\]"]
pub type EnblRstToleranceOfPric1274pric12741308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1274pric12741308>;
impl<'a, REG> EnblRstToleranceOfPric1274pric12741308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1274pric12741308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1274pric12741308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1274PRIC12741408` reader - Enable Write Protection of PRIC1274PRIC1_274\\[14:08\\]"]
pub type EnblWrProtOfPric1274pric12741408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1274PRIC12741408` writer - Enable Write Protection of PRIC1274PRIC1_274\\[14:08\\]"]
pub type EnblWrProtOfPric1274pric12741408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfUART9` reader - Enable Write Group #0 of UART9"]
pub type EnblWrGroup0ofUart9R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUART9` writer - Enable Write Group #0 of UART9"]
pub type EnblWrGroup0ofUart9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUART9` reader - Enable Write Group #1 of UART9"]
pub type EnblWrGroup1ofUart9R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUART9` writer - Enable Write Group #1 of UART9"]
pub type EnblWrGroup1ofUart9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUART9` reader - Enable Write Group #2 of UART9"]
pub type EnblWrGroup2ofUart9R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUART9` writer - Enable Write Group #2 of UART9"]
pub type EnblWrGroup2ofUart9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUART9` reader - Enable Write Group #3 of UART9"]
pub type EnblWrGroup3ofUart9R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUART9` writer - Enable Write Group #3 of UART9"]
pub type EnblWrGroup3ofUart9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUART9` reader - Enable Write Group #4 of UART9"]
pub type EnblWrGroup4ofUart9R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUART9` writer - Enable Write Group #4 of UART9"]
pub type EnblWrGroup4ofUart9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUART9` reader - Enable Write Group #5 of UART9"]
pub type EnblWrGroup5ofUart9R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUART9` writer - Enable Write Group #5 of UART9"]
pub type EnblWrGroup5ofUart9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1274PRIC1_274\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1274pric12742116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1274pric12742116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1274pric12742116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1274PRIC12742116` reader - Enable Reset Tolerance of PRIC1274PRIC1_274\\[21:16\\]"]
pub type EnblRstToleranceOfPric1274pric12742116R =
    crate::BitReader<EnblRstToleranceOfPric1274pric12742116>;
impl EnblRstToleranceOfPric1274pric12742116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1274pric12742116 {
        match self.bits {
            false => EnblRstToleranceOfPric1274pric12742116::ResetBySrst,
            true => EnblRstToleranceOfPric1274pric12742116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1274pric12742116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1274pric12742116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1274PRIC12742116` writer - Enable Reset Tolerance of PRIC1274PRIC1_274\\[21:16\\]"]
pub type EnblRstToleranceOfPric1274pric12742116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1274pric12742116>;
impl<'a, REG> EnblRstToleranceOfPric1274pric12742116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1274pric12742116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1274pric12742116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1274PRIC12742216` reader - Enable Write Protection of PRIC1274PRIC1_274\\[22:16\\]"]
pub type EnblWrProtOfPric1274pric12742216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1274PRIC12742216` writer - Enable Write Protection of PRIC1274PRIC1_274\\[22:16\\]"]
pub type EnblWrProtOfPric1274pric12742216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfUART10` reader - Enable Write Group #0 of UART10"]
pub type EnblWrGroup0ofUart10R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUART10` writer - Enable Write Group #0 of UART10"]
pub type EnblWrGroup0ofUart10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUART10` reader - Enable Write Group #1 of UART10"]
pub type EnblWrGroup1ofUart10R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUART10` writer - Enable Write Group #1 of UART10"]
pub type EnblWrGroup1ofUart10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUART10` reader - Enable Write Group #2 of UART10"]
pub type EnblWrGroup2ofUart10R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUART10` writer - Enable Write Group #2 of UART10"]
pub type EnblWrGroup2ofUart10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUART10` reader - Enable Write Group #3 of UART10"]
pub type EnblWrGroup3ofUart10R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUART10` writer - Enable Write Group #3 of UART10"]
pub type EnblWrGroup3ofUart10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUART10` reader - Enable Write Group #4 of UART10"]
pub type EnblWrGroup4ofUart10R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUART10` writer - Enable Write Group #4 of UART10"]
pub type EnblWrGroup4ofUart10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUART10` reader - Enable Write Group #5 of UART10"]
pub type EnblWrGroup5ofUart10R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUART10` writer - Enable Write Group #5 of UART10"]
pub type EnblWrGroup5ofUart10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1274PRIC1_274\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1274pric12742924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1274pric12742924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1274pric12742924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1274PRIC12742924` reader - Enable Reset Tolerance of PRIC1274PRIC1_274\\[29:24\\]"]
pub type EnblRstToleranceOfPric1274pric12742924R =
    crate::BitReader<EnblRstToleranceOfPric1274pric12742924>;
impl EnblRstToleranceOfPric1274pric12742924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1274pric12742924 {
        match self.bits {
            false => EnblRstToleranceOfPric1274pric12742924::ResetBySrst,
            true => EnblRstToleranceOfPric1274pric12742924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1274pric12742924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1274pric12742924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1274PRIC12742924` writer - Enable Reset Tolerance of PRIC1274PRIC1_274\\[29:24\\]"]
pub type EnblRstToleranceOfPric1274pric12742924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1274pric12742924>;
impl<'a, REG> EnblRstToleranceOfPric1274pric12742924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1274pric12742924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1274pric12742924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1274PRIC12743024` reader - Enable Write Protection of PRIC1274PRIC1_274\\[30:24\\]"]
pub type EnblWrProtOfPric1274pric12743024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1274PRIC12743024` writer - Enable Write Protection of PRIC1274PRIC1_274\\[30:24\\]"]
pub type EnblWrProtOfPric1274pric12743024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of UART7"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart7(&self) -> EnblWrGroup0ofUart7R {
        EnblWrGroup0ofUart7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of UART7"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart7(&self) -> EnblWrGroup1ofUart7R {
        EnblWrGroup1ofUart7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of UART7"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart7(&self) -> EnblWrGroup2ofUart7R {
        EnblWrGroup2ofUart7R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of UART7"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart7(&self) -> EnblWrGroup3ofUart7R {
        EnblWrGroup3ofUart7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of UART7"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart7(&self) -> EnblWrGroup4ofUart7R {
        EnblWrGroup4ofUart7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of UART7"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart7(&self) -> EnblWrGroup5ofUart7R {
        EnblWrGroup5ofUart7R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1274PRIC1_274\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1274pric12740500(
        &self,
    ) -> EnblRstToleranceOfPric1274pric12740500R {
        EnblRstToleranceOfPric1274pric12740500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1274PRIC1_274\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1274pric12740600(&self) -> EnblWrProtOfPric1274pric12740600R {
        EnblWrProtOfPric1274pric12740600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of UART8"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart8(&self) -> EnblWrGroup0ofUart8R {
        EnblWrGroup0ofUart8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of UART8"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart8(&self) -> EnblWrGroup1ofUart8R {
        EnblWrGroup1ofUart8R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of UART8"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart8(&self) -> EnblWrGroup2ofUart8R {
        EnblWrGroup2ofUart8R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of UART8"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart8(&self) -> EnblWrGroup3ofUart8R {
        EnblWrGroup3ofUart8R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of UART8"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart8(&self) -> EnblWrGroup4ofUart8R {
        EnblWrGroup4ofUart8R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of UART8"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart8(&self) -> EnblWrGroup5ofUart8R {
        EnblWrGroup5ofUart8R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1274PRIC1_274\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1274pric12741308(
        &self,
    ) -> EnblRstToleranceOfPric1274pric12741308R {
        EnblRstToleranceOfPric1274pric12741308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1274PRIC1_274\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1274pric12741408(&self) -> EnblWrProtOfPric1274pric12741408R {
        EnblWrProtOfPric1274pric12741408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of UART9"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart9(&self) -> EnblWrGroup0ofUart9R {
        EnblWrGroup0ofUart9R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of UART9"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart9(&self) -> EnblWrGroup1ofUart9R {
        EnblWrGroup1ofUart9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of UART9"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart9(&self) -> EnblWrGroup2ofUart9R {
        EnblWrGroup2ofUart9R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of UART9"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart9(&self) -> EnblWrGroup3ofUart9R {
        EnblWrGroup3ofUart9R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of UART9"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart9(&self) -> EnblWrGroup4ofUart9R {
        EnblWrGroup4ofUart9R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of UART9"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart9(&self) -> EnblWrGroup5ofUart9R {
        EnblWrGroup5ofUart9R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1274PRIC1_274\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1274pric12742116(
        &self,
    ) -> EnblRstToleranceOfPric1274pric12742116R {
        EnblRstToleranceOfPric1274pric12742116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1274PRIC1_274\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1274pric12742216(&self) -> EnblWrProtOfPric1274pric12742216R {
        EnblWrProtOfPric1274pric12742216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of UART10"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart10(&self) -> EnblWrGroup0ofUart10R {
        EnblWrGroup0ofUart10R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of UART10"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart10(&self) -> EnblWrGroup1ofUart10R {
        EnblWrGroup1ofUart10R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of UART10"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart10(&self) -> EnblWrGroup2ofUart10R {
        EnblWrGroup2ofUart10R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of UART10"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart10(&self) -> EnblWrGroup3ofUart10R {
        EnblWrGroup3ofUart10R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of UART10"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart10(&self) -> EnblWrGroup4ofUart10R {
        EnblWrGroup4ofUart10R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of UART10"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart10(&self) -> EnblWrGroup5ofUart10R {
        EnblWrGroup5ofUart10R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1274PRIC1_274\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1274pric12742924(
        &self,
    ) -> EnblRstToleranceOfPric1274pric12742924R {
        EnblRstToleranceOfPric1274pric12742924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1274PRIC1_274\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1274pric12743024(&self) -> EnblWrProtOfPric1274pric12743024R {
        EnblWrProtOfPric1274pric12743024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of UART7"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart7(&mut self) -> EnblWrGroup0ofUart7W<PricIo274Spec> {
        EnblWrGroup0ofUart7W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of UART7"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart7(&mut self) -> EnblWrGroup1ofUart7W<PricIo274Spec> {
        EnblWrGroup1ofUart7W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of UART7"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart7(&mut self) -> EnblWrGroup2ofUart7W<PricIo274Spec> {
        EnblWrGroup2ofUart7W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of UART7"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart7(&mut self) -> EnblWrGroup3ofUart7W<PricIo274Spec> {
        EnblWrGroup3ofUart7W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of UART7"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart7(&mut self) -> EnblWrGroup4ofUart7W<PricIo274Spec> {
        EnblWrGroup4ofUart7W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of UART7"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart7(&mut self) -> EnblWrGroup5ofUart7W<PricIo274Spec> {
        EnblWrGroup5ofUart7W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1274PRIC1_274\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1274pric12740500(
        &mut self,
    ) -> EnblRstToleranceOfPric1274pric12740500W<PricIo274Spec> {
        EnblRstToleranceOfPric1274pric12740500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1274PRIC1_274\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1274pric12740600(
        &mut self,
    ) -> EnblWrProtOfPric1274pric12740600W<PricIo274Spec> {
        EnblWrProtOfPric1274pric12740600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of UART8"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart8(&mut self) -> EnblWrGroup0ofUart8W<PricIo274Spec> {
        EnblWrGroup0ofUart8W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of UART8"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart8(&mut self) -> EnblWrGroup1ofUart8W<PricIo274Spec> {
        EnblWrGroup1ofUart8W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of UART8"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart8(&mut self) -> EnblWrGroup2ofUart8W<PricIo274Spec> {
        EnblWrGroup2ofUart8W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of UART8"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart8(&mut self) -> EnblWrGroup3ofUart8W<PricIo274Spec> {
        EnblWrGroup3ofUart8W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of UART8"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart8(&mut self) -> EnblWrGroup4ofUart8W<PricIo274Spec> {
        EnblWrGroup4ofUart8W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of UART8"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart8(&mut self) -> EnblWrGroup5ofUart8W<PricIo274Spec> {
        EnblWrGroup5ofUart8W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1274PRIC1_274\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1274pric12741308(
        &mut self,
    ) -> EnblRstToleranceOfPric1274pric12741308W<PricIo274Spec> {
        EnblRstToleranceOfPric1274pric12741308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1274PRIC1_274\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1274pric12741408(
        &mut self,
    ) -> EnblWrProtOfPric1274pric12741408W<PricIo274Spec> {
        EnblWrProtOfPric1274pric12741408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of UART9"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart9(&mut self) -> EnblWrGroup0ofUart9W<PricIo274Spec> {
        EnblWrGroup0ofUart9W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of UART9"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart9(&mut self) -> EnblWrGroup1ofUart9W<PricIo274Spec> {
        EnblWrGroup1ofUart9W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of UART9"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart9(&mut self) -> EnblWrGroup2ofUart9W<PricIo274Spec> {
        EnblWrGroup2ofUart9W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of UART9"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart9(&mut self) -> EnblWrGroup3ofUart9W<PricIo274Spec> {
        EnblWrGroup3ofUart9W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of UART9"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart9(&mut self) -> EnblWrGroup4ofUart9W<PricIo274Spec> {
        EnblWrGroup4ofUart9W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of UART9"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart9(&mut self) -> EnblWrGroup5ofUart9W<PricIo274Spec> {
        EnblWrGroup5ofUart9W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1274PRIC1_274\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1274pric12742116(
        &mut self,
    ) -> EnblRstToleranceOfPric1274pric12742116W<PricIo274Spec> {
        EnblRstToleranceOfPric1274pric12742116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1274PRIC1_274\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1274pric12742216(
        &mut self,
    ) -> EnblWrProtOfPric1274pric12742216W<PricIo274Spec> {
        EnblWrProtOfPric1274pric12742216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of UART10"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart10(&mut self) -> EnblWrGroup0ofUart10W<PricIo274Spec> {
        EnblWrGroup0ofUart10W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of UART10"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart10(&mut self) -> EnblWrGroup1ofUart10W<PricIo274Spec> {
        EnblWrGroup1ofUart10W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of UART10"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart10(&mut self) -> EnblWrGroup2ofUart10W<PricIo274Spec> {
        EnblWrGroup2ofUart10W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of UART10"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart10(&mut self) -> EnblWrGroup3ofUart10W<PricIo274Spec> {
        EnblWrGroup3ofUart10W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of UART10"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart10(&mut self) -> EnblWrGroup4ofUart10W<PricIo274Spec> {
        EnblWrGroup4ofUart10W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of UART10"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart10(&mut self) -> EnblWrGroup5ofUart10W<PricIo274Spec> {
        EnblWrGroup5ofUart10W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1274PRIC1_274\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1274pric12742924(
        &mut self,
    ) -> EnblRstToleranceOfPric1274pric12742924W<PricIo274Spec> {
        EnblRstToleranceOfPric1274pric12742924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1274PRIC1_274\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1274pric12743024(
        &mut self,
    ) -> EnblWrProtOfPric1274pric12743024W<PricIo274Spec> {
        EnblWrProtOfPric1274pric12743024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#29\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io274::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io274::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo274Spec;
impl crate::RegisterSpec for PricIo274Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io274::R`](R) reader structure"]
impl crate::Readable for PricIo274Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io274::W`](W) writer structure"]
impl crate::Writable for PricIo274Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO274 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo274Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
