#[doc = "Register `PRIC_IO370` reader"]
pub type R = crate::R<PricIo370Spec>;
#[doc = "Register `PRIC_IO370` writer"]
pub type W = crate::W<PricIo370Spec>;
#[doc = "Field `EnblReadGroup0OfUART3` reader - Enable Read Group #0 of UART3"]
pub type EnblReadGroup0ofUart3R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUART3` writer - Enable Read Group #0 of UART3"]
pub type EnblReadGroup0ofUart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUART3` reader - Enable Read Group #1 of UART3"]
pub type EnblReadGroup1ofUart3R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUART3` writer - Enable Read Group #1 of UART3"]
pub type EnblReadGroup1ofUart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUART3` reader - Enable Read Group #2 of UART3"]
pub type EnblReadGroup2ofUart3R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUART3` writer - Enable Read Group #2 of UART3"]
pub type EnblReadGroup2ofUart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUART3` reader - Enable Read Group #3 of UART3"]
pub type EnblReadGroup3ofUart3R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUART3` writer - Enable Read Group #3 of UART3"]
pub type EnblReadGroup3ofUart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUART3` reader - Enable Read Group #4 of UART3"]
pub type EnblReadGroup4ofUart3R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUART3` writer - Enable Read Group #4 of UART3"]
pub type EnblReadGroup4ofUart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUART3` reader - Enable Read Group #5 of UART3"]
pub type EnblReadGroup5ofUart3R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUART3` writer - Enable Read Group #5 of UART3"]
pub type EnblReadGroup5ofUart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1370PRIC1_370\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1370pric13700500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1370pric13700500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1370pric13700500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1370PRIC13700500` reader - Enable Reset Tolerance of PRIC1370PRIC1_370\\[05:00\\]"]
pub type EnblRstToleranceOfPric1370pric13700500R =
    crate::BitReader<EnblRstToleranceOfPric1370pric13700500>;
impl EnblRstToleranceOfPric1370pric13700500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1370pric13700500 {
        match self.bits {
            false => EnblRstToleranceOfPric1370pric13700500::ResetBySrst,
            true => EnblRstToleranceOfPric1370pric13700500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1370pric13700500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1370pric13700500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1370PRIC13700500` writer - Enable Reset Tolerance of PRIC1370PRIC1_370\\[05:00\\]"]
pub type EnblRstToleranceOfPric1370pric13700500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1370pric13700500>;
impl<'a, REG> EnblRstToleranceOfPric1370pric13700500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1370pric13700500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1370pric13700500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1370PRIC13700600` reader - Enable Write Protection of PRIC1370PRIC1_370\\[06:00\\]"]
pub type EnblWrProtOfPric1370pric13700600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1370PRIC13700600` writer - Enable Write Protection of PRIC1370PRIC1_370\\[06:00\\]"]
pub type EnblWrProtOfPric1370pric13700600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfUART4` reader - Enable Read Group #0 of UART4"]
pub type EnblReadGroup0ofUart4R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUART4` writer - Enable Read Group #0 of UART4"]
pub type EnblReadGroup0ofUart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUART4` reader - Enable Read Group #1 of UART4"]
pub type EnblReadGroup1ofUart4R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUART4` writer - Enable Read Group #1 of UART4"]
pub type EnblReadGroup1ofUart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUART4` reader - Enable Read Group #2 of UART4"]
pub type EnblReadGroup2ofUart4R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUART4` writer - Enable Read Group #2 of UART4"]
pub type EnblReadGroup2ofUart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUART4` reader - Enable Read Group #3 of UART4"]
pub type EnblReadGroup3ofUart4R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUART4` writer - Enable Read Group #3 of UART4"]
pub type EnblReadGroup3ofUart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUART4` reader - Enable Read Group #4 of UART4"]
pub type EnblReadGroup4ofUart4R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUART4` writer - Enable Read Group #4 of UART4"]
pub type EnblReadGroup4ofUart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUART4` reader - Enable Read Group #5 of UART4"]
pub type EnblReadGroup5ofUart4R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUART4` writer - Enable Read Group #5 of UART4"]
pub type EnblReadGroup5ofUart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1370PRIC1_370\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1370pric13701308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1370pric13701308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1370pric13701308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1370PRIC13701308` reader - Enable Reset Tolerance of PRIC1370PRIC1_370\\[13:08\\]"]
pub type EnblRstToleranceOfPric1370pric13701308R =
    crate::BitReader<EnblRstToleranceOfPric1370pric13701308>;
impl EnblRstToleranceOfPric1370pric13701308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1370pric13701308 {
        match self.bits {
            false => EnblRstToleranceOfPric1370pric13701308::ResetBySrst,
            true => EnblRstToleranceOfPric1370pric13701308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1370pric13701308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1370pric13701308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1370PRIC13701308` writer - Enable Reset Tolerance of PRIC1370PRIC1_370\\[13:08\\]"]
pub type EnblRstToleranceOfPric1370pric13701308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1370pric13701308>;
impl<'a, REG> EnblRstToleranceOfPric1370pric13701308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1370pric13701308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1370pric13701308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1370PRIC13701408` reader - Enable Write Protection of PRIC1370PRIC1_370\\[14:08\\]"]
pub type EnblWrProtOfPric1370pric13701408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1370PRIC13701408` writer - Enable Write Protection of PRIC1370PRIC1_370\\[14:08\\]"]
pub type EnblWrProtOfPric1370pric13701408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfUART5` reader - Enable Read Group #0 of UART5"]
pub type EnblReadGroup0ofUart5R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUART5` writer - Enable Read Group #0 of UART5"]
pub type EnblReadGroup0ofUart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUART5` reader - Enable Read Group #1 of UART5"]
pub type EnblReadGroup1ofUart5R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUART5` writer - Enable Read Group #1 of UART5"]
pub type EnblReadGroup1ofUart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUART5` reader - Enable Read Group #2 of UART5"]
pub type EnblReadGroup2ofUart5R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUART5` writer - Enable Read Group #2 of UART5"]
pub type EnblReadGroup2ofUart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUART5` reader - Enable Read Group #3 of UART5"]
pub type EnblReadGroup3ofUart5R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUART5` writer - Enable Read Group #3 of UART5"]
pub type EnblReadGroup3ofUart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUART5` reader - Enable Read Group #4 of UART5"]
pub type EnblReadGroup4ofUart5R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUART5` writer - Enable Read Group #4 of UART5"]
pub type EnblReadGroup4ofUart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUART5` reader - Enable Read Group #5 of UART5"]
pub type EnblReadGroup5ofUart5R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUART5` writer - Enable Read Group #5 of UART5"]
pub type EnblReadGroup5ofUart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1370PRIC1_370\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1370pric13702116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1370pric13702116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1370pric13702116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1370PRIC13702116` reader - Enable Reset Tolerance of PRIC1370PRIC1_370\\[21:16\\]"]
pub type EnblRstToleranceOfPric1370pric13702116R =
    crate::BitReader<EnblRstToleranceOfPric1370pric13702116>;
impl EnblRstToleranceOfPric1370pric13702116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1370pric13702116 {
        match self.bits {
            false => EnblRstToleranceOfPric1370pric13702116::ResetBySrst,
            true => EnblRstToleranceOfPric1370pric13702116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1370pric13702116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1370pric13702116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1370PRIC13702116` writer - Enable Reset Tolerance of PRIC1370PRIC1_370\\[21:16\\]"]
pub type EnblRstToleranceOfPric1370pric13702116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1370pric13702116>;
impl<'a, REG> EnblRstToleranceOfPric1370pric13702116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1370pric13702116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1370pric13702116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1370PRIC13702216` reader - Enable Write Protection of PRIC1370PRIC1_370\\[22:16\\]"]
pub type EnblWrProtOfPric1370pric13702216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1370PRIC13702216` writer - Enable Write Protection of PRIC1370PRIC1_370\\[22:16\\]"]
pub type EnblWrProtOfPric1370pric13702216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfUART6` reader - Enable Read Group #0 of UART6"]
pub type EnblReadGroup0ofUart6R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUART6` writer - Enable Read Group #0 of UART6"]
pub type EnblReadGroup0ofUart6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUART6` reader - Enable Read Group #1 of UART6"]
pub type EnblReadGroup1ofUart6R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUART6` writer - Enable Read Group #1 of UART6"]
pub type EnblReadGroup1ofUart6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUART6` reader - Enable Read Group #2 of UART6"]
pub type EnblReadGroup2ofUart6R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUART6` writer - Enable Read Group #2 of UART6"]
pub type EnblReadGroup2ofUart6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUART6` reader - Enable Read Group #3 of UART6"]
pub type EnblReadGroup3ofUart6R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUART6` writer - Enable Read Group #3 of UART6"]
pub type EnblReadGroup3ofUart6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUART6` reader - Enable Read Group #4 of UART6"]
pub type EnblReadGroup4ofUart6R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUART6` writer - Enable Read Group #4 of UART6"]
pub type EnblReadGroup4ofUart6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUART6` reader - Enable Read Group #5 of UART6"]
pub type EnblReadGroup5ofUart6R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUART6` writer - Enable Read Group #5 of UART6"]
pub type EnblReadGroup5ofUart6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1370PRIC1_370\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1370pric13702924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1370pric13702924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1370pric13702924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1370PRIC13702924` reader - Enable Reset Tolerance of PRIC1370PRIC1_370\\[29:24\\]"]
pub type EnblRstToleranceOfPric1370pric13702924R =
    crate::BitReader<EnblRstToleranceOfPric1370pric13702924>;
impl EnblRstToleranceOfPric1370pric13702924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1370pric13702924 {
        match self.bits {
            false => EnblRstToleranceOfPric1370pric13702924::ResetBySrst,
            true => EnblRstToleranceOfPric1370pric13702924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1370pric13702924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1370pric13702924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1370PRIC13702924` writer - Enable Reset Tolerance of PRIC1370PRIC1_370\\[29:24\\]"]
pub type EnblRstToleranceOfPric1370pric13702924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1370pric13702924>;
impl<'a, REG> EnblRstToleranceOfPric1370pric13702924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1370pric13702924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1370pric13702924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1370PRIC13703024` reader - Enable Write Protection of PRIC1370PRIC1_370\\[30:24\\]"]
pub type EnblWrProtOfPric1370pric13703024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1370PRIC13703024` writer - Enable Write Protection of PRIC1370PRIC1_370\\[30:24\\]"]
pub type EnblWrProtOfPric1370pric13703024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of UART3"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart3(&self) -> EnblReadGroup0ofUart3R {
        EnblReadGroup0ofUart3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of UART3"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart3(&self) -> EnblReadGroup1ofUart3R {
        EnblReadGroup1ofUart3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of UART3"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart3(&self) -> EnblReadGroup2ofUart3R {
        EnblReadGroup2ofUart3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of UART3"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart3(&self) -> EnblReadGroup3ofUart3R {
        EnblReadGroup3ofUart3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of UART3"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart3(&self) -> EnblReadGroup4ofUart3R {
        EnblReadGroup4ofUart3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of UART3"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart3(&self) -> EnblReadGroup5ofUart3R {
        EnblReadGroup5ofUart3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1370PRIC1_370\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1370pric13700500(
        &self,
    ) -> EnblRstToleranceOfPric1370pric13700500R {
        EnblRstToleranceOfPric1370pric13700500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1370PRIC1_370\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1370pric13700600(&self) -> EnblWrProtOfPric1370pric13700600R {
        EnblWrProtOfPric1370pric13700600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of UART4"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart4(&self) -> EnblReadGroup0ofUart4R {
        EnblReadGroup0ofUart4R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of UART4"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart4(&self) -> EnblReadGroup1ofUart4R {
        EnblReadGroup1ofUart4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of UART4"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart4(&self) -> EnblReadGroup2ofUart4R {
        EnblReadGroup2ofUart4R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of UART4"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart4(&self) -> EnblReadGroup3ofUart4R {
        EnblReadGroup3ofUart4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of UART4"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart4(&self) -> EnblReadGroup4ofUart4R {
        EnblReadGroup4ofUart4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of UART4"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart4(&self) -> EnblReadGroup5ofUart4R {
        EnblReadGroup5ofUart4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1370PRIC1_370\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1370pric13701308(
        &self,
    ) -> EnblRstToleranceOfPric1370pric13701308R {
        EnblRstToleranceOfPric1370pric13701308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1370PRIC1_370\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1370pric13701408(&self) -> EnblWrProtOfPric1370pric13701408R {
        EnblWrProtOfPric1370pric13701408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of UART5"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart5(&self) -> EnblReadGroup0ofUart5R {
        EnblReadGroup0ofUart5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of UART5"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart5(&self) -> EnblReadGroup1ofUart5R {
        EnblReadGroup1ofUart5R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of UART5"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart5(&self) -> EnblReadGroup2ofUart5R {
        EnblReadGroup2ofUart5R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of UART5"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart5(&self) -> EnblReadGroup3ofUart5R {
        EnblReadGroup3ofUart5R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of UART5"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart5(&self) -> EnblReadGroup4ofUart5R {
        EnblReadGroup4ofUart5R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of UART5"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart5(&self) -> EnblReadGroup5ofUart5R {
        EnblReadGroup5ofUart5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1370PRIC1_370\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1370pric13702116(
        &self,
    ) -> EnblRstToleranceOfPric1370pric13702116R {
        EnblRstToleranceOfPric1370pric13702116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1370PRIC1_370\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1370pric13702216(&self) -> EnblWrProtOfPric1370pric13702216R {
        EnblWrProtOfPric1370pric13702216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of UART6"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart6(&self) -> EnblReadGroup0ofUart6R {
        EnblReadGroup0ofUart6R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of UART6"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart6(&self) -> EnblReadGroup1ofUart6R {
        EnblReadGroup1ofUart6R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of UART6"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart6(&self) -> EnblReadGroup2ofUart6R {
        EnblReadGroup2ofUart6R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of UART6"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart6(&self) -> EnblReadGroup3ofUart6R {
        EnblReadGroup3ofUart6R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of UART6"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart6(&self) -> EnblReadGroup4ofUart6R {
        EnblReadGroup4ofUart6R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of UART6"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart6(&self) -> EnblReadGroup5ofUart6R {
        EnblReadGroup5ofUart6R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1370PRIC1_370\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1370pric13702924(
        &self,
    ) -> EnblRstToleranceOfPric1370pric13702924R {
        EnblRstToleranceOfPric1370pric13702924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1370PRIC1_370\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1370pric13703024(&self) -> EnblWrProtOfPric1370pric13703024R {
        EnblWrProtOfPric1370pric13703024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of UART3"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart3(&mut self) -> EnblReadGroup0ofUart3W<PricIo370Spec> {
        EnblReadGroup0ofUart3W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of UART3"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart3(&mut self) -> EnblReadGroup1ofUart3W<PricIo370Spec> {
        EnblReadGroup1ofUart3W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of UART3"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart3(&mut self) -> EnblReadGroup2ofUart3W<PricIo370Spec> {
        EnblReadGroup2ofUart3W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of UART3"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart3(&mut self) -> EnblReadGroup3ofUart3W<PricIo370Spec> {
        EnblReadGroup3ofUart3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of UART3"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart3(&mut self) -> EnblReadGroup4ofUart3W<PricIo370Spec> {
        EnblReadGroup4ofUart3W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of UART3"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart3(&mut self) -> EnblReadGroup5ofUart3W<PricIo370Spec> {
        EnblReadGroup5ofUart3W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1370PRIC1_370\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1370pric13700500(
        &mut self,
    ) -> EnblRstToleranceOfPric1370pric13700500W<PricIo370Spec> {
        EnblRstToleranceOfPric1370pric13700500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1370PRIC1_370\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1370pric13700600(
        &mut self,
    ) -> EnblWrProtOfPric1370pric13700600W<PricIo370Spec> {
        EnblWrProtOfPric1370pric13700600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of UART4"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart4(&mut self) -> EnblReadGroup0ofUart4W<PricIo370Spec> {
        EnblReadGroup0ofUart4W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of UART4"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart4(&mut self) -> EnblReadGroup1ofUart4W<PricIo370Spec> {
        EnblReadGroup1ofUart4W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of UART4"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart4(&mut self) -> EnblReadGroup2ofUart4W<PricIo370Spec> {
        EnblReadGroup2ofUart4W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of UART4"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart4(&mut self) -> EnblReadGroup3ofUart4W<PricIo370Spec> {
        EnblReadGroup3ofUart4W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of UART4"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart4(&mut self) -> EnblReadGroup4ofUart4W<PricIo370Spec> {
        EnblReadGroup4ofUart4W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of UART4"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart4(&mut self) -> EnblReadGroup5ofUart4W<PricIo370Spec> {
        EnblReadGroup5ofUart4W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1370PRIC1_370\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1370pric13701308(
        &mut self,
    ) -> EnblRstToleranceOfPric1370pric13701308W<PricIo370Spec> {
        EnblRstToleranceOfPric1370pric13701308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1370PRIC1_370\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1370pric13701408(
        &mut self,
    ) -> EnblWrProtOfPric1370pric13701408W<PricIo370Spec> {
        EnblWrProtOfPric1370pric13701408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of UART5"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart5(&mut self) -> EnblReadGroup0ofUart5W<PricIo370Spec> {
        EnblReadGroup0ofUart5W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of UART5"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart5(&mut self) -> EnblReadGroup1ofUart5W<PricIo370Spec> {
        EnblReadGroup1ofUart5W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of UART5"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart5(&mut self) -> EnblReadGroup2ofUart5W<PricIo370Spec> {
        EnblReadGroup2ofUart5W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of UART5"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart5(&mut self) -> EnblReadGroup3ofUart5W<PricIo370Spec> {
        EnblReadGroup3ofUart5W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of UART5"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart5(&mut self) -> EnblReadGroup4ofUart5W<PricIo370Spec> {
        EnblReadGroup4ofUart5W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of UART5"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart5(&mut self) -> EnblReadGroup5ofUart5W<PricIo370Spec> {
        EnblReadGroup5ofUart5W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1370PRIC1_370\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1370pric13702116(
        &mut self,
    ) -> EnblRstToleranceOfPric1370pric13702116W<PricIo370Spec> {
        EnblRstToleranceOfPric1370pric13702116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1370PRIC1_370\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1370pric13702216(
        &mut self,
    ) -> EnblWrProtOfPric1370pric13702216W<PricIo370Spec> {
        EnblWrProtOfPric1370pric13702216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of UART6"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart6(&mut self) -> EnblReadGroup0ofUart6W<PricIo370Spec> {
        EnblReadGroup0ofUart6W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of UART6"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart6(&mut self) -> EnblReadGroup1ofUart6W<PricIo370Spec> {
        EnblReadGroup1ofUart6W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of UART6"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart6(&mut self) -> EnblReadGroup2ofUart6W<PricIo370Spec> {
        EnblReadGroup2ofUart6W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of UART6"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart6(&mut self) -> EnblReadGroup3ofUart6W<PricIo370Spec> {
        EnblReadGroup3ofUart6W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of UART6"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart6(&mut self) -> EnblReadGroup4ofUart6W<PricIo370Spec> {
        EnblReadGroup4ofUart6W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of UART6"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart6(&mut self) -> EnblReadGroup5ofUart6W<PricIo370Spec> {
        EnblReadGroup5ofUart6W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1370PRIC1_370\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1370pric13702924(
        &mut self,
    ) -> EnblRstToleranceOfPric1370pric13702924W<PricIo370Spec> {
        EnblRstToleranceOfPric1370pric13702924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1370PRIC1_370\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1370pric13703024(
        &mut self,
    ) -> EnblWrProtOfPric1370pric13703024W<PricIo370Spec> {
        EnblWrProtOfPric1370pric13703024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io370::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io370::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo370Spec;
impl crate::RegisterSpec for PricIo370Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io370::R`](R) reader structure"]
impl crate::Readable for PricIo370Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io370::W`](W) writer structure"]
impl crate::Writable for PricIo370Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO370 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo370Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
