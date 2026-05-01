#[doc = "Register `PRIC_IO36C` reader"]
pub type R = crate::R<PricIo36cSpec>;
#[doc = "Register `PRIC_IO36C` writer"]
pub type W = crate::W<PricIo36cSpec>;
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
#[doc = "Field `EnblReadGroup0OfUART0` reader - Enable Read Group #0 of UART0"]
pub type EnblReadGroup0ofUart0R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUART0` writer - Enable Read Group #0 of UART0"]
pub type EnblReadGroup0ofUart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUART0` reader - Enable Read Group #1 of UART0"]
pub type EnblReadGroup1ofUart0R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUART0` writer - Enable Read Group #1 of UART0"]
pub type EnblReadGroup1ofUart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUART0` reader - Enable Read Group #2 of UART0"]
pub type EnblReadGroup2ofUart0R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUART0` writer - Enable Read Group #2 of UART0"]
pub type EnblReadGroup2ofUart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUART0` reader - Enable Read Group #3 of UART0"]
pub type EnblReadGroup3ofUart0R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUART0` writer - Enable Read Group #3 of UART0"]
pub type EnblReadGroup3ofUart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUART0` reader - Enable Read Group #4 of UART0"]
pub type EnblReadGroup4ofUart0R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUART0` writer - Enable Read Group #4 of UART0"]
pub type EnblReadGroup4ofUart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUART0` reader - Enable Read Group #5 of UART0"]
pub type EnblReadGroup5ofUart0R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUART0` writer - Enable Read Group #5 of UART0"]
pub type EnblReadGroup5ofUart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC136CPRIC1_36C\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric136cpric136c1308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric136cpric136c1308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric136cpric136c1308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC136CPRIC136C1308` reader - Enable Reset Tolerance of PRIC136CPRIC1_36C\\[13:08\\]"]
pub type EnblRstToleranceOfPric136cpric136c1308R =
    crate::BitReader<EnblRstToleranceOfPric136cpric136c1308>;
impl EnblRstToleranceOfPric136cpric136c1308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric136cpric136c1308 {
        match self.bits {
            false => EnblRstToleranceOfPric136cpric136c1308::ResetBySrst,
            true => EnblRstToleranceOfPric136cpric136c1308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric136cpric136c1308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric136cpric136c1308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC136CPRIC136C1308` writer - Enable Reset Tolerance of PRIC136CPRIC1_36C\\[13:08\\]"]
pub type EnblRstToleranceOfPric136cpric136c1308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric136cpric136c1308>;
impl<'a, REG> EnblRstToleranceOfPric136cpric136c1308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric136cpric136c1308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric136cpric136c1308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC136CPRIC136C1408` reader - Enable Write Protection of PRIC136CPRIC1_36C\\[14:08\\]"]
pub type EnblWrProtOfPric136cpric136c1408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC136CPRIC136C1408` writer - Enable Write Protection of PRIC136CPRIC1_36C\\[14:08\\]"]
pub type EnblWrProtOfPric136cpric136c1408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfUART1` reader - Enable Read Group #0 of UART1"]
pub type EnblReadGroup0ofUart1R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUART1` writer - Enable Read Group #0 of UART1"]
pub type EnblReadGroup0ofUart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUART1` reader - Enable Read Group #1 of UART1"]
pub type EnblReadGroup1ofUart1R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUART1` writer - Enable Read Group #1 of UART1"]
pub type EnblReadGroup1ofUart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUART1` reader - Enable Read Group #2 of UART1"]
pub type EnblReadGroup2ofUart1R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUART1` writer - Enable Read Group #2 of UART1"]
pub type EnblReadGroup2ofUart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUART1` reader - Enable Read Group #3 of UART1"]
pub type EnblReadGroup3ofUart1R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUART1` writer - Enable Read Group #3 of UART1"]
pub type EnblReadGroup3ofUart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUART1` reader - Enable Read Group #4 of UART1"]
pub type EnblReadGroup4ofUart1R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUART1` writer - Enable Read Group #4 of UART1"]
pub type EnblReadGroup4ofUart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUART1` reader - Enable Read Group #5 of UART1"]
pub type EnblReadGroup5ofUart1R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUART1` writer - Enable Read Group #5 of UART1"]
pub type EnblReadGroup5ofUart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC136CPRIC1_36C\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric136cpric136c2116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric136cpric136c2116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric136cpric136c2116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC136CPRIC136C2116` reader - Enable Reset Tolerance of PRIC136CPRIC1_36C\\[21:16\\]"]
pub type EnblRstToleranceOfPric136cpric136c2116R =
    crate::BitReader<EnblRstToleranceOfPric136cpric136c2116>;
impl EnblRstToleranceOfPric136cpric136c2116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric136cpric136c2116 {
        match self.bits {
            false => EnblRstToleranceOfPric136cpric136c2116::ResetBySrst,
            true => EnblRstToleranceOfPric136cpric136c2116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric136cpric136c2116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric136cpric136c2116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC136CPRIC136C2116` writer - Enable Reset Tolerance of PRIC136CPRIC1_36C\\[21:16\\]"]
pub type EnblRstToleranceOfPric136cpric136c2116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric136cpric136c2116>;
impl<'a, REG> EnblRstToleranceOfPric136cpric136c2116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric136cpric136c2116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric136cpric136c2116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC136CPRIC136C2216` reader - Enable Write Protection of PRIC136CPRIC1_36C\\[22:16\\]"]
pub type EnblWrProtOfPric136cpric136c2216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC136CPRIC136C2216` writer - Enable Write Protection of PRIC136CPRIC1_36C\\[22:16\\]"]
pub type EnblWrProtOfPric136cpric136c2216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfUART2` reader - Enable Read Group #0 of UART2"]
pub type EnblReadGroup0ofUart2R = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUART2` writer - Enable Read Group #0 of UART2"]
pub type EnblReadGroup0ofUart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUART2` reader - Enable Read Group #1 of UART2"]
pub type EnblReadGroup1ofUart2R = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUART2` writer - Enable Read Group #1 of UART2"]
pub type EnblReadGroup1ofUart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUART2` reader - Enable Read Group #2 of UART2"]
pub type EnblReadGroup2ofUart2R = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUART2` writer - Enable Read Group #2 of UART2"]
pub type EnblReadGroup2ofUart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUART2` reader - Enable Read Group #3 of UART2"]
pub type EnblReadGroup3ofUart2R = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUART2` writer - Enable Read Group #3 of UART2"]
pub type EnblReadGroup3ofUart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUART2` reader - Enable Read Group #4 of UART2"]
pub type EnblReadGroup4ofUart2R = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUART2` writer - Enable Read Group #4 of UART2"]
pub type EnblReadGroup4ofUart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUART2` reader - Enable Read Group #5 of UART2"]
pub type EnblReadGroup5ofUart2R = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUART2` writer - Enable Read Group #5 of UART2"]
pub type EnblReadGroup5ofUart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC136CPRIC1_36C\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric136cpric136c2924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric136cpric136c2924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric136cpric136c2924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC136CPRIC136C2924` reader - Enable Reset Tolerance of PRIC136CPRIC1_36C\\[29:24\\]"]
pub type EnblRstToleranceOfPric136cpric136c2924R =
    crate::BitReader<EnblRstToleranceOfPric136cpric136c2924>;
impl EnblRstToleranceOfPric136cpric136c2924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric136cpric136c2924 {
        match self.bits {
            false => EnblRstToleranceOfPric136cpric136c2924::ResetBySrst,
            true => EnblRstToleranceOfPric136cpric136c2924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric136cpric136c2924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric136cpric136c2924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC136CPRIC136C2924` writer - Enable Reset Tolerance of PRIC136CPRIC1_36C\\[29:24\\]"]
pub type EnblRstToleranceOfPric136cpric136c2924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric136cpric136c2924>;
impl<'a, REG> EnblRstToleranceOfPric136cpric136c2924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric136cpric136c2924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric136cpric136c2924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC136CPRIC136C3024` reader - Enable Write Protection of PRIC136CPRIC1_36C\\[30:24\\]"]
pub type EnblWrProtOfPric136cpric136c3024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC136CPRIC136C3024` writer - Enable Write Protection of PRIC136CPRIC1_36C\\[30:24\\]"]
pub type EnblWrProtOfPric136cpric136c3024W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 8 - Enable Read Group #0 of UART0"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart0(&self) -> EnblReadGroup0ofUart0R {
        EnblReadGroup0ofUart0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of UART0"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart0(&self) -> EnblReadGroup1ofUart0R {
        EnblReadGroup1ofUart0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of UART0"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart0(&self) -> EnblReadGroup2ofUart0R {
        EnblReadGroup2ofUart0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of UART0"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart0(&self) -> EnblReadGroup3ofUart0R {
        EnblReadGroup3ofUart0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of UART0"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart0(&self) -> EnblReadGroup4ofUart0R {
        EnblReadGroup4ofUart0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of UART0"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart0(&self) -> EnblReadGroup5ofUart0R {
        EnblReadGroup5ofUart0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC136CPRIC1_36C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric136cpric136c1308(
        &self,
    ) -> EnblRstToleranceOfPric136cpric136c1308R {
        EnblRstToleranceOfPric136cpric136c1308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC136CPRIC1_36C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric136cpric136c1408(&self) -> EnblWrProtOfPric136cpric136c1408R {
        EnblWrProtOfPric136cpric136c1408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of UART1"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart1(&self) -> EnblReadGroup0ofUart1R {
        EnblReadGroup0ofUart1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of UART1"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart1(&self) -> EnblReadGroup1ofUart1R {
        EnblReadGroup1ofUart1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of UART1"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart1(&self) -> EnblReadGroup2ofUart1R {
        EnblReadGroup2ofUart1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of UART1"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart1(&self) -> EnblReadGroup3ofUart1R {
        EnblReadGroup3ofUart1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of UART1"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart1(&self) -> EnblReadGroup4ofUart1R {
        EnblReadGroup4ofUart1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of UART1"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart1(&self) -> EnblReadGroup5ofUart1R {
        EnblReadGroup5ofUart1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC136CPRIC1_36C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric136cpric136c2116(
        &self,
    ) -> EnblRstToleranceOfPric136cpric136c2116R {
        EnblRstToleranceOfPric136cpric136c2116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC136CPRIC1_36C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric136cpric136c2216(&self) -> EnblWrProtOfPric136cpric136c2216R {
        EnblWrProtOfPric136cpric136c2216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of UART2"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart2(&self) -> EnblReadGroup0ofUart2R {
        EnblReadGroup0ofUart2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of UART2"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart2(&self) -> EnblReadGroup1ofUart2R {
        EnblReadGroup1ofUart2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of UART2"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart2(&self) -> EnblReadGroup2ofUart2R {
        EnblReadGroup2ofUart2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of UART2"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart2(&self) -> EnblReadGroup3ofUart2R {
        EnblReadGroup3ofUart2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of UART2"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart2(&self) -> EnblReadGroup4ofUart2R {
        EnblReadGroup4ofUart2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of UART2"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart2(&self) -> EnblReadGroup5ofUart2R {
        EnblReadGroup5ofUart2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC136CPRIC1_36C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric136cpric136c2924(
        &self,
    ) -> EnblRstToleranceOfPric136cpric136c2924R {
        EnblRstToleranceOfPric136cpric136c2924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC136CPRIC1_36C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric136cpric136c3024(&self) -> EnblWrProtOfPric136cpric136c3024R {
        EnblWrProtOfPric136cpric136c3024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo36cSpec> {
        Reserved7W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo36cSpec> {
        Reserved6W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo36cSpec> {
        Reserved5W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo36cSpec> {
        Reserved4W::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo36cSpec> {
        Reserved3W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo36cSpec> {
        Reserved2W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo36cSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of UART0"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart0(&mut self) -> EnblReadGroup0ofUart0W<PricIo36cSpec> {
        EnblReadGroup0ofUart0W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of UART0"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart0(&mut self) -> EnblReadGroup1ofUart0W<PricIo36cSpec> {
        EnblReadGroup1ofUart0W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of UART0"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart0(&mut self) -> EnblReadGroup2ofUart0W<PricIo36cSpec> {
        EnblReadGroup2ofUart0W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of UART0"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart0(&mut self) -> EnblReadGroup3ofUart0W<PricIo36cSpec> {
        EnblReadGroup3ofUart0W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of UART0"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart0(&mut self) -> EnblReadGroup4ofUart0W<PricIo36cSpec> {
        EnblReadGroup4ofUart0W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of UART0"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart0(&mut self) -> EnblReadGroup5ofUart0W<PricIo36cSpec> {
        EnblReadGroup5ofUart0W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC136CPRIC1_36C\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric136cpric136c1308(
        &mut self,
    ) -> EnblRstToleranceOfPric136cpric136c1308W<PricIo36cSpec> {
        EnblRstToleranceOfPric136cpric136c1308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC136CPRIC1_36C\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric136cpric136c1408(
        &mut self,
    ) -> EnblWrProtOfPric136cpric136c1408W<PricIo36cSpec> {
        EnblWrProtOfPric136cpric136c1408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of UART1"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart1(&mut self) -> EnblReadGroup0ofUart1W<PricIo36cSpec> {
        EnblReadGroup0ofUart1W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of UART1"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart1(&mut self) -> EnblReadGroup1ofUart1W<PricIo36cSpec> {
        EnblReadGroup1ofUart1W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of UART1"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart1(&mut self) -> EnblReadGroup2ofUart1W<PricIo36cSpec> {
        EnblReadGroup2ofUart1W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of UART1"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart1(&mut self) -> EnblReadGroup3ofUart1W<PricIo36cSpec> {
        EnblReadGroup3ofUart1W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of UART1"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart1(&mut self) -> EnblReadGroup4ofUart1W<PricIo36cSpec> {
        EnblReadGroup4ofUart1W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of UART1"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart1(&mut self) -> EnblReadGroup5ofUart1W<PricIo36cSpec> {
        EnblReadGroup5ofUart1W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC136CPRIC1_36C\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric136cpric136c2116(
        &mut self,
    ) -> EnblRstToleranceOfPric136cpric136c2116W<PricIo36cSpec> {
        EnblRstToleranceOfPric136cpric136c2116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC136CPRIC1_36C\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric136cpric136c2216(
        &mut self,
    ) -> EnblWrProtOfPric136cpric136c2216W<PricIo36cSpec> {
        EnblWrProtOfPric136cpric136c2216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of UART2"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart2(&mut self) -> EnblReadGroup0ofUart2W<PricIo36cSpec> {
        EnblReadGroup0ofUart2W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of UART2"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart2(&mut self) -> EnblReadGroup1ofUart2W<PricIo36cSpec> {
        EnblReadGroup1ofUart2W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of UART2"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart2(&mut self) -> EnblReadGroup2ofUart2W<PricIo36cSpec> {
        EnblReadGroup2ofUart2W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of UART2"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart2(&mut self) -> EnblReadGroup3ofUart2W<PricIo36cSpec> {
        EnblReadGroup3ofUart2W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of UART2"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart2(&mut self) -> EnblReadGroup4ofUart2W<PricIo36cSpec> {
        EnblReadGroup4ofUart2W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of UART2"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart2(&mut self) -> EnblReadGroup5ofUart2W<PricIo36cSpec> {
        EnblReadGroup5ofUart2W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC136CPRIC1_36C\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric136cpric136c2924(
        &mut self,
    ) -> EnblRstToleranceOfPric136cpric136c2924W<PricIo36cSpec> {
        EnblRstToleranceOfPric136cpric136c2924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC136CPRIC1_36C\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric136cpric136c3024(
        &mut self,
    ) -> EnblWrProtOfPric136cpric136c3024W<PricIo36cSpec> {
        EnblWrProtOfPric136cpric136c3024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#27\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io36c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io36c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo36cSpec;
impl crate::RegisterSpec for PricIo36cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io36c::R`](R) reader structure"]
impl crate::Readable for PricIo36cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io36c::W`](W) writer structure"]
impl crate::Writable for PricIo36cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO36C to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo36cSpec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
