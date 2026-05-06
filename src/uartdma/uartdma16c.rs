#[doc = "Register `UARTDMA16C` reader"]
pub type R = crate::R<Uartdma16cSpec>;
#[doc = "Register `UARTDMA16C` writer"]
pub type W = crate::W<Uartdma16cSpec>;
#[doc = "UART10 TX buffer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart10txbufSize {
    #[doc = "0: 1KB"]
    _1kb = 0,
    #[doc = "1: 4KB"]
    _4kb = 1,
    #[doc = "2: 16KB"]
    _16kb = 2,
    #[doc = "3: 64KB"]
    _64kb = 3,
}
impl From<Uart10txbufSize> for u8 {
    #[inline(always)]
    fn from(variant: Uart10txbufSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart10txbufSize {
    type Ux = u8;
}
impl crate::IsEnum for Uart10txbufSize {}
#[doc = "Field `UART10TXBufSize` reader - UART10 TX buffer size"]
pub type Uart10txbufSizeR = crate::FieldReader<Uart10txbufSize>;
impl Uart10txbufSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart10txbufSize {
        match self.bits {
            0 => Uart10txbufSize::_1kb,
            1 => Uart10txbufSize::_4kb,
            2 => Uart10txbufSize::_16kb,
            3 => Uart10txbufSize::_64kb,
            _ => unreachable!(),
        }
    }
    #[doc = "1KB"]
    #[inline(always)]
    pub fn is_1kb(&self) -> bool {
        *self == Uart10txbufSize::_1kb
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == Uart10txbufSize::_4kb
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn is_16kb(&self) -> bool {
        *self == Uart10txbufSize::_16kb
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == Uart10txbufSize::_64kb
    }
}
#[doc = "Field `UART10TXBufSize` writer - UART10 TX buffer size"]
pub type Uart10txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart10txbufSize, crate::Safe>;
impl<'a, REG> Uart10txbufSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1KB"]
    #[inline(always)]
    pub fn _1kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart10txbufSize::_1kb)
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart10txbufSize::_4kb)
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn _16kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart10txbufSize::_16kb)
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut crate::W<REG> {
        self.variant(Uart10txbufSize::_64kb)
    }
}
#[doc = "Field `Reserved02` reader - reserved(0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "UART10 TX DMA time out disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart10txdmatimeOutDisable {
    #[doc = "0: The TX DMA will not start to transmit until at least a full double word data in TX buffer or time out."]
    TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut = 0,
    #[doc = "1: Time out is disabled"]
    TimeOutIsDisabled = 1,
}
impl From<Uart10txdmatimeOutDisable> for bool {
    #[inline(always)]
    fn from(variant: Uart10txdmatimeOutDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART10TXDMATimeOutDisable` reader - UART10 TX DMA time out disable"]
pub type Uart10txdmatimeOutDisableR = crate::BitReader<Uart10txdmatimeOutDisable>;
impl Uart10txdmatimeOutDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart10txdmatimeOutDisable {
        match self . bits { false => Uart10txdmatimeOutDisable :: TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut , true => Uart10txdmatimeOutDisable :: TimeOutIsDisabled , }
    }
    #[doc = "The TX DMA will not start to transmit until at least a full double word data in TX buffer or time out."]
    #[inline(always)]
    pub fn is_the_tx_dma_will_not_start_to_transmit_until_at_least_a_full_double_word_data_in_tx_buffer_or_time_out(
        &self,
    ) -> bool {
        * self == Uart10txdmatimeOutDisable :: TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut
    }
    #[doc = "Time out is disabled"]
    #[inline(always)]
    pub fn is_time_out_is_disabled(&self) -> bool {
        *self == Uart10txdmatimeOutDisable::TimeOutIsDisabled
    }
}
#[doc = "Field `UART10TXDMATimeOutDisable` writer - UART10 TX DMA time out disable"]
pub type Uart10txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG, Uart10txdmatimeOutDisable>;
impl<'a, REG> Uart10txdmatimeOutDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The TX DMA will not start to transmit until at least a full double word data in TX buffer or time out."]
    #[inline(always)]
    pub fn the_tx_dma_will_not_start_to_transmit_until_at_least_a_full_double_word_data_in_tx_buffer_or_time_out(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Uart10txdmatimeOutDisable :: TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut)
    }
    #[doc = "Time out is disabled"]
    #[inline(always)]
    pub fn time_out_is_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Uart10txdmatimeOutDisable::TimeOutIsDisabled)
    }
}
#[doc = "Field `Reserved01` reader - reserved(0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `Reserved01` writer - reserved(0)"]
pub type Reserved01W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART10TXBufHighBaseAddr` reader - UART10 TX buffer high base address"]
pub type Uart10txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART10TXBufHighBaseAddr` writer - UART10 TX buffer high base address"]
pub type Uart10txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - UART10 TX buffer size"]
    #[inline(always)]
    pub fn uart10txbuf_size(&self) -> Uart10txbufSizeR {
        Uart10txbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART10 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart10txdmatime_out_disable(&self) -> Uart10txdmatimeOutDisableR {
        Uart10txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - UART10 TX buffer high base address"]
    #[inline(always)]
    pub fn uart10txbuf_high_base_addr(&self) -> Uart10txbufHighBaseAddrR {
        Uart10txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART10 TX buffer size"]
    #[inline(always)]
    pub fn uart10txbuf_size(&mut self) -> Uart10txbufSizeW<Uartdma16cSpec> {
        Uart10txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART10 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart10txdmatime_out_disable(&mut self) -> Uart10txdmatimeOutDisableW<Uartdma16cSpec> {
        Uart10txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&mut self) -> Reserved01W<Uartdma16cSpec> {
        Reserved01W::new(self, 5)
    }
    #[doc = "Bits 8:10 - UART10 TX buffer high base address"]
    #[inline(always)]
    pub fn uart10txbuf_high_base_addr(&mut self) -> Uart10txbufHighBaseAddrW<Uartdma16cSpec> {
        Uart10txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART10 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma16c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma16c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma16cSpec;
impl crate::RegisterSpec for Uartdma16cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma16c::R`](R) reader structure"]
impl crate::Readable for Uartdma16cSpec {}
#[doc = "`write(|w| ..)` method takes [`uartdma16c::W`](W) writer structure"]
impl crate::Writable for Uartdma16cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA16C to value 0"]
impl crate::Resettable for Uartdma16cSpec {}
