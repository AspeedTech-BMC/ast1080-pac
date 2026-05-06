#[doc = "Register `UARTDMA14C` reader"]
pub type R = crate::R<Uartdma14cSpec>;
#[doc = "Register `UARTDMA14C` writer"]
pub type W = crate::W<Uartdma14cSpec>;
#[doc = "UART9 TX buffer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart9txbufSize {
    #[doc = "0: 1KB"]
    _1kb = 0,
    #[doc = "1: 4KB"]
    _4kb = 1,
    #[doc = "2: 16KB"]
    _16kb = 2,
    #[doc = "3: 64KB"]
    _64kb = 3,
}
impl From<Uart9txbufSize> for u8 {
    #[inline(always)]
    fn from(variant: Uart9txbufSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart9txbufSize {
    type Ux = u8;
}
impl crate::IsEnum for Uart9txbufSize {}
#[doc = "Field `UART9TXBufSize` reader - UART9 TX buffer size"]
pub type Uart9txbufSizeR = crate::FieldReader<Uart9txbufSize>;
impl Uart9txbufSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart9txbufSize {
        match self.bits {
            0 => Uart9txbufSize::_1kb,
            1 => Uart9txbufSize::_4kb,
            2 => Uart9txbufSize::_16kb,
            3 => Uart9txbufSize::_64kb,
            _ => unreachable!(),
        }
    }
    #[doc = "1KB"]
    #[inline(always)]
    pub fn is_1kb(&self) -> bool {
        *self == Uart9txbufSize::_1kb
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == Uart9txbufSize::_4kb
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn is_16kb(&self) -> bool {
        *self == Uart9txbufSize::_16kb
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == Uart9txbufSize::_64kb
    }
}
#[doc = "Field `UART9TXBufSize` writer - UART9 TX buffer size"]
pub type Uart9txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart9txbufSize, crate::Safe>;
impl<'a, REG> Uart9txbufSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1KB"]
    #[inline(always)]
    pub fn _1kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart9txbufSize::_1kb)
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart9txbufSize::_4kb)
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn _16kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart9txbufSize::_16kb)
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart9txbufSize::_64kb)
    }
}
#[doc = "Field `Reserved02` reader - reserved(0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "UART9 TX DMA time out disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart9txdmatimeOutDisable {
    #[doc = "0: The TX DMA will not start to transmit until at least a full double word data in TX buffer or time out."]
    TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut = 0,
    #[doc = "1: Time out is disabled"]
    TimeOutIsDisabled = 1,
}
impl From<Uart9txdmatimeOutDisable> for bool {
    #[inline(always)]
    fn from(variant: Uart9txdmatimeOutDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART9TXDMATimeOutDisable` reader - UART9 TX DMA time out disable"]
pub type Uart9txdmatimeOutDisableR = crate::BitReader<Uart9txdmatimeOutDisable>;
impl Uart9txdmatimeOutDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart9txdmatimeOutDisable {
        match self . bits { false => Uart9txdmatimeOutDisable :: TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut , true => Uart9txdmatimeOutDisable :: TimeOutIsDisabled , }
    }
    #[doc = "The TX DMA will not start to transmit until at least a full double word data in TX buffer or time out."]
    #[inline(always)]
    pub fn is_the_tx_dma_will_not_start_to_transmit_until_at_least_a_full_double_word_data_in_tx_buffer_or_time_out(
        &self,
    ) -> bool {
        * self == Uart9txdmatimeOutDisable :: TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut
    }
    #[doc = "Time out is disabled"]
    #[inline(always)]
    pub fn is_time_out_is_disabled(&self) -> bool {
        *self == Uart9txdmatimeOutDisable::TimeOutIsDisabled
    }
}
#[doc = "Field `UART9TXDMATimeOutDisable` writer - UART9 TX DMA time out disable"]
pub type Uart9txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG, Uart9txdmatimeOutDisable>;
impl<'a, REG> Uart9txdmatimeOutDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The TX DMA will not start to transmit until at least a full double word data in TX buffer or time out."]
    #[inline(always)]
    pub fn the_tx_dma_will_not_start_to_transmit_until_at_least_a_full_double_word_data_in_tx_buffer_or_time_out(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Uart9txdmatimeOutDisable :: TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut)
    }
    #[doc = "Time out is disabled"]
    #[inline(always)]
    pub fn time_out_is_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Uart9txdmatimeOutDisable::TimeOutIsDisabled)
    }
}
#[doc = "Field `Reserved01` reader - reserved(0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `Reserved01` writer - reserved(0)"]
pub type Reserved01W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART9TXBufHighBaseAddr` reader - UART9 TX buffer high base address"]
pub type Uart9txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART9TXBufHighBaseAddr` writer - UART9 TX buffer high base address"]
pub type Uart9txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - UART9 TX buffer size"]
    #[inline(always)]
    pub fn uart9txbuf_size(&self) -> Uart9txbufSizeR {
        Uart9txbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART9 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart9txdmatime_out_disable(&self) -> Uart9txdmatimeOutDisableR {
        Uart9txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - UART9 TX buffer high base address"]
    #[inline(always)]
    pub fn uart9txbuf_high_base_addr(&self) -> Uart9txbufHighBaseAddrR {
        Uart9txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART9 TX buffer size"]
    #[inline(always)]
    pub fn uart9txbuf_size(&mut self) -> Uart9txbufSizeW<Uartdma14cSpec> {
        Uart9txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART9 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart9txdmatime_out_disable(&mut self) -> Uart9txdmatimeOutDisableW<Uartdma14cSpec> {
        Uart9txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&mut self) -> Reserved01W<Uartdma14cSpec> {
        Reserved01W::new(self, 5)
    }
    #[doc = "Bits 8:10 - UART9 TX buffer high base address"]
    #[inline(always)]
    pub fn uart9txbuf_high_base_addr(&mut self) -> Uart9txbufHighBaseAddrW<Uartdma14cSpec> {
        Uart9txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART9 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma14c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma14c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma14cSpec;
impl crate::RegisterSpec for Uartdma14cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma14c::R`](R) reader structure"]
impl crate::Readable for Uartdma14cSpec {}
#[doc = "`write(|w| ..)` method takes [`uartdma14c::W`](W) writer structure"]
impl crate::Writable for Uartdma14cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA14C to value 0"]
impl crate::Resettable for Uartdma14cSpec {}
