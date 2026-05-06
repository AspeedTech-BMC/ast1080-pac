#[doc = "Register `UARTDMA19C` reader"]
pub type R = crate::R<Uartdma19cSpec>;
#[doc = "Register `UARTDMA19C` writer"]
pub type W = crate::W<Uartdma19cSpec>;
#[doc = "UART11 RX buffer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart11rxbufSize {
    #[doc = "0: 1KB"]
    _1kb = 0,
    #[doc = "1: 4KB"]
    _4kb = 1,
    #[doc = "2: 16KB"]
    _16kb = 2,
    #[doc = "3: 64KB"]
    _64kb = 3,
}
impl From<Uart11rxbufSize> for u8 {
    #[inline(always)]
    fn from(variant: Uart11rxbufSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart11rxbufSize {
    type Ux = u8;
}
impl crate::IsEnum for Uart11rxbufSize {}
#[doc = "Field `UART11RXBufSize` reader - UART11 RX buffer size"]
pub type Uart11rxbufSizeR = crate::FieldReader<Uart11rxbufSize>;
impl Uart11rxbufSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart11rxbufSize {
        match self.bits {
            0 => Uart11rxbufSize::_1kb,
            1 => Uart11rxbufSize::_4kb,
            2 => Uart11rxbufSize::_16kb,
            3 => Uart11rxbufSize::_64kb,
            _ => unreachable!(),
        }
    }
    #[doc = "1KB"]
    #[inline(always)]
    pub fn is_1kb(&self) -> bool {
        *self == Uart11rxbufSize::_1kb
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == Uart11rxbufSize::_4kb
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn is_16kb(&self) -> bool {
        *self == Uart11rxbufSize::_16kb
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == Uart11rxbufSize::_64kb
    }
}
#[doc = "Field `UART11RXBufSize` writer - UART11 RX buffer size"]
pub type Uart11rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart11rxbufSize, crate::Safe>;
impl<'a, REG> Uart11rxbufSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1KB"]
    #[inline(always)]
    pub fn _1kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart11rxbufSize::_1kb)
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart11rxbufSize::_4kb)
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn _16kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart11rxbufSize::_16kb)
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart11rxbufSize::_64kb)
    }
}
#[doc = "Field `Reserved02` reader - reserved(0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "UART11 RX DMA time out disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart11rxdmatimeOutDisable {
    #[doc = "0: The RX DMA will assert interrupt when RX buffer is full or RX buffer is not empty and time out."]
    TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut = 0,
    #[doc = "1: Time out is disabled. The RX DMA will assert interrupt only when RX buffer is full."]
    TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull = 1,
}
impl From<Uart11rxdmatimeOutDisable> for bool {
    #[inline(always)]
    fn from(variant: Uart11rxdmatimeOutDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART11RXDMATimeOutDisable` reader - UART11 RX DMA time out disable"]
pub type Uart11rxdmatimeOutDisableR = crate::BitReader<Uart11rxdmatimeOutDisable>;
impl Uart11rxdmatimeOutDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart11rxdmatimeOutDisable {
        match self . bits { false => Uart11rxdmatimeOutDisable :: TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut , true => Uart11rxdmatimeOutDisable :: TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull , }
    }
    #[doc = "The RX DMA will assert interrupt when RX buffer is full or RX buffer is not empty and time out."]
    #[inline(always)]
    pub fn is_the_rx_dma_will_assert_interrupt_when_rx_buffer_is_full_or_rx_buffer_is_not_empty_and_time_out(
        &self,
    ) -> bool {
        * self == Uart11rxdmatimeOutDisable :: TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut
    }
    #[doc = "Time out is disabled. The RX DMA will assert interrupt only when RX buffer is full."]
    #[inline(always)]
    pub fn is_time_out_is_disabled_the_rx_dma_will_assert_interrupt_only_when_rx_buffer_is_full(
        &self,
    ) -> bool {
        * self == Uart11rxdmatimeOutDisable :: TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull
    }
}
#[doc = "Field `UART11RXDMATimeOutDisable` writer - UART11 RX DMA time out disable"]
pub type Uart11rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG, Uart11rxdmatimeOutDisable>;
impl<'a, REG> Uart11rxdmatimeOutDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RX DMA will assert interrupt when RX buffer is full or RX buffer is not empty and time out."]
    #[inline(always)]
    pub fn the_rx_dma_will_assert_interrupt_when_rx_buffer_is_full_or_rx_buffer_is_not_empty_and_time_out(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Uart11rxdmatimeOutDisable :: TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut)
    }
    #[doc = "Time out is disabled. The RX DMA will assert interrupt only when RX buffer is full."]
    #[inline(always)]
    pub fn time_out_is_disabled_the_rx_dma_will_assert_interrupt_only_when_rx_buffer_is_full(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Uart11rxdmatimeOutDisable :: TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull)
    }
}
#[doc = "UART11 RX DMA full mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart11rxdmafullMode {
    #[doc = "0: RX buffer is full when RX read pointer is equal to RX write pointer minus 1. This is used for ring buffer mode."]
    RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode = 0,
    #[doc = "1: RX buffer is full when RX write pointer is equal to or greater than RX write pointer. This is used for single buffer mode."]
    RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode =
        1,
}
impl From<Uart11rxdmafullMode> for bool {
    #[inline(always)]
    fn from(variant: Uart11rxdmafullMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART11RXDMAFullMode` reader - UART11 RX DMA full mode"]
pub type Uart11rxdmafullModeR = crate::BitReader<Uart11rxdmafullMode>;
impl Uart11rxdmafullModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart11rxdmafullMode {
        match self . bits { false => Uart11rxdmafullMode :: RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode , true => Uart11rxdmafullMode :: RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode , }
    }
    #[doc = "RX buffer is full when RX read pointer is equal to RX write pointer minus 1. This is used for ring buffer mode."]
    #[inline(always)]
    pub fn is_rx_buffer_is_full_when_rx_read_pointer_is_equal_to_rx_write_pointer_minus_1_this_is_used_for_ring_buffer_mode(
        &self,
    ) -> bool {
        * self == Uart11rxdmafullMode :: RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode
    }
    #[doc = "RX buffer is full when RX write pointer is equal to or greater than RX write pointer. This is used for single buffer mode."]
    #[inline(always)]
    pub fn is_rx_buffer_is_full_when_rx_write_pointer_is_equal_to_or_greater_than_rx_write_pointer_this_is_used_for_single_buffer_mode(
        &self,
    ) -> bool {
        * self == Uart11rxdmafullMode :: RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode
    }
}
#[doc = "Field `UART11RXDMAFullMode` writer - UART11 RX DMA full mode"]
pub type Uart11rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG, Uart11rxdmafullMode>;
impl<'a, REG> Uart11rxdmafullModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX buffer is full when RX read pointer is equal to RX write pointer minus 1. This is used for ring buffer mode."]
    #[inline(always)]
    pub fn rx_buffer_is_full_when_rx_read_pointer_is_equal_to_rx_write_pointer_minus_1_this_is_used_for_ring_buffer_mode(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Uart11rxdmafullMode :: RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode)
    }
    #[doc = "RX buffer is full when RX write pointer is equal to or greater than RX write pointer. This is used for single buffer mode."]
    #[inline(always)]
    pub fn rx_buffer_is_full_when_rx_write_pointer_is_equal_to_or_greater_than_rx_write_pointer_this_is_used_for_single_buffer_mode(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Uart11rxdmafullMode :: RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode)
    }
}
#[doc = "Field `Reserved01` reader - reserved(0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `Reserved01` writer - reserved(0)"]
pub type Reserved01W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART11RXBufHighBaseAddr` reader - UART11 RX buffer high base address"]
pub type Uart11rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART11RXBufHighBaseAddr` writer - UART11 RX buffer high base address"]
pub type Uart11rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - UART11 RX buffer size"]
    #[inline(always)]
    pub fn uart11rxbuf_size(&self) -> Uart11rxbufSizeR {
        Uart11rxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART11 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart11rxdmatime_out_disable(&self) -> Uart11rxdmatimeOutDisableR {
        Uart11rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART11 RX DMA full mode"]
    #[inline(always)]
    pub fn uart11rxdmafull_mode(&self) -> Uart11rxdmafullModeR {
        Uart11rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - UART11 RX buffer high base address"]
    #[inline(always)]
    pub fn uart11rxbuf_high_base_addr(&self) -> Uart11rxbufHighBaseAddrR {
        Uart11rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART11 RX buffer size"]
    #[inline(always)]
    pub fn uart11rxbuf_size(&mut self) -> Uart11rxbufSizeW<Uartdma19cSpec> {
        Uart11rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART11 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart11rxdmatime_out_disable(&mut self) -> Uart11rxdmatimeOutDisableW<Uartdma19cSpec> {
        Uart11rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - UART11 RX DMA full mode"]
    #[inline(always)]
    pub fn uart11rxdmafull_mode(&mut self) -> Uart11rxdmafullModeW<Uartdma19cSpec> {
        Uart11rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&mut self) -> Reserved01W<Uartdma19cSpec> {
        Reserved01W::new(self, 6)
    }
    #[doc = "Bits 8:10 - UART11 RX buffer high base address"]
    #[inline(always)]
    pub fn uart11rxbuf_high_base_addr(&mut self) -> Uart11rxbufHighBaseAddrW<Uartdma19cSpec> {
        Uart11rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART11 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma19c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma19c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma19cSpec;
impl crate::RegisterSpec for Uartdma19cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma19c::R`](R) reader structure"]
impl crate::Readable for Uartdma19cSpec {}
#[doc = "`write(|w| ..)` method takes [`uartdma19c::W`](W) writer structure"]
impl crate::Writable for Uartdma19cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA19C to value 0"]
impl crate::Resettable for Uartdma19cSpec {}
