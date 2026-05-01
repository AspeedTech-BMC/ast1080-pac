#[doc = "Register `PRIC_IO248` reader"]
pub type R = crate::R<PricIo248Spec>;
#[doc = "Register `PRIC_IO248` writer"]
pub type W = crate::W<PricIo248Spec>;
#[doc = "Field `EnblWrGroup0OfUartDMA` reader - Enable Write Group #0 of UartDMA"]
pub type EnblWrGroup0ofUartDmaR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUartDMA` writer - Enable Write Group #0 of UartDMA"]
pub type EnblWrGroup0ofUartDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUartDMA` reader - Enable Write Group #1 of UartDMA"]
pub type EnblWrGroup1ofUartDmaR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUartDMA` writer - Enable Write Group #1 of UartDMA"]
pub type EnblWrGroup1ofUartDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUartDMA` reader - Enable Write Group #2 of UartDMA"]
pub type EnblWrGroup2ofUartDmaR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUartDMA` writer - Enable Write Group #2 of UartDMA"]
pub type EnblWrGroup2ofUartDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUartDMA` reader - Enable Write Group #3 of UartDMA"]
pub type EnblWrGroup3ofUartDmaR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUartDMA` writer - Enable Write Group #3 of UartDMA"]
pub type EnblWrGroup3ofUartDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUartDMA` reader - Enable Write Group #4 of UartDMA"]
pub type EnblWrGroup4ofUartDmaR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUartDMA` writer - Enable Write Group #4 of UartDMA"]
pub type EnblWrGroup4ofUartDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUartDMA` reader - Enable Write Group #5 of UartDMA"]
pub type EnblWrGroup5ofUartDmaR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUartDMA` writer - Enable Write Group #5 of UartDMA"]
pub type EnblWrGroup5ofUartDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1248PRIC1_248\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1248pric12480500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1248pric12480500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1248pric12480500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1248PRIC12480500` reader - Enable Reset Tolerance of PRIC1248PRIC1_248\\[05:00\\]"]
pub type EnblRstToleranceOfPric1248pric12480500R =
    crate::BitReader<EnblRstToleranceOfPric1248pric12480500>;
impl EnblRstToleranceOfPric1248pric12480500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1248pric12480500 {
        match self.bits {
            false => EnblRstToleranceOfPric1248pric12480500::ResetBySrst,
            true => EnblRstToleranceOfPric1248pric12480500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1248pric12480500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1248pric12480500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1248PRIC12480500` writer - Enable Reset Tolerance of PRIC1248PRIC1_248\\[05:00\\]"]
pub type EnblRstToleranceOfPric1248pric12480500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1248pric12480500>;
impl<'a, REG> EnblRstToleranceOfPric1248pric12480500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1248pric12480500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1248pric12480500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1248PRIC12480600` reader - Enable Write Protection of PRIC1248PRIC1_248\\[06:00\\]"]
pub type EnblWrProtOfPric1248pric12480600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1248PRIC12480600` writer - Enable Write Protection of PRIC1248PRIC1_248\\[06:00\\]"]
pub type EnblWrProtOfPric1248pric12480600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfUartDBG` reader - Enable Write Group #0 of UartDBG"]
pub type EnblWrGroup0ofUartDbgR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUartDBG` writer - Enable Write Group #0 of UartDBG"]
pub type EnblWrGroup0ofUartDbgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUartDBG` reader - Enable Write Group #1 of UartDBG"]
pub type EnblWrGroup1ofUartDbgR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUartDBG` writer - Enable Write Group #1 of UartDBG"]
pub type EnblWrGroup1ofUartDbgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUartDBG` reader - Enable Write Group #2 of UartDBG"]
pub type EnblWrGroup2ofUartDbgR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUartDBG` writer - Enable Write Group #2 of UartDBG"]
pub type EnblWrGroup2ofUartDbgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUartDBG` reader - Enable Write Group #3 of UartDBG"]
pub type EnblWrGroup3ofUartDbgR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUartDBG` writer - Enable Write Group #3 of UartDBG"]
pub type EnblWrGroup3ofUartDbgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUartDBG` reader - Enable Write Group #4 of UartDBG"]
pub type EnblWrGroup4ofUartDbgR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUartDBG` writer - Enable Write Group #4 of UartDBG"]
pub type EnblWrGroup4ofUartDbgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUartDBG` reader - Enable Write Group #5 of UartDBG"]
pub type EnblWrGroup5ofUartDbgR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUartDBG` writer - Enable Write Group #5 of UartDBG"]
pub type EnblWrGroup5ofUartDbgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1248PRIC1_248\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1248pric12481308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1248pric12481308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1248pric12481308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1248PRIC12481308` reader - Enable Reset Tolerance of PRIC1248PRIC1_248\\[13:08\\]"]
pub type EnblRstToleranceOfPric1248pric12481308R =
    crate::BitReader<EnblRstToleranceOfPric1248pric12481308>;
impl EnblRstToleranceOfPric1248pric12481308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1248pric12481308 {
        match self.bits {
            false => EnblRstToleranceOfPric1248pric12481308::ResetBySrst,
            true => EnblRstToleranceOfPric1248pric12481308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1248pric12481308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1248pric12481308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1248PRIC12481308` writer - Enable Reset Tolerance of PRIC1248PRIC1_248\\[13:08\\]"]
pub type EnblRstToleranceOfPric1248pric12481308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1248pric12481308>;
impl<'a, REG> EnblRstToleranceOfPric1248pric12481308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1248pric12481308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1248pric12481308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1248PRIC12481408` reader - Enable Write Protection of PRIC1248PRIC1_248\\[14:08\\]"]
pub type EnblWrProtOfPric1248pric12481408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1248PRIC12481408` writer - Enable Write Protection of PRIC1248PRIC1_248\\[14:08\\]"]
pub type EnblWrProtOfPric1248pric12481408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved15` reader - Reserved"]
pub type Reserved15R = crate::BitReader;
#[doc = "Field `Reserved15` writer - Reserved"]
pub type Reserved15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved14` reader - Reserved"]
pub type Reserved14R = crate::BitReader;
#[doc = "Field `Reserved14` writer - Reserved"]
pub type Reserved14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved13` reader - Reserved"]
pub type Reserved13R = crate::BitReader;
#[doc = "Field `Reserved13` writer - Reserved"]
pub type Reserved13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved12` reader - Reserved"]
pub type Reserved12R = crate::BitReader;
#[doc = "Field `Reserved12` writer - Reserved"]
pub type Reserved12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved11` reader - Reserved"]
pub type Reserved11R = crate::BitReader;
#[doc = "Field `Reserved11` writer - Reserved"]
pub type Reserved11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved10` reader - Reserved"]
pub type Reserved10R = crate::BitReader;
#[doc = "Field `Reserved10` writer - Reserved"]
pub type Reserved10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved9` reader - Reserved"]
pub type Reserved9R = crate::BitReader;
#[doc = "Field `Reserved9` writer - Reserved"]
pub type Reserved9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved8` reader - Reserved"]
pub type Reserved8R = crate::BitReader;
#[doc = "Field `Reserved8` writer - Reserved"]
pub type Reserved8W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of UartDMA"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart_dma(&self) -> EnblWrGroup0ofUartDmaR {
        EnblWrGroup0ofUartDmaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of UartDMA"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart_dma(&self) -> EnblWrGroup1ofUartDmaR {
        EnblWrGroup1ofUartDmaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of UartDMA"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart_dma(&self) -> EnblWrGroup2ofUartDmaR {
        EnblWrGroup2ofUartDmaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of UartDMA"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart_dma(&self) -> EnblWrGroup3ofUartDmaR {
        EnblWrGroup3ofUartDmaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of UartDMA"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart_dma(&self) -> EnblWrGroup4ofUartDmaR {
        EnblWrGroup4ofUartDmaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of UartDMA"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart_dma(&self) -> EnblWrGroup5ofUartDmaR {
        EnblWrGroup5ofUartDmaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1248PRIC1_248\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1248pric12480500(
        &self,
    ) -> EnblRstToleranceOfPric1248pric12480500R {
        EnblRstToleranceOfPric1248pric12480500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1248PRIC1_248\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1248pric12480600(&self) -> EnblWrProtOfPric1248pric12480600R {
        EnblWrProtOfPric1248pric12480600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of UartDBG"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart_dbg(&self) -> EnblWrGroup0ofUartDbgR {
        EnblWrGroup0ofUartDbgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of UartDBG"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart_dbg(&self) -> EnblWrGroup1ofUartDbgR {
        EnblWrGroup1ofUartDbgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of UartDBG"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart_dbg(&self) -> EnblWrGroup2ofUartDbgR {
        EnblWrGroup2ofUartDbgR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of UartDBG"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart_dbg(&self) -> EnblWrGroup3ofUartDbgR {
        EnblWrGroup3ofUartDbgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of UartDBG"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart_dbg(&self) -> EnblWrGroup4ofUartDbgR {
        EnblWrGroup4ofUartDbgR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of UartDBG"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart_dbg(&self) -> EnblWrGroup5ofUartDbgR {
        EnblWrGroup5ofUartDbgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1248PRIC1_248\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1248pric12481308(
        &self,
    ) -> EnblRstToleranceOfPric1248pric12481308R {
        EnblRstToleranceOfPric1248pric12481308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1248PRIC1_248\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1248pric12481408(&self) -> EnblWrProtOfPric1248pric12481408R {
        EnblWrProtOfPric1248pric12481408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of UartDMA"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart_dma(&mut self) -> EnblWrGroup0ofUartDmaW<PricIo248Spec> {
        EnblWrGroup0ofUartDmaW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of UartDMA"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart_dma(&mut self) -> EnblWrGroup1ofUartDmaW<PricIo248Spec> {
        EnblWrGroup1ofUartDmaW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of UartDMA"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart_dma(&mut self) -> EnblWrGroup2ofUartDmaW<PricIo248Spec> {
        EnblWrGroup2ofUartDmaW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of UartDMA"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart_dma(&mut self) -> EnblWrGroup3ofUartDmaW<PricIo248Spec> {
        EnblWrGroup3ofUartDmaW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of UartDMA"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart_dma(&mut self) -> EnblWrGroup4ofUartDmaW<PricIo248Spec> {
        EnblWrGroup4ofUartDmaW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of UartDMA"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart_dma(&mut self) -> EnblWrGroup5ofUartDmaW<PricIo248Spec> {
        EnblWrGroup5ofUartDmaW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1248PRIC1_248\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1248pric12480500(
        &mut self,
    ) -> EnblRstToleranceOfPric1248pric12480500W<PricIo248Spec> {
        EnblRstToleranceOfPric1248pric12480500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1248PRIC1_248\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1248pric12480600(
        &mut self,
    ) -> EnblWrProtOfPric1248pric12480600W<PricIo248Spec> {
        EnblWrProtOfPric1248pric12480600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of UartDBG"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart_dbg(&mut self) -> EnblWrGroup0ofUartDbgW<PricIo248Spec> {
        EnblWrGroup0ofUartDbgW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of UartDBG"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart_dbg(&mut self) -> EnblWrGroup1ofUartDbgW<PricIo248Spec> {
        EnblWrGroup1ofUartDbgW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of UartDBG"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart_dbg(&mut self) -> EnblWrGroup2ofUartDbgW<PricIo248Spec> {
        EnblWrGroup2ofUartDbgW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of UartDBG"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart_dbg(&mut self) -> EnblWrGroup3ofUartDbgW<PricIo248Spec> {
        EnblWrGroup3ofUartDbgW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of UartDBG"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart_dbg(&mut self) -> EnblWrGroup4ofUartDbgW<PricIo248Spec> {
        EnblWrGroup4ofUartDbgW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of UartDBG"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart_dbg(&mut self) -> EnblWrGroup5ofUartDbgW<PricIo248Spec> {
        EnblWrGroup5ofUartDbgW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1248PRIC1_248\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1248pric12481308(
        &mut self,
    ) -> EnblRstToleranceOfPric1248pric12481308W<PricIo248Spec> {
        EnblRstToleranceOfPric1248pric12481308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1248PRIC1_248\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1248pric12481408(
        &mut self,
    ) -> EnblWrProtOfPric1248pric12481408W<PricIo248Spec> {
        EnblWrProtOfPric1248pric12481408W::new(self, 15)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved15(&mut self) -> Reserved15W<PricIo248Spec> {
        Reserved15W::new(self, 16)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&mut self) -> Reserved14W<PricIo248Spec> {
        Reserved14W::new(self, 17)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn reserved13(&mut self) -> Reserved13W<PricIo248Spec> {
        Reserved13W::new(self, 18)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn reserved12(&mut self) -> Reserved12W<PricIo248Spec> {
        Reserved12W::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&mut self) -> Reserved11W<PricIo248Spec> {
        Reserved11W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&mut self) -> Reserved10W<PricIo248Spec> {
        Reserved10W::new(self, 21)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&mut self) -> Reserved9W<PricIo248Spec> {
        Reserved9W::new(self, 22)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> Reserved8W<PricIo248Spec> {
        Reserved8W::new(self, 23)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo248Spec> {
        Reserved7W::new(self, 24)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo248Spec> {
        Reserved6W::new(self, 25)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo248Spec> {
        Reserved5W::new(self, 26)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo248Spec> {
        Reserved4W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo248Spec> {
        Reserved3W::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo248Spec> {
        Reserved2W::new(self, 29)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo248Spec> {
        Reserved1W::new(self, 30)
    }
}
#[doc = "Slave Write Group Setting Register \\#18\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io248::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io248::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo248Spec;
impl crate::RegisterSpec for PricIo248Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io248::R`](R) reader structure"]
impl crate::Readable for PricIo248Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io248::W`](W) writer structure"]
impl crate::Writable for PricIo248Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO248 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo248Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
