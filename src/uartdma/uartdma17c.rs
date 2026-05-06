#[doc = "Register `UARTDMA17C` reader"]
pub type R = crate::R<Uartdma17cSpec>;
#[doc = "Register `UARTDMA17C` writer"]
pub type W = crate::W<Uartdma17cSpec>;
#[doc = "UART10 RX buffer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart10rxbufSize {
    #[doc = "0: 1KB"]
    _1kb = 0,
    #[doc = "1: 4KB"]
    _4kb = 1,
    #[doc = "2: 16KB"]
    _16kb = 2,
    #[doc = "3: 64KB"]
    _64kb = 3,
}
impl From<Uart10rxbufSize> for u8 {
    #[inline(always)]
    fn from(variant: Uart10rxbufSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart10rxbufSize {
    type Ux = u8;
}
impl crate::IsEnum for Uart10rxbufSize {}
#[doc = "Field `UART10RXBufSize` reader - UART10 RX buffer size"]
pub type Uart10rxbufSizeR = crate::FieldReader<Uart10rxbufSize>;
impl Uart10rxbufSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart10rxbufSize {
        match self.bits {
            0 => Uart10rxbufSize::_1kb,
            1 => Uart10rxbufSize::_4kb,
            2 => Uart10rxbufSize::_16kb,
            3 => Uart10rxbufSize::_64kb,
            _ => unreachable!(),
        }
    }
    #[doc = "1KB"]
    #[inline(always)]
    pub fn is_1kb(&self) -> bool {
        *self == Uart10rxbufSize::_1kb
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == Uart10rxbufSize::_4kb
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn is_16kb(&self) -> bool {
        *self == Uart10rxbufSize::_16kb
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == Uart10rxbufSize::_64kb
    }
}
#[doc = "Field `UART10RXBufSize` writer - UART10 RX buffer size"]
pub type Uart10rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart10rxbufSize, crate::Safe>;
impl<'a, REG> Uart10rxbufSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1KB"]
    #[inline(always)]
    pub fn _1kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart10rxbufSize::_1kb)
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart10rxbufSize::_4kb)
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn _16kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart10rxbufSize::_16kb)
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart10rxbufSize::_64kb)
    }
}
#[doc = "Field `Reserved02` reader - reserved(0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "UART10 RX DMA time out disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart10rxdmatimeOutDisable {
    #[doc = "0: The RX DMA will assert interrupt when RX buffer is full or RX buffer is not empty and time out."]
    TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut = 0,
    #[doc = "1: Time out is disabled. The RX DMA will assert interrupt only when RX buffer is full."]
    TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull = 1,
}
impl From<Uart10rxdmatimeOutDisable> for bool {
    #[inline(always)]
    fn from(variant: Uart10rxdmatimeOutDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART10RXDMATimeOutDisable` reader - UART10 RX DMA time out disable"]
pub type Uart10rxdmatimeOutDisableR = crate::BitReader<Uart10rxdmatimeOutDisable>;
impl Uart10rxdmatimeOutDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart10rxdmatimeOutDisable {
        match self . bits { false => Uart10rxdmatimeOutDisable :: TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut , true => Uart10rxdmatimeOutDisable :: TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull , }
    }
    #[doc = "The RX DMA will assert interrupt when RX buffer is full or RX buffer is not empty and time out."]
    #[inline(always)]
    pub fn is_the_rx_dma_will_assert_interrupt_when_rx_buffer_is_full_or_rx_buffer_is_not_empty_and_time_out(
        &self,
    ) -> bool {
        * self == Uart10rxdmatimeOutDisable :: TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut
    }
    #[doc = "Time out is disabled. The RX DMA will assert interrupt only when RX buffer is full."]
    #[inline(always)]
    pub fn is_time_out_is_disabled_the_rx_dma_will_assert_interrupt_only_when_rx_buffer_is_full(
        &self,
    ) -> bool {
        * self == Uart10rxdmatimeOutDisable :: TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull
    }
}
#[doc = "Field `UART10RXDMATimeOutDisable` writer - UART10 RX DMA time out disable"]
pub type Uart10rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG, Uart10rxdmatimeOutDisable>;
impl<'a, REG> Uart10rxdmatimeOutDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RX DMA will assert interrupt when RX buffer is full or RX buffer is not empty and time out."]
    #[inline(always)]
    pub fn the_rx_dma_will_assert_interrupt_when_rx_buffer_is_full_or_rx_buffer_is_not_empty_and_time_out(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Uart10rxdmatimeOutDisable :: TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut)
    }
    #[doc = "Time out is disabled. The RX DMA will assert interrupt only when RX buffer is full."]
    #[inline(always)]
    pub fn time_out_is_disabled_the_rx_dma_will_assert_interrupt_only_when_rx_buffer_is_full(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Uart10rxdmatimeOutDisable :: TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull)
    }
}
#[doc = "UART10 RX DMA full mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart10rxdmafullMode {
    #[doc = "0: RX buffer is full when RX read pointer is equal to RX write pointer minus 1. This is used for ring buffer mode."]
    RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode = 0,
    #[doc = "1: RX buffer is full when RX write pointer is equal to or greater than RX write pointer. This is used for single buffer mode."]
    RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode =
        1,
}
impl From<Uart10rxdmafullMode> for bool {
    #[inline(always)]
    fn from(variant: Uart10rxdmafullMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART10RXDMAFullMode` reader - UART10 RX DMA full mode"]
pub type Uart10rxdmafullModeR = crate::BitReader<Uart10rxdmafullMode>;
impl Uart10rxdmafullModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart10rxdmafullMode {
        match self . bits { false => Uart10rxdmafullMode :: RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode , true => Uart10rxdmafullMode :: RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode , }
    }
    #[doc = "RX buffer is full when RX read pointer is equal to RX write pointer minus 1. This is used for ring buffer mode."]
    #[inline(always)]
    pub fn is_rx_buffer_is_full_when_rx_read_pointer_is_equal_to_rx_write_pointer_minus_1_this_is_used_for_ring_buffer_mode(
        &self,
    ) -> bool {
        * self == Uart10rxdmafullMode :: RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode
    }
    #[doc = "RX buffer is full when RX write pointer is equal to or greater than RX write pointer. This is used for single buffer mode."]
    #[inline(always)]
    pub fn is_rx_buffer_is_full_when_rx_write_pointer_is_equal_to_or_greater_than_rx_write_pointer_this_is_used_for_single_buffer_mode(
        &self,
    ) -> bool {
        * self == Uart10rxdmafullMode :: RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode
    }
}
#[doc = "Field `UART10RXDMAFullMode` writer - UART10 RX DMA full mode"]
pub type Uart10rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG, Uart10rxdmafullMode>;
impl<'a, REG> Uart10rxdmafullModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX buffer is full when RX read pointer is equal to RX write pointer minus 1. This is used for ring buffer mode."]
    #[inline(always)]
    pub fn rx_buffer_is_full_when_rx_read_pointer_is_equal_to_rx_write_pointer_minus_1_this_is_used_for_ring_buffer_mode(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Uart10rxdmafullMode :: RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode)
    }
    #[doc = "RX buffer is full when RX write pointer is equal to or greater than RX write pointer. This is used for single buffer mode."]
    #[inline(always)]
    pub fn rx_buffer_is_full_when_rx_write_pointer_is_equal_to_or_greater_than_rx_write_pointer_this_is_used_for_single_buffer_mode(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Uart10rxdmafullMode :: RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode)
    }
}
#[doc = "Field `Reserved01` reader - reserved(0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `Reserved01` writer - reserved(0)"]
pub type Reserved01W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART10RXBufHighBaseAddr` reader - UART10 RX buffer high base address"]
pub type Uart10rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART10RXBufHighBaseAddr` writer - UART10 RX buffer high base address"]
pub type Uart10rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - UART10 RX buffer size"]
    #[inline(always)]
    pub fn uart10rxbuf_size(&self) -> Uart10rxbufSizeR {
        Uart10rxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART10 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart10rxdmatime_out_disable(&self) -> Uart10rxdmatimeOutDisableR {
        Uart10rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART10 RX DMA full mode"]
    #[inline(always)]
    pub fn uart10rxdmafull_mode(&self) -> Uart10rxdmafullModeR {
        Uart10rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - UART10 RX buffer high base address"]
    #[inline(always)]
    pub fn uart10rxbuf_high_base_addr(&self) -> Uart10rxbufHighBaseAddrR {
        Uart10rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART10 RX buffer size"]
    #[inline(always)]
    pub fn uart10rxbuf_size(&mut self) -> Uart10rxbufSizeW<Uartdma17cSpec> {
        Uart10rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART10 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart10rxdmatime_out_disable(&mut self) -> Uart10rxdmatimeOutDisableW<Uartdma17cSpec> {
        Uart10rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - UART10 RX DMA full mode"]
    #[inline(always)]
    pub fn uart10rxdmafull_mode(&mut self) -> Uart10rxdmafullModeW<Uartdma17cSpec> {
        Uart10rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&mut self) -> Reserved01W<Uartdma17cSpec> {
        Reserved01W::new(self, 6)
    }
    #[doc = "Bits 8:10 - UART10 RX buffer high base address"]
    #[inline(always)]
    pub fn uart10rxbuf_high_base_addr(&mut self) -> Uart10rxbufHighBaseAddrW<Uartdma17cSpec> {
        Uart10rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART10 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma17c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma17c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma17cSpec;
impl crate::RegisterSpec for Uartdma17cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma17c::R`](R) reader structure"]
impl crate::Readable for Uartdma17cSpec {}
#[doc = "`write(|w| ..)` method takes [`uartdma17c::W`](W) writer structure"]
impl crate::Writable for Uartdma17cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA17C to value 0"]
impl crate::Resettable for Uartdma17cSpec {}
