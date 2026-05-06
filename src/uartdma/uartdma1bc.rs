#[doc = "Register `UARTDMA1BC` reader"]
pub type R = crate::R<Uartdma1bcSpec>;
#[doc = "Register `UARTDMA1BC` writer"]
pub type W = crate::W<Uartdma1bcSpec>;
#[doc = "UART-BMC RX buffer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UartbmcrxbufSize {
    #[doc = "0: 1KB"]
    _1kb = 0,
    #[doc = "1: 4KB"]
    _4kb = 1,
    #[doc = "2: 16KB"]
    _16kb = 2,
    #[doc = "3: 64KB"]
    _64kb = 3,
}
impl From<UartbmcrxbufSize> for u8 {
    #[inline(always)]
    fn from(variant: UartbmcrxbufSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UartbmcrxbufSize {
    type Ux = u8;
}
impl crate::IsEnum for UartbmcrxbufSize {}
#[doc = "Field `UARTBMCRXBufSize` reader - UART-BMC RX buffer size"]
pub type UartbmcrxbufSizeR = crate::FieldReader<UartbmcrxbufSize>;
impl UartbmcrxbufSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UartbmcrxbufSize {
        match self.bits {
            0 => UartbmcrxbufSize::_1kb,
            1 => UartbmcrxbufSize::_4kb,
            2 => UartbmcrxbufSize::_16kb,
            3 => UartbmcrxbufSize::_64kb,
            _ => unreachable!(),
        }
    }
    #[doc = "1KB"]
    #[inline(always)]
    pub fn is_1kb(&self) -> bool {
        *self == UartbmcrxbufSize::_1kb
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == UartbmcrxbufSize::_4kb
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn is_16kb(&self) -> bool {
        *self == UartbmcrxbufSize::_16kb
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == UartbmcrxbufSize::_64kb
    }
}
#[doc = "Field `UARTBMCRXBufSize` writer - UART-BMC RX buffer size"]
pub type UartbmcrxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, UartbmcrxbufSize, crate::Safe>;
impl<'a, REG> UartbmcrxbufSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1KB"]
    #[inline(always)]
    pub fn _1kb(self) -> &'a mut crate::W<REG> {
        self.variant(UartbmcrxbufSize::_1kb)
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut crate::W<REG> {
        self.variant(UartbmcrxbufSize::_4kb)
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn _16kb(self) -> &'a mut crate::W<REG> {
        self.variant(UartbmcrxbufSize::_16kb)
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut crate::W<REG> {
        self.variant(UartbmcrxbufSize::_64kb)
    }
}
#[doc = "Field `Reserved02` reader - reserved(0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "UART-BMC RX DMA time out disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UartbmcrxdmatimeOutDisable {
    #[doc = "0: The RX DMA will assert interrupt when RX buffer is full or RX buffer is not empty and time out."]
    TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut = 0,
    #[doc = "1: Time out is disabled. The RX DMA will assert interrupt only when RX buffer is full."]
    TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull = 1,
}
impl From<UartbmcrxdmatimeOutDisable> for bool {
    #[inline(always)]
    fn from(variant: UartbmcrxdmatimeOutDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UARTBMCRXDMATimeOutDisable` reader - UART-BMC RX DMA time out disable"]
pub type UartbmcrxdmatimeOutDisableR = crate::BitReader<UartbmcrxdmatimeOutDisable>;
impl UartbmcrxdmatimeOutDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UartbmcrxdmatimeOutDisable {
        match self . bits { false => UartbmcrxdmatimeOutDisable :: TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut , true => UartbmcrxdmatimeOutDisable :: TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull , }
    }
    #[doc = "The RX DMA will assert interrupt when RX buffer is full or RX buffer is not empty and time out."]
    #[inline(always)]
    pub fn is_the_rx_dma_will_assert_interrupt_when_rx_buffer_is_full_or_rx_buffer_is_not_empty_and_time_out(
        &self,
    ) -> bool {
        * self == UartbmcrxdmatimeOutDisable :: TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut
    }
    #[doc = "Time out is disabled. The RX DMA will assert interrupt only when RX buffer is full."]
    #[inline(always)]
    pub fn is_time_out_is_disabled_the_rx_dma_will_assert_interrupt_only_when_rx_buffer_is_full(
        &self,
    ) -> bool {
        * self == UartbmcrxdmatimeOutDisable :: TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull
    }
}
#[doc = "Field `UARTBMCRXDMATimeOutDisable` writer - UART-BMC RX DMA time out disable"]
pub type UartbmcrxdmatimeOutDisableW<'a, REG> =
    crate::BitWriter<'a, REG, UartbmcrxdmatimeOutDisable>;
impl<'a, REG> UartbmcrxdmatimeOutDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RX DMA will assert interrupt when RX buffer is full or RX buffer is not empty and time out."]
    #[inline(always)]
    pub fn the_rx_dma_will_assert_interrupt_when_rx_buffer_is_full_or_rx_buffer_is_not_empty_and_time_out(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (UartbmcrxdmatimeOutDisable :: TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut)
    }
    #[doc = "Time out is disabled. The RX DMA will assert interrupt only when RX buffer is full."]
    #[inline(always)]
    pub fn time_out_is_disabled_the_rx_dma_will_assert_interrupt_only_when_rx_buffer_is_full(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (UartbmcrxdmatimeOutDisable :: TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull)
    }
}
#[doc = "UART-BMC RX DMA full mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UartbmcrxdmafullMode {
    #[doc = "0: RX buffer is full when RX read pointer is equal to RX write pointer minus 1. This is used for ring buffer mode."]
    RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode = 0,
    #[doc = "1: RX buffer is full when RX write pointer is equal to or greater than RX write pointer. This is used for single buffer mode."]
    RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode =
        1,
}
impl From<UartbmcrxdmafullMode> for bool {
    #[inline(always)]
    fn from(variant: UartbmcrxdmafullMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UARTBMCRXDMAFullMode` reader - UART-BMC RX DMA full mode"]
pub type UartbmcrxdmafullModeR = crate::BitReader<UartbmcrxdmafullMode>;
impl UartbmcrxdmafullModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UartbmcrxdmafullMode {
        match self . bits { false => UartbmcrxdmafullMode :: RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode , true => UartbmcrxdmafullMode :: RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode , }
    }
    #[doc = "RX buffer is full when RX read pointer is equal to RX write pointer minus 1. This is used for ring buffer mode."]
    #[inline(always)]
    pub fn is_rx_buffer_is_full_when_rx_read_pointer_is_equal_to_rx_write_pointer_minus_1_this_is_used_for_ring_buffer_mode(
        &self,
    ) -> bool {
        * self == UartbmcrxdmafullMode :: RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode
    }
    #[doc = "RX buffer is full when RX write pointer is equal to or greater than RX write pointer. This is used for single buffer mode."]
    #[inline(always)]
    pub fn is_rx_buffer_is_full_when_rx_write_pointer_is_equal_to_or_greater_than_rx_write_pointer_this_is_used_for_single_buffer_mode(
        &self,
    ) -> bool {
        * self == UartbmcrxdmafullMode :: RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode
    }
}
#[doc = "Field `UARTBMCRXDMAFullMode` writer - UART-BMC RX DMA full mode"]
pub type UartbmcrxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG, UartbmcrxdmafullMode>;
impl<'a, REG> UartbmcrxdmafullModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX buffer is full when RX read pointer is equal to RX write pointer minus 1. This is used for ring buffer mode."]
    #[inline(always)]
    pub fn rx_buffer_is_full_when_rx_read_pointer_is_equal_to_rx_write_pointer_minus_1_this_is_used_for_ring_buffer_mode(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (UartbmcrxdmafullMode :: RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode)
    }
    #[doc = "RX buffer is full when RX write pointer is equal to or greater than RX write pointer. This is used for single buffer mode."]
    #[inline(always)]
    pub fn rx_buffer_is_full_when_rx_write_pointer_is_equal_to_or_greater_than_rx_write_pointer_this_is_used_for_single_buffer_mode(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (UartbmcrxdmafullMode :: RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode)
    }
}
#[doc = "Field `Reserved01` reader - reserved(0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `Reserved01` writer - reserved(0)"]
pub type Reserved01W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UARTBMCRXBufHighBaseAddr` reader - UART-BMC RX buffer high base address"]
pub type UartbmcrxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UARTBMCRXBufHighBaseAddr` writer - UART-BMC RX buffer high base address"]
pub type UartbmcrxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - UART-BMC RX buffer size"]
    #[inline(always)]
    pub fn uartbmcrxbuf_size(&self) -> UartbmcrxbufSizeR {
        UartbmcrxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART-BMC RX DMA time out disable"]
    #[inline(always)]
    pub fn uartbmcrxdmatime_out_disable(&self) -> UartbmcrxdmatimeOutDisableR {
        UartbmcrxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART-BMC RX DMA full mode"]
    #[inline(always)]
    pub fn uartbmcrxdmafull_mode(&self) -> UartbmcrxdmafullModeR {
        UartbmcrxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - UART-BMC RX buffer high base address"]
    #[inline(always)]
    pub fn uartbmcrxbuf_high_base_addr(&self) -> UartbmcrxbufHighBaseAddrR {
        UartbmcrxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART-BMC RX buffer size"]
    #[inline(always)]
    pub fn uartbmcrxbuf_size(&mut self) -> UartbmcrxbufSizeW<Uartdma1bcSpec> {
        UartbmcrxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART-BMC RX DMA time out disable"]
    #[inline(always)]
    pub fn uartbmcrxdmatime_out_disable(&mut self) -> UartbmcrxdmatimeOutDisableW<Uartdma1bcSpec> {
        UartbmcrxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - UART-BMC RX DMA full mode"]
    #[inline(always)]
    pub fn uartbmcrxdmafull_mode(&mut self) -> UartbmcrxdmafullModeW<Uartdma1bcSpec> {
        UartbmcrxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&mut self) -> Reserved01W<Uartdma1bcSpec> {
        Reserved01W::new(self, 6)
    }
    #[doc = "Bits 8:10 - UART-BMC RX buffer high base address"]
    #[inline(always)]
    pub fn uartbmcrxbuf_high_base_addr(&mut self) -> UartbmcrxbufHighBaseAddrW<Uartdma1bcSpec> {
        UartbmcrxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART-BMC RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma1bcSpec;
impl crate::RegisterSpec for Uartdma1bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma1bc::R`](R) reader structure"]
impl crate::Readable for Uartdma1bcSpec {}
#[doc = "`write(|w| ..)` method takes [`uartdma1bc::W`](W) writer structure"]
impl crate::Writable for Uartdma1bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA1BC to value 0"]
impl crate::Resettable for Uartdma1bcSpec {}
