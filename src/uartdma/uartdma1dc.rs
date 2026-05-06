#[doc = "Register `UARTDMA1DC` reader"]
pub type R = crate::R<Uartdma1dcSpec>;
#[doc = "Register `UARTDMA1DC` writer"]
pub type W = crate::W<Uartdma1dcSpec>;
#[doc = "VUART0 RX buffer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vuart0rxbufSize {
    #[doc = "0: 1KB"]
    _1kb = 0,
    #[doc = "1: 4KB"]
    _4kb = 1,
    #[doc = "2: 16KB"]
    _16kb = 2,
    #[doc = "3: 64KB"]
    _64kb = 3,
    #[doc = "4: 128KB"]
    _128kb = 4,
    #[doc = "5: 256KB"]
    _256kb = 5,
    #[doc = "6: 512KB"]
    _512kb = 6,
    #[doc = "7: 1024KB"]
    _1024kb = 7,
    #[doc = "8: 2048KB"]
    _2048kb = 8,
    #[doc = "9: 4096KB"]
    _4096kb = 9,
    #[doc = "10: 8192KB"]
    _8192kb = 10,
    #[doc = "11: 16384KB"]
    _16384kb = 11,
}
impl From<Vuart0rxbufSize> for u8 {
    #[inline(always)]
    fn from(variant: Vuart0rxbufSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vuart0rxbufSize {
    type Ux = u8;
}
impl crate::IsEnum for Vuart0rxbufSize {}
#[doc = "Field `VUART0RXBufSize` reader - VUART0 RX buffer size"]
pub type Vuart0rxbufSizeR = crate::FieldReader<Vuart0rxbufSize>;
impl Vuart0rxbufSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vuart0rxbufSize> {
        match self.bits {
            0 => Some(Vuart0rxbufSize::_1kb),
            1 => Some(Vuart0rxbufSize::_4kb),
            2 => Some(Vuart0rxbufSize::_16kb),
            3 => Some(Vuart0rxbufSize::_64kb),
            4 => Some(Vuart0rxbufSize::_128kb),
            5 => Some(Vuart0rxbufSize::_256kb),
            6 => Some(Vuart0rxbufSize::_512kb),
            7 => Some(Vuart0rxbufSize::_1024kb),
            8 => Some(Vuart0rxbufSize::_2048kb),
            9 => Some(Vuart0rxbufSize::_4096kb),
            10 => Some(Vuart0rxbufSize::_8192kb),
            11 => Some(Vuart0rxbufSize::_16384kb),
            _ => None,
        }
    }
    #[doc = "1KB"]
    #[inline(always)]
    pub fn is_1kb(&self) -> bool {
        *self == Vuart0rxbufSize::_1kb
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == Vuart0rxbufSize::_4kb
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn is_16kb(&self) -> bool {
        *self == Vuart0rxbufSize::_16kb
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == Vuart0rxbufSize::_64kb
    }
    #[doc = "128KB"]
    #[inline(always)]
    pub fn is_128kb(&self) -> bool {
        *self == Vuart0rxbufSize::_128kb
    }
    #[doc = "256KB"]
    #[inline(always)]
    pub fn is_256kb(&self) -> bool {
        *self == Vuart0rxbufSize::_256kb
    }
    #[doc = "512KB"]
    #[inline(always)]
    pub fn is_512kb(&self) -> bool {
        *self == Vuart0rxbufSize::_512kb
    }
    #[doc = "1024KB"]
    #[inline(always)]
    pub fn is_1024kb(&self) -> bool {
        *self == Vuart0rxbufSize::_1024kb
    }
    #[doc = "2048KB"]
    #[inline(always)]
    pub fn is_2048kb(&self) -> bool {
        *self == Vuart0rxbufSize::_2048kb
    }
    #[doc = "4096KB"]
    #[inline(always)]
    pub fn is_4096kb(&self) -> bool {
        *self == Vuart0rxbufSize::_4096kb
    }
    #[doc = "8192KB"]
    #[inline(always)]
    pub fn is_8192kb(&self) -> bool {
        *self == Vuart0rxbufSize::_8192kb
    }
    #[doc = "16384KB"]
    #[inline(always)]
    pub fn is_16384kb(&self) -> bool {
        *self == Vuart0rxbufSize::_16384kb
    }
}
#[doc = "Field `VUART0RXBufSize` writer - VUART0 RX buffer size"]
pub type Vuart0rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Vuart0rxbufSize>;
impl<'a, REG> Vuart0rxbufSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1KB"]
    #[inline(always)]
    pub fn _1kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0rxbufSize::_1kb)
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0rxbufSize::_4kb)
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn _16kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0rxbufSize::_16kb)
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0rxbufSize::_64kb)
    }
    #[doc = "128KB"]
    #[inline(always)]
    pub fn _128kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0rxbufSize::_128kb)
    }
    #[doc = "256KB"]
    #[inline(always)]
    pub fn _256kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0rxbufSize::_256kb)
    }
    #[doc = "512KB"]
    #[inline(always)]
    pub fn _512kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0rxbufSize::_512kb)
    }
    #[doc = "1024KB"]
    #[inline(always)]
    pub fn _1024kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0rxbufSize::_1024kb)
    }
    #[doc = "2048KB"]
    #[inline(always)]
    pub fn _2048kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0rxbufSize::_2048kb)
    }
    #[doc = "4096KB"]
    #[inline(always)]
    pub fn _4096kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0rxbufSize::_4096kb)
    }
    #[doc = "8192KB"]
    #[inline(always)]
    pub fn _8192kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0rxbufSize::_8192kb)
    }
    #[doc = "16384KB"]
    #[inline(always)]
    pub fn _16384kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0rxbufSize::_16384kb)
    }
}
#[doc = "VUART0 RX DMA time out disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vuart0rxdmatimeOutDisable {
    #[doc = "0: The RX DMA will assert interrupt when RX buffer is full or RX buffer is not empty and time out."]
    TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut = 0,
    #[doc = "1: Time out is disabled. The RX DMA will assert interrupt only when RX buffer is full."]
    TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull = 1,
}
impl From<Vuart0rxdmatimeOutDisable> for bool {
    #[inline(always)]
    fn from(variant: Vuart0rxdmatimeOutDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VUART0RXDMATimeOutDisable` reader - VUART0 RX DMA time out disable"]
pub type Vuart0rxdmatimeOutDisableR = crate::BitReader<Vuart0rxdmatimeOutDisable>;
impl Vuart0rxdmatimeOutDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vuart0rxdmatimeOutDisable {
        match self . bits { false => Vuart0rxdmatimeOutDisable :: TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut , true => Vuart0rxdmatimeOutDisable :: TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull , }
    }
    #[doc = "The RX DMA will assert interrupt when RX buffer is full or RX buffer is not empty and time out."]
    #[inline(always)]
    pub fn is_the_rx_dma_will_assert_interrupt_when_rx_buffer_is_full_or_rx_buffer_is_not_empty_and_time_out(
        &self,
    ) -> bool {
        * self == Vuart0rxdmatimeOutDisable :: TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut
    }
    #[doc = "Time out is disabled. The RX DMA will assert interrupt only when RX buffer is full."]
    #[inline(always)]
    pub fn is_time_out_is_disabled_the_rx_dma_will_assert_interrupt_only_when_rx_buffer_is_full(
        &self,
    ) -> bool {
        * self == Vuart0rxdmatimeOutDisable :: TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull
    }
}
#[doc = "Field `VUART0RXDMATimeOutDisable` writer - VUART0 RX DMA time out disable"]
pub type Vuart0rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG, Vuart0rxdmatimeOutDisable>;
impl<'a, REG> Vuart0rxdmatimeOutDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RX DMA will assert interrupt when RX buffer is full or RX buffer is not empty and time out."]
    #[inline(always)]
    pub fn the_rx_dma_will_assert_interrupt_when_rx_buffer_is_full_or_rx_buffer_is_not_empty_and_time_out(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Vuart0rxdmatimeOutDisable :: TheRxDmaWillAssertInterruptWhenRxBufferIsFullOrRxBufferIsNotEmptyAndTimeOut)
    }
    #[doc = "Time out is disabled. The RX DMA will assert interrupt only when RX buffer is full."]
    #[inline(always)]
    pub fn time_out_is_disabled_the_rx_dma_will_assert_interrupt_only_when_rx_buffer_is_full(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Vuart0rxdmatimeOutDisable :: TimeOutIsDisabledTheRxDmaWillAssertInterruptOnlyWhenRxBufferIsFull)
    }
}
#[doc = "VUART0 RX DMA full mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vuart0rxdmafullMode {
    #[doc = "0: RX buffer is full when RX read pointer is equal to RX write pointer minus 1. This is used for ring buffer mode."]
    RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode = 0,
    #[doc = "1: RX buffer is full when RX write pointer is equal to or greater than RX write pointer. This is used for single buffer mode."]
    RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode =
        1,
}
impl From<Vuart0rxdmafullMode> for bool {
    #[inline(always)]
    fn from(variant: Vuart0rxdmafullMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VUART0RXDMAFullMode` reader - VUART0 RX DMA full mode"]
pub type Vuart0rxdmafullModeR = crate::BitReader<Vuart0rxdmafullMode>;
impl Vuart0rxdmafullModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vuart0rxdmafullMode {
        match self . bits { false => Vuart0rxdmafullMode :: RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode , true => Vuart0rxdmafullMode :: RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode , }
    }
    #[doc = "RX buffer is full when RX read pointer is equal to RX write pointer minus 1. This is used for ring buffer mode."]
    #[inline(always)]
    pub fn is_rx_buffer_is_full_when_rx_read_pointer_is_equal_to_rx_write_pointer_minus_1_this_is_used_for_ring_buffer_mode(
        &self,
    ) -> bool {
        * self == Vuart0rxdmafullMode :: RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode
    }
    #[doc = "RX buffer is full when RX write pointer is equal to or greater than RX write pointer. This is used for single buffer mode."]
    #[inline(always)]
    pub fn is_rx_buffer_is_full_when_rx_write_pointer_is_equal_to_or_greater_than_rx_write_pointer_this_is_used_for_single_buffer_mode(
        &self,
    ) -> bool {
        * self == Vuart0rxdmafullMode :: RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode
    }
}
#[doc = "Field `VUART0RXDMAFullMode` writer - VUART0 RX DMA full mode"]
pub type Vuart0rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG, Vuart0rxdmafullMode>;
impl<'a, REG> Vuart0rxdmafullModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX buffer is full when RX read pointer is equal to RX write pointer minus 1. This is used for ring buffer mode."]
    #[inline(always)]
    pub fn rx_buffer_is_full_when_rx_read_pointer_is_equal_to_rx_write_pointer_minus_1_this_is_used_for_ring_buffer_mode(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Vuart0rxdmafullMode :: RxBufferIsFullWhenRxReadPointerIsEqualToRxWritePointerMinus1ThisIsUsedForRingBufferMode)
    }
    #[doc = "RX buffer is full when RX write pointer is equal to or greater than RX write pointer. This is used for single buffer mode."]
    #[inline(always)]
    pub fn rx_buffer_is_full_when_rx_write_pointer_is_equal_to_or_greater_than_rx_write_pointer_this_is_used_for_single_buffer_mode(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Vuart0rxdmafullMode :: RxBufferIsFullWhenRxWritePointerIsEqualToOrGreaterThanRxWritePointerThisIsUsedForSingleBufferMode)
    }
}
#[doc = "Field `Reserved01` reader - reserved(0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `Reserved01` writer - reserved(0)"]
pub type Reserved01W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VUART0RXBufHighBaseAddr` reader - VUART0 RX buffer high base address"]
pub type Vuart0rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `VUART0RXBufHighBaseAddr` writer - VUART0 RX buffer high base address"]
pub type Vuart0rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - VUART0 RX buffer size"]
    #[inline(always)]
    pub fn vuart0rxbuf_size(&self) -> Vuart0rxbufSizeR {
        Vuart0rxbufSizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - VUART0 RX DMA time out disable"]
    #[inline(always)]
    pub fn vuart0rxdmatime_out_disable(&self) -> Vuart0rxdmatimeOutDisableR {
        Vuart0rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VUART0 RX DMA full mode"]
    #[inline(always)]
    pub fn vuart0rxdmafull_mode(&self) -> Vuart0rxdmafullModeR {
        Vuart0rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - VUART0 RX buffer high base address"]
    #[inline(always)]
    pub fn vuart0rxbuf_high_base_addr(&self) -> Vuart0rxbufHighBaseAddrR {
        Vuart0rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - VUART0 RX buffer size"]
    #[inline(always)]
    pub fn vuart0rxbuf_size(&mut self) -> Vuart0rxbufSizeW<Uartdma1dcSpec> {
        Vuart0rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - VUART0 RX DMA time out disable"]
    #[inline(always)]
    pub fn vuart0rxdmatime_out_disable(&mut self) -> Vuart0rxdmatimeOutDisableW<Uartdma1dcSpec> {
        Vuart0rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - VUART0 RX DMA full mode"]
    #[inline(always)]
    pub fn vuart0rxdmafull_mode(&mut self) -> Vuart0rxdmafullModeW<Uartdma1dcSpec> {
        Vuart0rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&mut self) -> Reserved01W<Uartdma1dcSpec> {
        Reserved01W::new(self, 6)
    }
    #[doc = "Bits 8:10 - VUART0 RX buffer high base address"]
    #[inline(always)]
    pub fn vuart0rxbuf_high_base_addr(&mut self) -> Vuart0rxbufHighBaseAddrW<Uartdma1dcSpec> {
        Vuart0rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "VUART0 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma1dcSpec;
impl crate::RegisterSpec for Uartdma1dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma1dc::R`](R) reader structure"]
impl crate::Readable for Uartdma1dcSpec {}
#[doc = "`write(|w| ..)` method takes [`uartdma1dc::W`](W) writer structure"]
impl crate::Writable for Uartdma1dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA1DC to value 0"]
impl crate::Resettable for Uartdma1dcSpec {}
