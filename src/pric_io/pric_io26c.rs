#[doc = "Register `PRIC_IO26C` reader"]
pub type R = crate::R<PricIo26cSpec>;
#[doc = "Register `PRIC_IO26C` writer"]
pub type W = crate::W<PricIo26cSpec>;
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
#[doc = "Field `EnblWrGroup0OfUART0` reader - Enable Write Group #0 of UART0"]
pub type EnblWrGroup0ofUart0R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUART0` writer - Enable Write Group #0 of UART0"]
pub type EnblWrGroup0ofUart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUART0` reader - Enable Write Group #1 of UART0"]
pub type EnblWrGroup1ofUart0R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUART0` writer - Enable Write Group #1 of UART0"]
pub type EnblWrGroup1ofUart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUART0` reader - Enable Write Group #2 of UART0"]
pub type EnblWrGroup2ofUart0R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUART0` writer - Enable Write Group #2 of UART0"]
pub type EnblWrGroup2ofUart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUART0` reader - Enable Write Group #3 of UART0"]
pub type EnblWrGroup3ofUart0R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUART0` writer - Enable Write Group #3 of UART0"]
pub type EnblWrGroup3ofUart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUART0` reader - Enable Write Group #4 of UART0"]
pub type EnblWrGroup4ofUart0R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUART0` writer - Enable Write Group #4 of UART0"]
pub type EnblWrGroup4ofUart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUART0` reader - Enable Write Group #5 of UART0"]
pub type EnblWrGroup5ofUart0R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUART0` writer - Enable Write Group #5 of UART0"]
pub type EnblWrGroup5ofUart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC126CPRIC1_26C\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric126cpric126c1308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric126cpric126c1308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric126cpric126c1308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC126CPRIC126C1308` reader - Enable Reset Tolerance of PRIC126CPRIC1_26C\\[13:08\\]"]
pub type EnblRstToleranceOfPric126cpric126c1308R =
    crate::BitReader<EnblRstToleranceOfPric126cpric126c1308>;
impl EnblRstToleranceOfPric126cpric126c1308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric126cpric126c1308 {
        match self.bits {
            false => EnblRstToleranceOfPric126cpric126c1308::ResetBySrst,
            true => EnblRstToleranceOfPric126cpric126c1308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric126cpric126c1308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric126cpric126c1308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC126CPRIC126C1308` writer - Enable Reset Tolerance of PRIC126CPRIC1_26C\\[13:08\\]"]
pub type EnblRstToleranceOfPric126cpric126c1308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric126cpric126c1308>;
impl<'a, REG> EnblRstToleranceOfPric126cpric126c1308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric126cpric126c1308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric126cpric126c1308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC126CPRIC126C1408` reader - Enable Write Protection of PRIC126CPRIC1_26C\\[14:08\\]"]
pub type EnblWrProtOfPric126cpric126c1408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC126CPRIC126C1408` writer - Enable Write Protection of PRIC126CPRIC1_26C\\[14:08\\]"]
pub type EnblWrProtOfPric126cpric126c1408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfUART1` reader - Enable Write Group #0 of UART1"]
pub type EnblWrGroup0ofUart1R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUART1` writer - Enable Write Group #0 of UART1"]
pub type EnblWrGroup0ofUart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUART1` reader - Enable Write Group #1 of UART1"]
pub type EnblWrGroup1ofUart1R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUART1` writer - Enable Write Group #1 of UART1"]
pub type EnblWrGroup1ofUart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUART1` reader - Enable Write Group #2 of UART1"]
pub type EnblWrGroup2ofUart1R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUART1` writer - Enable Write Group #2 of UART1"]
pub type EnblWrGroup2ofUart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUART1` reader - Enable Write Group #3 of UART1"]
pub type EnblWrGroup3ofUart1R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUART1` writer - Enable Write Group #3 of UART1"]
pub type EnblWrGroup3ofUart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUART1` reader - Enable Write Group #4 of UART1"]
pub type EnblWrGroup4ofUart1R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUART1` writer - Enable Write Group #4 of UART1"]
pub type EnblWrGroup4ofUart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUART1` reader - Enable Write Group #5 of UART1"]
pub type EnblWrGroup5ofUart1R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUART1` writer - Enable Write Group #5 of UART1"]
pub type EnblWrGroup5ofUart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC126CPRIC1_26C\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric126cpric126c2116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric126cpric126c2116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric126cpric126c2116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC126CPRIC126C2116` reader - Enable Reset Tolerance of PRIC126CPRIC1_26C\\[21:16\\]"]
pub type EnblRstToleranceOfPric126cpric126c2116R =
    crate::BitReader<EnblRstToleranceOfPric126cpric126c2116>;
impl EnblRstToleranceOfPric126cpric126c2116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric126cpric126c2116 {
        match self.bits {
            false => EnblRstToleranceOfPric126cpric126c2116::ResetBySrst,
            true => EnblRstToleranceOfPric126cpric126c2116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric126cpric126c2116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric126cpric126c2116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC126CPRIC126C2116` writer - Enable Reset Tolerance of PRIC126CPRIC1_26C\\[21:16\\]"]
pub type EnblRstToleranceOfPric126cpric126c2116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric126cpric126c2116>;
impl<'a, REG> EnblRstToleranceOfPric126cpric126c2116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric126cpric126c2116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric126cpric126c2116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC126CPRIC126C2216` reader - Enable Write Protection of PRIC126CPRIC1_26C\\[22:16\\]"]
pub type EnblWrProtOfPric126cpric126c2216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC126CPRIC126C2216` writer - Enable Write Protection of PRIC126CPRIC1_26C\\[22:16\\]"]
pub type EnblWrProtOfPric126cpric126c2216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfUART2` reader - Enable Write Group #0 of UART2"]
pub type EnblWrGroup0ofUart2R = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUART2` writer - Enable Write Group #0 of UART2"]
pub type EnblWrGroup0ofUart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUART2` reader - Enable Write Group #1 of UART2"]
pub type EnblWrGroup1ofUart2R = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUART2` writer - Enable Write Group #1 of UART2"]
pub type EnblWrGroup1ofUart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUART2` reader - Enable Write Group #2 of UART2"]
pub type EnblWrGroup2ofUart2R = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUART2` writer - Enable Write Group #2 of UART2"]
pub type EnblWrGroup2ofUart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUART2` reader - Enable Write Group #3 of UART2"]
pub type EnblWrGroup3ofUart2R = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUART2` writer - Enable Write Group #3 of UART2"]
pub type EnblWrGroup3ofUart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUART2` reader - Enable Write Group #4 of UART2"]
pub type EnblWrGroup4ofUart2R = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUART2` writer - Enable Write Group #4 of UART2"]
pub type EnblWrGroup4ofUart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUART2` reader - Enable Write Group #5 of UART2"]
pub type EnblWrGroup5ofUart2R = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUART2` writer - Enable Write Group #5 of UART2"]
pub type EnblWrGroup5ofUart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC126CPRIC1_26C\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric126cpric126c2924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric126cpric126c2924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric126cpric126c2924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC126CPRIC126C2924` reader - Enable Reset Tolerance of PRIC126CPRIC1_26C\\[29:24\\]"]
pub type EnblRstToleranceOfPric126cpric126c2924R =
    crate::BitReader<EnblRstToleranceOfPric126cpric126c2924>;
impl EnblRstToleranceOfPric126cpric126c2924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric126cpric126c2924 {
        match self.bits {
            false => EnblRstToleranceOfPric126cpric126c2924::ResetBySrst,
            true => EnblRstToleranceOfPric126cpric126c2924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric126cpric126c2924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric126cpric126c2924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC126CPRIC126C2924` writer - Enable Reset Tolerance of PRIC126CPRIC1_26C\\[29:24\\]"]
pub type EnblRstToleranceOfPric126cpric126c2924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric126cpric126c2924>;
impl<'a, REG> EnblRstToleranceOfPric126cpric126c2924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric126cpric126c2924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric126cpric126c2924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC126CPRIC126C3024` reader - Enable Write Protection of PRIC126CPRIC1_26C\\[30:24\\]"]
pub type EnblWrProtOfPric126cpric126c3024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC126CPRIC126C3024` writer - Enable Write Protection of PRIC126CPRIC1_26C\\[30:24\\]"]
pub type EnblWrProtOfPric126cpric126c3024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of UART0"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart0(&self) -> EnblWrGroup0ofUart0R {
        EnblWrGroup0ofUart0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of UART0"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart0(&self) -> EnblWrGroup1ofUart0R {
        EnblWrGroup1ofUart0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of UART0"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart0(&self) -> EnblWrGroup2ofUart0R {
        EnblWrGroup2ofUart0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of UART0"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart0(&self) -> EnblWrGroup3ofUart0R {
        EnblWrGroup3ofUart0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of UART0"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart0(&self) -> EnblWrGroup4ofUart0R {
        EnblWrGroup4ofUart0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of UART0"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart0(&self) -> EnblWrGroup5ofUart0R {
        EnblWrGroup5ofUart0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC126CPRIC1_26C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric126cpric126c1308(
        &self,
    ) -> EnblRstToleranceOfPric126cpric126c1308R {
        EnblRstToleranceOfPric126cpric126c1308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC126CPRIC1_26C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric126cpric126c1408(&self) -> EnblWrProtOfPric126cpric126c1408R {
        EnblWrProtOfPric126cpric126c1408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of UART1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart1(&self) -> EnblWrGroup0ofUart1R {
        EnblWrGroup0ofUart1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of UART1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart1(&self) -> EnblWrGroup1ofUart1R {
        EnblWrGroup1ofUart1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of UART1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart1(&self) -> EnblWrGroup2ofUart1R {
        EnblWrGroup2ofUart1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of UART1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart1(&self) -> EnblWrGroup3ofUart1R {
        EnblWrGroup3ofUart1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of UART1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart1(&self) -> EnblWrGroup4ofUart1R {
        EnblWrGroup4ofUart1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of UART1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart1(&self) -> EnblWrGroup5ofUart1R {
        EnblWrGroup5ofUart1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC126CPRIC1_26C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric126cpric126c2116(
        &self,
    ) -> EnblRstToleranceOfPric126cpric126c2116R {
        EnblRstToleranceOfPric126cpric126c2116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC126CPRIC1_26C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric126cpric126c2216(&self) -> EnblWrProtOfPric126cpric126c2216R {
        EnblWrProtOfPric126cpric126c2216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of UART2"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart2(&self) -> EnblWrGroup0ofUart2R {
        EnblWrGroup0ofUart2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of UART2"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart2(&self) -> EnblWrGroup1ofUart2R {
        EnblWrGroup1ofUart2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of UART2"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart2(&self) -> EnblWrGroup2ofUart2R {
        EnblWrGroup2ofUart2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of UART2"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart2(&self) -> EnblWrGroup3ofUart2R {
        EnblWrGroup3ofUart2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of UART2"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart2(&self) -> EnblWrGroup4ofUart2R {
        EnblWrGroup4ofUart2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of UART2"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart2(&self) -> EnblWrGroup5ofUart2R {
        EnblWrGroup5ofUart2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC126CPRIC1_26C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric126cpric126c2924(
        &self,
    ) -> EnblRstToleranceOfPric126cpric126c2924R {
        EnblRstToleranceOfPric126cpric126c2924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC126CPRIC1_26C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric126cpric126c3024(&self) -> EnblWrProtOfPric126cpric126c3024R {
        EnblWrProtOfPric126cpric126c3024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo26cSpec> {
        Reserved7W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo26cSpec> {
        Reserved6W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo26cSpec> {
        Reserved5W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo26cSpec> {
        Reserved4W::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo26cSpec> {
        Reserved3W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo26cSpec> {
        Reserved2W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo26cSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of UART0"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart0(&mut self) -> EnblWrGroup0ofUart0W<PricIo26cSpec> {
        EnblWrGroup0ofUart0W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of UART0"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart0(&mut self) -> EnblWrGroup1ofUart0W<PricIo26cSpec> {
        EnblWrGroup1ofUart0W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of UART0"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart0(&mut self) -> EnblWrGroup2ofUart0W<PricIo26cSpec> {
        EnblWrGroup2ofUart0W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of UART0"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart0(&mut self) -> EnblWrGroup3ofUart0W<PricIo26cSpec> {
        EnblWrGroup3ofUart0W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of UART0"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart0(&mut self) -> EnblWrGroup4ofUart0W<PricIo26cSpec> {
        EnblWrGroup4ofUart0W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of UART0"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart0(&mut self) -> EnblWrGroup5ofUart0W<PricIo26cSpec> {
        EnblWrGroup5ofUart0W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC126CPRIC1_26C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric126cpric126c1308(
        &mut self,
    ) -> EnblRstToleranceOfPric126cpric126c1308W<PricIo26cSpec> {
        EnblRstToleranceOfPric126cpric126c1308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC126CPRIC1_26C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric126cpric126c1408(
        &mut self,
    ) -> EnblWrProtOfPric126cpric126c1408W<PricIo26cSpec> {
        EnblWrProtOfPric126cpric126c1408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of UART1"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart1(&mut self) -> EnblWrGroup0ofUart1W<PricIo26cSpec> {
        EnblWrGroup0ofUart1W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of UART1"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart1(&mut self) -> EnblWrGroup1ofUart1W<PricIo26cSpec> {
        EnblWrGroup1ofUart1W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of UART1"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart1(&mut self) -> EnblWrGroup2ofUart1W<PricIo26cSpec> {
        EnblWrGroup2ofUart1W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of UART1"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart1(&mut self) -> EnblWrGroup3ofUart1W<PricIo26cSpec> {
        EnblWrGroup3ofUart1W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of UART1"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart1(&mut self) -> EnblWrGroup4ofUart1W<PricIo26cSpec> {
        EnblWrGroup4ofUart1W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of UART1"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart1(&mut self) -> EnblWrGroup5ofUart1W<PricIo26cSpec> {
        EnblWrGroup5ofUart1W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC126CPRIC1_26C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric126cpric126c2116(
        &mut self,
    ) -> EnblRstToleranceOfPric126cpric126c2116W<PricIo26cSpec> {
        EnblRstToleranceOfPric126cpric126c2116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC126CPRIC1_26C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric126cpric126c2216(
        &mut self,
    ) -> EnblWrProtOfPric126cpric126c2216W<PricIo26cSpec> {
        EnblWrProtOfPric126cpric126c2216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of UART2"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart2(&mut self) -> EnblWrGroup0ofUart2W<PricIo26cSpec> {
        EnblWrGroup0ofUart2W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of UART2"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart2(&mut self) -> EnblWrGroup1ofUart2W<PricIo26cSpec> {
        EnblWrGroup1ofUart2W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of UART2"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart2(&mut self) -> EnblWrGroup2ofUart2W<PricIo26cSpec> {
        EnblWrGroup2ofUart2W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of UART2"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart2(&mut self) -> EnblWrGroup3ofUart2W<PricIo26cSpec> {
        EnblWrGroup3ofUart2W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of UART2"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart2(&mut self) -> EnblWrGroup4ofUart2W<PricIo26cSpec> {
        EnblWrGroup4ofUart2W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of UART2"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart2(&mut self) -> EnblWrGroup5ofUart2W<PricIo26cSpec> {
        EnblWrGroup5ofUart2W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC126CPRIC1_26C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric126cpric126c2924(
        &mut self,
    ) -> EnblRstToleranceOfPric126cpric126c2924W<PricIo26cSpec> {
        EnblRstToleranceOfPric126cpric126c2924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC126CPRIC1_26C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric126cpric126c3024(
        &mut self,
    ) -> EnblWrProtOfPric126cpric126c3024W<PricIo26cSpec> {
        EnblWrProtOfPric126cpric126c3024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#27\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io26c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io26c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo26cSpec;
impl crate::RegisterSpec for PricIo26cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io26c::R`](R) reader structure"]
impl crate::Readable for PricIo26cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io26c::W`](W) writer structure"]
impl crate::Writable for PricIo26cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO26C to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo26cSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
