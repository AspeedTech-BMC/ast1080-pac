#[doc = "Register `UARTDMA1AC` reader"]
pub type R = crate::R<Uartdma1acSpec>;
#[doc = "Register `UARTDMA1AC` writer"]
pub type W = crate::W<Uartdma1acSpec>;
#[doc = "UART-BMC TX buffer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UartbmctxbufSize {
    #[doc = "0: 1KB"]
    _1kb = 0,
    #[doc = "1: 4KB"]
    _4kb = 1,
    #[doc = "2: 16KB"]
    _16kb = 2,
    #[doc = "3: 64KB"]
    _64kb = 3,
}
impl From<UartbmctxbufSize> for u8 {
    #[inline(always)]
    fn from(variant: UartbmctxbufSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UartbmctxbufSize {
    type Ux = u8;
}
impl crate::IsEnum for UartbmctxbufSize {}
#[doc = "Field `UARTBMCTXBufSize` reader - UART-BMC TX buffer size"]
pub type UartbmctxbufSizeR = crate::FieldReader<UartbmctxbufSize>;
impl UartbmctxbufSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UartbmctxbufSize {
        match self.bits {
            0 => UartbmctxbufSize::_1kb,
            1 => UartbmctxbufSize::_4kb,
            2 => UartbmctxbufSize::_16kb,
            3 => UartbmctxbufSize::_64kb,
            _ => unreachable!(),
        }
    }
    #[doc = "1KB"]
    #[inline(always)]
    pub fn is_1kb(&self) -> bool {
        *self == UartbmctxbufSize::_1kb
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == UartbmctxbufSize::_4kb
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn is_16kb(&self) -> bool {
        *self == UartbmctxbufSize::_16kb
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == UartbmctxbufSize::_64kb
    }
}
#[doc = "Field `UARTBMCTXBufSize` writer - UART-BMC TX buffer size"]
pub type UartbmctxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, UartbmctxbufSize, crate::Safe>;
impl<'a, REG> UartbmctxbufSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1KB"]
    #[inline(always)]
    pub fn _1kb(self) -> &'a mut crate::W<REG> {
        self.variant(UartbmctxbufSize::_1kb)
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut crate::W<REG> {
        self.variant(UartbmctxbufSize::_4kb)
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn _16kb(self) -> &'a mut crate::W<REG> {
        self.variant(UartbmctxbufSize::_16kb)
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut crate::W<REG> {
        self.variant(UartbmctxbufSize::_64kb)
    }
}
#[doc = "Field `Reserved02` reader - reserved(0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "UART-BMC TX DMA time out disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UartbmctxdmatimeOutDisable {
    #[doc = "0: The TX DMA will not start to transmit until at least a full double word data in TX buffer or time out."]
    TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut = 0,
    #[doc = "1: Time out is disabled"]
    TimeOutIsDisabled = 1,
}
impl From<UartbmctxdmatimeOutDisable> for bool {
    #[inline(always)]
    fn from(variant: UartbmctxdmatimeOutDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UARTBMCTXDMATimeOutDisable` reader - UART-BMC TX DMA time out disable"]
pub type UartbmctxdmatimeOutDisableR = crate::BitReader<UartbmctxdmatimeOutDisable>;
impl UartbmctxdmatimeOutDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UartbmctxdmatimeOutDisable {
        match self . bits { false => UartbmctxdmatimeOutDisable :: TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut , true => UartbmctxdmatimeOutDisable :: TimeOutIsDisabled , }
    }
    #[doc = "The TX DMA will not start to transmit until at least a full double word data in TX buffer or time out."]
    #[inline(always)]
    pub fn is_the_tx_dma_will_not_start_to_transmit_until_at_least_a_full_double_word_data_in_tx_buffer_or_time_out(
        &self,
    ) -> bool {
        * self == UartbmctxdmatimeOutDisable :: TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut
    }
    #[doc = "Time out is disabled"]
    #[inline(always)]
    pub fn is_time_out_is_disabled(&self) -> bool {
        *self == UartbmctxdmatimeOutDisable::TimeOutIsDisabled
    }
}
#[doc = "Field `UARTBMCTXDMATimeOutDisable` writer - UART-BMC TX DMA time out disable"]
pub type UartbmctxdmatimeOutDisableW<'a, REG> =
    crate::BitWriter<'a, REG, UartbmctxdmatimeOutDisable>;
impl<'a, REG> UartbmctxdmatimeOutDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The TX DMA will not start to transmit until at least a full double word data in TX buffer or time out."]
    #[inline(always)]
    pub fn the_tx_dma_will_not_start_to_transmit_until_at_least_a_full_double_word_data_in_tx_buffer_or_time_out(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (UartbmctxdmatimeOutDisable :: TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut)
    }
    #[doc = "Time out is disabled"]
    #[inline(always)]
    pub fn time_out_is_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UartbmctxdmatimeOutDisable::TimeOutIsDisabled)
    }
}
#[doc = "Field `Reserved01` reader - reserved(0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `Reserved01` writer - reserved(0)"]
pub type Reserved01W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UARTBMCTXBufHighBaseAddr` reader - UART-BMC TX buffer high base address"]
pub type UartbmctxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UARTBMCTXBufHighBaseAddr` writer - UART-BMC TX buffer high base address"]
pub type UartbmctxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - UART-BMC TX buffer size"]
    #[inline(always)]
    pub fn uartbmctxbuf_size(&self) -> UartbmctxbufSizeR {
        UartbmctxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART-BMC TX DMA time out disable"]
    #[inline(always)]
    pub fn uartbmctxdmatime_out_disable(&self) -> UartbmctxdmatimeOutDisableR {
        UartbmctxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - UART-BMC TX buffer high base address"]
    #[inline(always)]
    pub fn uartbmctxbuf_high_base_addr(&self) -> UartbmctxbufHighBaseAddrR {
        UartbmctxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART-BMC TX buffer size"]
    #[inline(always)]
    pub fn uartbmctxbuf_size(&mut self) -> UartbmctxbufSizeW<Uartdma1acSpec> {
        UartbmctxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART-BMC TX DMA time out disable"]
    #[inline(always)]
    pub fn uartbmctxdmatime_out_disable(&mut self) -> UartbmctxdmatimeOutDisableW<Uartdma1acSpec> {
        UartbmctxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&mut self) -> Reserved01W<Uartdma1acSpec> {
        Reserved01W::new(self, 5)
    }
    #[doc = "Bits 8:10 - UART-BMC TX buffer high base address"]
    #[inline(always)]
    pub fn uartbmctxbuf_high_base_addr(&mut self) -> UartbmctxbufHighBaseAddrW<Uartdma1acSpec> {
        UartbmctxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART-BMC TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma1acSpec;
impl crate::RegisterSpec for Uartdma1acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma1ac::R`](R) reader structure"]
impl crate::Readable for Uartdma1acSpec {}
#[doc = "`write(|w| ..)` method takes [`uartdma1ac::W`](W) writer structure"]
impl crate::Writable for Uartdma1acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA1AC to value 0"]
impl crate::Resettable for Uartdma1acSpec {}
