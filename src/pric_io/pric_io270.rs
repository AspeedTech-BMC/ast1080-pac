#[doc = "Register `PRIC_IO270` reader"]
pub type R = crate::R<PricIo270Spec>;
#[doc = "Register `PRIC_IO270` writer"]
pub type W = crate::W<PricIo270Spec>;
#[doc = "Field `EnblWrGroup0OfUART3` reader - Enable Write Group #0 of UART3"]
pub type EnblWrGroup0ofUart3R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUART3` writer - Enable Write Group #0 of UART3"]
pub type EnblWrGroup0ofUart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUART3` reader - Enable Write Group #1 of UART3"]
pub type EnblWrGroup1ofUart3R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUART3` writer - Enable Write Group #1 of UART3"]
pub type EnblWrGroup1ofUart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUART3` reader - Enable Write Group #2 of UART3"]
pub type EnblWrGroup2ofUart3R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUART3` writer - Enable Write Group #2 of UART3"]
pub type EnblWrGroup2ofUart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUART3` reader - Enable Write Group #3 of UART3"]
pub type EnblWrGroup3ofUart3R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUART3` writer - Enable Write Group #3 of UART3"]
pub type EnblWrGroup3ofUart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUART3` reader - Enable Write Group #4 of UART3"]
pub type EnblWrGroup4ofUart3R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUART3` writer - Enable Write Group #4 of UART3"]
pub type EnblWrGroup4ofUart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUART3` reader - Enable Write Group #5 of UART3"]
pub type EnblWrGroup5ofUart3R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUART3` writer - Enable Write Group #5 of UART3"]
pub type EnblWrGroup5ofUart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1270PRIC1_270\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1270pric12700500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1270pric12700500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1270pric12700500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1270PRIC12700500` reader - Enable Reset Tolerance of PRIC1270PRIC1_270\\[05:00\\]"]
pub type EnblRstToleranceOfPric1270pric12700500R =
    crate::BitReader<EnblRstToleranceOfPric1270pric12700500>;
impl EnblRstToleranceOfPric1270pric12700500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1270pric12700500 {
        match self.bits {
            false => EnblRstToleranceOfPric1270pric12700500::ResetBySrst,
            true => EnblRstToleranceOfPric1270pric12700500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1270pric12700500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1270pric12700500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1270PRIC12700500` writer - Enable Reset Tolerance of PRIC1270PRIC1_270\\[05:00\\]"]
pub type EnblRstToleranceOfPric1270pric12700500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1270pric12700500>;
impl<'a, REG> EnblRstToleranceOfPric1270pric12700500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1270pric12700500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1270pric12700500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1270PRIC12700600` reader - Enable Write Protection of PRIC1270PRIC1_270\\[06:00\\]"]
pub type EnblWrProtOfPric1270pric12700600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1270PRIC12700600` writer - Enable Write Protection of PRIC1270PRIC1_270\\[06:00\\]"]
pub type EnblWrProtOfPric1270pric12700600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfUART4` reader - Enable Write Group #0 of UART4"]
pub type EnblWrGroup0ofUart4R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUART4` writer - Enable Write Group #0 of UART4"]
pub type EnblWrGroup0ofUart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUART4` reader - Enable Write Group #1 of UART4"]
pub type EnblWrGroup1ofUart4R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUART4` writer - Enable Write Group #1 of UART4"]
pub type EnblWrGroup1ofUart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUART4` reader - Enable Write Group #2 of UART4"]
pub type EnblWrGroup2ofUart4R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUART4` writer - Enable Write Group #2 of UART4"]
pub type EnblWrGroup2ofUart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUART4` reader - Enable Write Group #3 of UART4"]
pub type EnblWrGroup3ofUart4R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUART4` writer - Enable Write Group #3 of UART4"]
pub type EnblWrGroup3ofUart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUART4` reader - Enable Write Group #4 of UART4"]
pub type EnblWrGroup4ofUart4R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUART4` writer - Enable Write Group #4 of UART4"]
pub type EnblWrGroup4ofUart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUART4` reader - Enable Write Group #5 of UART4"]
pub type EnblWrGroup5ofUart4R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUART4` writer - Enable Write Group #5 of UART4"]
pub type EnblWrGroup5ofUart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1270PRIC1_270\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1270pric12701308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1270pric12701308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1270pric12701308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1270PRIC12701308` reader - Enable Reset Tolerance of PRIC1270PRIC1_270\\[13:08\\]"]
pub type EnblRstToleranceOfPric1270pric12701308R =
    crate::BitReader<EnblRstToleranceOfPric1270pric12701308>;
impl EnblRstToleranceOfPric1270pric12701308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1270pric12701308 {
        match self.bits {
            false => EnblRstToleranceOfPric1270pric12701308::ResetBySrst,
            true => EnblRstToleranceOfPric1270pric12701308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1270pric12701308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1270pric12701308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1270PRIC12701308` writer - Enable Reset Tolerance of PRIC1270PRIC1_270\\[13:08\\]"]
pub type EnblRstToleranceOfPric1270pric12701308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1270pric12701308>;
impl<'a, REG> EnblRstToleranceOfPric1270pric12701308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1270pric12701308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1270pric12701308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1270PRIC12701408` reader - Enable Write Protection of PRIC1270PRIC1_270\\[14:08\\]"]
pub type EnblWrProtOfPric1270pric12701408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1270PRIC12701408` writer - Enable Write Protection of PRIC1270PRIC1_270\\[14:08\\]"]
pub type EnblWrProtOfPric1270pric12701408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfUART5` reader - Enable Write Group #0 of UART5"]
pub type EnblWrGroup0ofUart5R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUART5` writer - Enable Write Group #0 of UART5"]
pub type EnblWrGroup0ofUart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUART5` reader - Enable Write Group #1 of UART5"]
pub type EnblWrGroup1ofUart5R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUART5` writer - Enable Write Group #1 of UART5"]
pub type EnblWrGroup1ofUart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUART5` reader - Enable Write Group #2 of UART5"]
pub type EnblWrGroup2ofUart5R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUART5` writer - Enable Write Group #2 of UART5"]
pub type EnblWrGroup2ofUart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUART5` reader - Enable Write Group #3 of UART5"]
pub type EnblWrGroup3ofUart5R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUART5` writer - Enable Write Group #3 of UART5"]
pub type EnblWrGroup3ofUart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUART5` reader - Enable Write Group #4 of UART5"]
pub type EnblWrGroup4ofUart5R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUART5` writer - Enable Write Group #4 of UART5"]
pub type EnblWrGroup4ofUart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUART5` reader - Enable Write Group #5 of UART5"]
pub type EnblWrGroup5ofUart5R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUART5` writer - Enable Write Group #5 of UART5"]
pub type EnblWrGroup5ofUart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1270PRIC1_270\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1270pric12702116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1270pric12702116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1270pric12702116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1270PRIC12702116` reader - Enable Reset Tolerance of PRIC1270PRIC1_270\\[21:16\\]"]
pub type EnblRstToleranceOfPric1270pric12702116R =
    crate::BitReader<EnblRstToleranceOfPric1270pric12702116>;
impl EnblRstToleranceOfPric1270pric12702116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1270pric12702116 {
        match self.bits {
            false => EnblRstToleranceOfPric1270pric12702116::ResetBySrst,
            true => EnblRstToleranceOfPric1270pric12702116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1270pric12702116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1270pric12702116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1270PRIC12702116` writer - Enable Reset Tolerance of PRIC1270PRIC1_270\\[21:16\\]"]
pub type EnblRstToleranceOfPric1270pric12702116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1270pric12702116>;
impl<'a, REG> EnblRstToleranceOfPric1270pric12702116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1270pric12702116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1270pric12702116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1270PRIC12702216` reader - Enable Write Protection of PRIC1270PRIC1_270\\[22:16\\]"]
pub type EnblWrProtOfPric1270pric12702216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1270PRIC12702216` writer - Enable Write Protection of PRIC1270PRIC1_270\\[22:16\\]"]
pub type EnblWrProtOfPric1270pric12702216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfUART6` reader - Enable Write Group #0 of UART6"]
pub type EnblWrGroup0ofUart6R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUART6` writer - Enable Write Group #0 of UART6"]
pub type EnblWrGroup0ofUart6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUART6` reader - Enable Write Group #1 of UART6"]
pub type EnblWrGroup1ofUart6R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUART6` writer - Enable Write Group #1 of UART6"]
pub type EnblWrGroup1ofUart6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUART6` reader - Enable Write Group #2 of UART6"]
pub type EnblWrGroup2ofUart6R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUART6` writer - Enable Write Group #2 of UART6"]
pub type EnblWrGroup2ofUart6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUART6` reader - Enable Write Group #3 of UART6"]
pub type EnblWrGroup3ofUart6R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUART6` writer - Enable Write Group #3 of UART6"]
pub type EnblWrGroup3ofUart6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUART6` reader - Enable Write Group #4 of UART6"]
pub type EnblWrGroup4ofUart6R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUART6` writer - Enable Write Group #4 of UART6"]
pub type EnblWrGroup4ofUart6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUART6` reader - Enable Write Group #5 of UART6"]
pub type EnblWrGroup5ofUart6R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUART6` writer - Enable Write Group #5 of UART6"]
pub type EnblWrGroup5ofUart6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1270PRIC1_270\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1270pric12702924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1270pric12702924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1270pric12702924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1270PRIC12702924` reader - Enable Reset Tolerance of PRIC1270PRIC1_270\\[29:24\\]"]
pub type EnblRstToleranceOfPric1270pric12702924R =
    crate::BitReader<EnblRstToleranceOfPric1270pric12702924>;
impl EnblRstToleranceOfPric1270pric12702924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1270pric12702924 {
        match self.bits {
            false => EnblRstToleranceOfPric1270pric12702924::ResetBySrst,
            true => EnblRstToleranceOfPric1270pric12702924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1270pric12702924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1270pric12702924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1270PRIC12702924` writer - Enable Reset Tolerance of PRIC1270PRIC1_270\\[29:24\\]"]
pub type EnblRstToleranceOfPric1270pric12702924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1270pric12702924>;
impl<'a, REG> EnblRstToleranceOfPric1270pric12702924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1270pric12702924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1270pric12702924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1270PRIC12703024` reader - Enable Write Protection of PRIC1270PRIC1_270\\[30:24\\]"]
pub type EnblWrProtOfPric1270pric12703024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1270PRIC12703024` writer - Enable Write Protection of PRIC1270PRIC1_270\\[30:24\\]"]
pub type EnblWrProtOfPric1270pric12703024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of UART3"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart3(&self) -> EnblWrGroup0ofUart3R {
        EnblWrGroup0ofUart3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of UART3"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart3(&self) -> EnblWrGroup1ofUart3R {
        EnblWrGroup1ofUart3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of UART3"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart3(&self) -> EnblWrGroup2ofUart3R {
        EnblWrGroup2ofUart3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of UART3"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart3(&self) -> EnblWrGroup3ofUart3R {
        EnblWrGroup3ofUart3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of UART3"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart3(&self) -> EnblWrGroup4ofUart3R {
        EnblWrGroup4ofUart3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of UART3"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart3(&self) -> EnblWrGroup5ofUart3R {
        EnblWrGroup5ofUart3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1270PRIC1_270\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1270pric12700500(
        &self,
    ) -> EnblRstToleranceOfPric1270pric12700500R {
        EnblRstToleranceOfPric1270pric12700500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1270PRIC1_270\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1270pric12700600(&self) -> EnblWrProtOfPric1270pric12700600R {
        EnblWrProtOfPric1270pric12700600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of UART4"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart4(&self) -> EnblWrGroup0ofUart4R {
        EnblWrGroup0ofUart4R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of UART4"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart4(&self) -> EnblWrGroup1ofUart4R {
        EnblWrGroup1ofUart4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of UART4"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart4(&self) -> EnblWrGroup2ofUart4R {
        EnblWrGroup2ofUart4R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of UART4"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart4(&self) -> EnblWrGroup3ofUart4R {
        EnblWrGroup3ofUart4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of UART4"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart4(&self) -> EnblWrGroup4ofUart4R {
        EnblWrGroup4ofUart4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of UART4"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart4(&self) -> EnblWrGroup5ofUart4R {
        EnblWrGroup5ofUart4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1270PRIC1_270\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1270pric12701308(
        &self,
    ) -> EnblRstToleranceOfPric1270pric12701308R {
        EnblRstToleranceOfPric1270pric12701308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1270PRIC1_270\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1270pric12701408(&self) -> EnblWrProtOfPric1270pric12701408R {
        EnblWrProtOfPric1270pric12701408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of UART5"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart5(&self) -> EnblWrGroup0ofUart5R {
        EnblWrGroup0ofUart5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of UART5"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart5(&self) -> EnblWrGroup1ofUart5R {
        EnblWrGroup1ofUart5R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of UART5"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart5(&self) -> EnblWrGroup2ofUart5R {
        EnblWrGroup2ofUart5R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of UART5"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart5(&self) -> EnblWrGroup3ofUart5R {
        EnblWrGroup3ofUart5R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of UART5"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart5(&self) -> EnblWrGroup4ofUart5R {
        EnblWrGroup4ofUart5R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of UART5"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart5(&self) -> EnblWrGroup5ofUart5R {
        EnblWrGroup5ofUart5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1270PRIC1_270\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1270pric12702116(
        &self,
    ) -> EnblRstToleranceOfPric1270pric12702116R {
        EnblRstToleranceOfPric1270pric12702116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1270PRIC1_270\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1270pric12702216(&self) -> EnblWrProtOfPric1270pric12702216R {
        EnblWrProtOfPric1270pric12702216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of UART6"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart6(&self) -> EnblWrGroup0ofUart6R {
        EnblWrGroup0ofUart6R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of UART6"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart6(&self) -> EnblWrGroup1ofUart6R {
        EnblWrGroup1ofUart6R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of UART6"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart6(&self) -> EnblWrGroup2ofUart6R {
        EnblWrGroup2ofUart6R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of UART6"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart6(&self) -> EnblWrGroup3ofUart6R {
        EnblWrGroup3ofUart6R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of UART6"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart6(&self) -> EnblWrGroup4ofUart6R {
        EnblWrGroup4ofUart6R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of UART6"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart6(&self) -> EnblWrGroup5ofUart6R {
        EnblWrGroup5ofUart6R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1270PRIC1_270\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1270pric12702924(
        &self,
    ) -> EnblRstToleranceOfPric1270pric12702924R {
        EnblRstToleranceOfPric1270pric12702924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1270PRIC1_270\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1270pric12703024(&self) -> EnblWrProtOfPric1270pric12703024R {
        EnblWrProtOfPric1270pric12703024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of UART3"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart3(&mut self) -> EnblWrGroup0ofUart3W<PricIo270Spec> {
        EnblWrGroup0ofUart3W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of UART3"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart3(&mut self) -> EnblWrGroup1ofUart3W<PricIo270Spec> {
        EnblWrGroup1ofUart3W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of UART3"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart3(&mut self) -> EnblWrGroup2ofUart3W<PricIo270Spec> {
        EnblWrGroup2ofUart3W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of UART3"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart3(&mut self) -> EnblWrGroup3ofUart3W<PricIo270Spec> {
        EnblWrGroup3ofUart3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of UART3"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart3(&mut self) -> EnblWrGroup4ofUart3W<PricIo270Spec> {
        EnblWrGroup4ofUart3W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of UART3"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart3(&mut self) -> EnblWrGroup5ofUart3W<PricIo270Spec> {
        EnblWrGroup5ofUart3W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1270PRIC1_270\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1270pric12700500(
        &mut self,
    ) -> EnblRstToleranceOfPric1270pric12700500W<PricIo270Spec> {
        EnblRstToleranceOfPric1270pric12700500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1270PRIC1_270\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1270pric12700600(
        &mut self,
    ) -> EnblWrProtOfPric1270pric12700600W<PricIo270Spec> {
        EnblWrProtOfPric1270pric12700600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of UART4"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart4(&mut self) -> EnblWrGroup0ofUart4W<PricIo270Spec> {
        EnblWrGroup0ofUart4W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of UART4"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart4(&mut self) -> EnblWrGroup1ofUart4W<PricIo270Spec> {
        EnblWrGroup1ofUart4W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of UART4"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart4(&mut self) -> EnblWrGroup2ofUart4W<PricIo270Spec> {
        EnblWrGroup2ofUart4W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of UART4"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart4(&mut self) -> EnblWrGroup3ofUart4W<PricIo270Spec> {
        EnblWrGroup3ofUart4W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of UART4"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart4(&mut self) -> EnblWrGroup4ofUart4W<PricIo270Spec> {
        EnblWrGroup4ofUart4W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of UART4"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart4(&mut self) -> EnblWrGroup5ofUart4W<PricIo270Spec> {
        EnblWrGroup5ofUart4W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1270PRIC1_270\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1270pric12701308(
        &mut self,
    ) -> EnblRstToleranceOfPric1270pric12701308W<PricIo270Spec> {
        EnblRstToleranceOfPric1270pric12701308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1270PRIC1_270\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1270pric12701408(
        &mut self,
    ) -> EnblWrProtOfPric1270pric12701408W<PricIo270Spec> {
        EnblWrProtOfPric1270pric12701408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of UART5"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart5(&mut self) -> EnblWrGroup0ofUart5W<PricIo270Spec> {
        EnblWrGroup0ofUart5W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of UART5"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart5(&mut self) -> EnblWrGroup1ofUart5W<PricIo270Spec> {
        EnblWrGroup1ofUart5W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of UART5"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart5(&mut self) -> EnblWrGroup2ofUart5W<PricIo270Spec> {
        EnblWrGroup2ofUart5W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of UART5"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart5(&mut self) -> EnblWrGroup3ofUart5W<PricIo270Spec> {
        EnblWrGroup3ofUart5W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of UART5"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart5(&mut self) -> EnblWrGroup4ofUart5W<PricIo270Spec> {
        EnblWrGroup4ofUart5W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of UART5"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart5(&mut self) -> EnblWrGroup5ofUart5W<PricIo270Spec> {
        EnblWrGroup5ofUart5W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1270PRIC1_270\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1270pric12702116(
        &mut self,
    ) -> EnblRstToleranceOfPric1270pric12702116W<PricIo270Spec> {
        EnblRstToleranceOfPric1270pric12702116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1270PRIC1_270\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1270pric12702216(
        &mut self,
    ) -> EnblWrProtOfPric1270pric12702216W<PricIo270Spec> {
        EnblWrProtOfPric1270pric12702216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of UART6"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart6(&mut self) -> EnblWrGroup0ofUart6W<PricIo270Spec> {
        EnblWrGroup0ofUart6W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of UART6"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart6(&mut self) -> EnblWrGroup1ofUart6W<PricIo270Spec> {
        EnblWrGroup1ofUart6W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of UART6"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart6(&mut self) -> EnblWrGroup2ofUart6W<PricIo270Spec> {
        EnblWrGroup2ofUart6W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of UART6"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart6(&mut self) -> EnblWrGroup3ofUart6W<PricIo270Spec> {
        EnblWrGroup3ofUart6W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of UART6"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart6(&mut self) -> EnblWrGroup4ofUart6W<PricIo270Spec> {
        EnblWrGroup4ofUart6W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of UART6"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart6(&mut self) -> EnblWrGroup5ofUart6W<PricIo270Spec> {
        EnblWrGroup5ofUart6W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1270PRIC1_270\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1270pric12702924(
        &mut self,
    ) -> EnblRstToleranceOfPric1270pric12702924W<PricIo270Spec> {
        EnblRstToleranceOfPric1270pric12702924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1270PRIC1_270\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1270pric12703024(
        &mut self,
    ) -> EnblWrProtOfPric1270pric12703024W<PricIo270Spec> {
        EnblWrProtOfPric1270pric12703024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io270::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io270::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo270Spec;
impl crate::RegisterSpec for PricIo270Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io270::R`](R) reader structure"]
impl crate::Readable for PricIo270Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io270::W`](W) writer structure"]
impl crate::Writable for PricIo270Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO270 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo270Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
