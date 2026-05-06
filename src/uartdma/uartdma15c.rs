#[doc = "Register `UARTDMA15C` reader"]
pub type R = crate::R<Uartdma15cSpec>;
#[doc = "Register `UARTDMA15C` writer"]
pub type W = crate::W<Uartdma15cSpec>;
#[doc = "UART9 RX buffer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart9rxbufSize {
    #[doc = "0: 1KB"]
    _1kb = 0,
    #[doc = "1: 4KB"]
    _4kb = 1,
    #[doc = "2: 16KB"]
    _16kb = 2,
    #[doc = "3: 64KB"]
    _64kb = 3,
}
impl From<Uart9rxbufSize> for u8 {
    #[inline(always)]
    fn from(variant: Uart9rxbufSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart9rxbufSize {
    type Ux = u8;
}
impl crate::IsEnum for Uart9rxbufSize {}
#[doc = "Field `UART9RXBufSize` reader - UART9 RX buffer size"]
pub type Uart9rxbufSizeR = crate::FieldReader<Uart9rxbufSize>;
impl Uart9rxbufSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart9rxbufSize {
        match self.bits {
            0 => Uart9rxbufSize::_1kb,
            1 => Uart9rxbufSize::_4kb,
            2 => Uart9rxbufSize::_16kb,
            3 => Uart9rxbufSize::_64kb,
            _ => unreachable!(),
        }
    }
    #[doc = "1KB"]
    #[inline(always)]
    pub fn is_1kb(&self) -> bool {
        *self == Uart9rxbufSize::_1kb
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == Uart9rxbufSize::_4kb
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn is_16kb(&self) -> bool {
        *self == Uart9rxbufSize::_16kb
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == Uart9rxbufSize::_64kb
    }
}
#[doc = "Field `UART9RXBufSize` writer - UART9 RX buffer size"]
pub type Uart9rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart9rxbufSize, crate::Safe>;
impl<'a, REG> Uart9rxbufSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1KB"]
    #[inline(always)]
    pub fn _1kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart9rxbufSize::_1kb)
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart9rxbufSize::_4kb)
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn _16kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart9rxbufSize::_16kb)
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart9rxbufSize::_64kb)
    }
}
#[doc = "Field `Reserved02` reader - reserved(0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "UART9 RX DMA time out disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart9rxdmatimeOutDisable {
    #[doc = "0: The RX DMA will assert interrupt when RX buffer is full or RX buffer is not empty and time out."]
    TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut = 0,
    #[doc = "1: Time out is disabled. The RX DMA will assert interrupt only when RX buffer is full."]
    TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull = 1,
}
impl From<Uart9rxdmatimeOutDisable> for bool {
    #[inline(always)]
    fn from(variant: Uart9rxdmatimeOutDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART9RXDMATimeOutDisable` reader - UART9 RX DMA time out disable"]
pub type Uart9rxdmatimeOutDisableR = crate::BitReader<Uart9rxdmatimeOutDisable>;
impl Uart9rxdmatimeOutDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart9rxdmatimeOutDisable {
        match self . bits { false => Uart9rxdmatimeOutDisable :: TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut , true => Uart9rxdmatimeOutDisable :: TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull , }
    }
    #[doc = "The RX DMA will assert interrupt when RX buffer is full or RX buffer is not empty and time out."]
    #[inline(always)]
    pub fn is_the_rx_dma_will_assert_interrupt_when_rx_buffer_is_full_or_rx_buffer_is_not_empty_and_time_out(
        &self,
    ) -> bool {
        * self == Uart9rxdmatimeOutDisable :: TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut
    }
    #[doc = "Time out is disabled. The RX DMA will assert interrupt only when RX buffer is full."]
    #[inline(always)]
    pub fn is_time_out_is_disabled_the_rx_dma_will_assert_interrupt_only_when_rx_buffer_is_full(
        &self,
    ) -> bool {
        * self == Uart9rxdmatimeOutDisable :: TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull
    }
}
#[doc = "Field `UART9RXDMATimeOutDisable` writer - UART9 RX DMA time out disable"]
pub type Uart9rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG, Uart9rxdmatimeOutDisable>;
impl<'a, REG> Uart9rxdmatimeOutDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RX DMA will assert interrupt when RX buffer is full or RX buffer is not empty and time out."]
    #[inline(always)]
    pub fn the_rx_dma_will_assert_interrupt_when_rx_buffer_is_full_or_rx_buffer_is_not_empty_and_time_out(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Uart9rxdmatimeOutDisable :: TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut)
    }
    #[doc = "Time out is disabled. The RX DMA will assert interrupt only when RX buffer is full."]
    #[inline(always)]
    pub fn time_out_is_disabled_the_rx_dma_will_assert_interrupt_only_when_rx_buffer_is_full(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Uart9rxdmatimeOutDisable :: TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull)
    }
}
#[doc = "UART9 RX DMA full mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart9rxdmafullMode {
    #[doc = "0: RX buffer is full when RX read pointer is equal to RX write pointer minus 1. This is used for ring buffer mode."]
    RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode = 0,
    #[doc = "1: RX buffer is full when RX write pointer is equal to or greater than RX write pointer. This is used for single buffer mode."]
    RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode =
        1,
}
impl From<Uart9rxdmafullMode> for bool {
    #[inline(always)]
    fn from(variant: Uart9rxdmafullMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART9RXDMAFullMode` reader - UART9 RX DMA full mode"]
pub type Uart9rxdmafullModeR = crate::BitReader<Uart9rxdmafullMode>;
impl Uart9rxdmafullModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart9rxdmafullMode {
        match self . bits { false => Uart9rxdmafullMode :: RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode , true => Uart9rxdmafullMode :: RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode , }
    }
    #[doc = "RX buffer is full when RX read pointer is equal to RX write pointer minus 1. This is used for ring buffer mode."]
    #[inline(always)]
    pub fn is_rx_buffer_is_full_when_rx_read_pointer_is_equal_to_rx_write_pointer_minus_1_this_is_used_for_ring_buffer_mode(
        &self,
    ) -> bool {
        * self == Uart9rxdmafullMode :: RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode
    }
    #[doc = "RX buffer is full when RX write pointer is equal to or greater than RX write pointer. This is used for single buffer mode."]
    #[inline(always)]
    pub fn is_rx_buffer_is_full_when_rx_write_pointer_is_equal_to_or_greater_than_rx_write_pointer_this_is_used_for_single_buffer_mode(
        &self,
    ) -> bool {
        * self == Uart9rxdmafullMode :: RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode
    }
}
#[doc = "Field `UART9RXDMAFullMode` writer - UART9 RX DMA full mode"]
pub type Uart9rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG, Uart9rxdmafullMode>;
impl<'a, REG> Uart9rxdmafullModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX buffer is full when RX read pointer is equal to RX write pointer minus 1. This is used for ring buffer mode."]
    #[inline(always)]
    pub fn rx_buffer_is_full_when_rx_read_pointer_is_equal_to_rx_write_pointer_minus_1_this_is_used_for_ring_buffer_mode(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Uart9rxdmafullMode :: RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode)
    }
    #[doc = "RX buffer is full when RX write pointer is equal to or greater than RX write pointer. This is used for single buffer mode."]
    #[inline(always)]
    pub fn rx_buffer_is_full_when_rx_write_pointer_is_equal_to_or_greater_than_rx_write_pointer_this_is_used_for_single_buffer_mode(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Uart9rxdmafullMode :: RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode)
    }
}
#[doc = "Field `Reserved01` reader - reserved(0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `Reserved01` writer - reserved(0)"]
pub type Reserved01W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART9RXBufHighBaseAddr` reader - UART9 RX buffer high base address"]
pub type Uart9rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART9RXBufHighBaseAddr` writer - UART9 RX buffer high base address"]
pub type Uart9rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - UART9 RX buffer size"]
    #[inline(always)]
    pub fn uart9rxbuf_size(&self) -> Uart9rxbufSizeR {
        Uart9rxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART9 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart9rxdmatime_out_disable(&self) -> Uart9rxdmatimeOutDisableR {
        Uart9rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART9 RX DMA full mode"]
    #[inline(always)]
    pub fn uart9rxdmafull_mode(&self) -> Uart9rxdmafullModeR {
        Uart9rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - UART9 RX buffer high base address"]
    #[inline(always)]
    pub fn uart9rxbuf_high_base_addr(&self) -> Uart9rxbufHighBaseAddrR {
        Uart9rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART9 RX buffer size"]
    #[inline(always)]
    pub fn uart9rxbuf_size(&mut self) -> Uart9rxbufSizeW<Uartdma15cSpec> {
        Uart9rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART9 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart9rxdmatime_out_disable(&mut self) -> Uart9rxdmatimeOutDisableW<Uartdma15cSpec> {
        Uart9rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - UART9 RX DMA full mode"]
    #[inline(always)]
    pub fn uart9rxdmafull_mode(&mut self) -> Uart9rxdmafullModeW<Uartdma15cSpec> {
        Uart9rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&mut self) -> Reserved01W<Uartdma15cSpec> {
        Reserved01W::new(self, 6)
    }
    #[doc = "Bits 8:10 - UART9 RX buffer high base address"]
    #[inline(always)]
    pub fn uart9rxbuf_high_base_addr(&mut self) -> Uart9rxbufHighBaseAddrW<Uartdma15cSpec> {
        Uart9rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART9 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma15c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma15c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma15cSpec;
impl crate::RegisterSpec for Uartdma15cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma15c::R`](R) reader structure"]
impl crate::Readable for Uartdma15cSpec {}
#[doc = "`write(|w| ..)` method takes [`uartdma15c::W`](W) writer structure"]
impl crate::Writable for Uartdma15cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA15C to value 0"]
impl crate::Resettable for Uartdma15cSpec {}
