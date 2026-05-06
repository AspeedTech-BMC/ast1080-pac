#[doc = "Register `UARTDMA1CC` reader"]
pub type R = crate::R<Uartdma1ccSpec>;
#[doc = "Register `UARTDMA1CC` writer"]
pub type W = crate::W<Uartdma1ccSpec>;
#[doc = "VUART0 TX buffer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vuart0txbufSize {
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
impl From<Vuart0txbufSize> for u8 {
    #[inline(always)]
    fn from(variant: Vuart0txbufSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vuart0txbufSize {
    type Ux = u8;
}
impl crate::IsEnum for Vuart0txbufSize {}
#[doc = "Field `VUART0TXBufSize` reader - VUART0 TX buffer size"]
pub type Vuart0txbufSizeR = crate::FieldReader<Vuart0txbufSize>;
impl Vuart0txbufSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vuart0txbufSize> {
        match self.bits {
            0 => Some(Vuart0txbufSize::_1kb),
            1 => Some(Vuart0txbufSize::_4kb),
            2 => Some(Vuart0txbufSize::_16kb),
            3 => Some(Vuart0txbufSize::_64kb),
            4 => Some(Vuart0txbufSize::_128kb),
            5 => Some(Vuart0txbufSize::_256kb),
            6 => Some(Vuart0txbufSize::_512kb),
            7 => Some(Vuart0txbufSize::_1024kb),
            8 => Some(Vuart0txbufSize::_2048kb),
            9 => Some(Vuart0txbufSize::_4096kb),
            10 => Some(Vuart0txbufSize::_8192kb),
            11 => Some(Vuart0txbufSize::_16384kb),
            _ => None,
        }
    }
    #[doc = "1KB"]
    #[inline(always)]
    pub fn is_1kb(&self) -> bool {
        *self == Vuart0txbufSize::_1kb
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == Vuart0txbufSize::_4kb
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn is_16kb(&self) -> bool {
        *self == Vuart0txbufSize::_16kb
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == Vuart0txbufSize::_64kb
    }
    #[doc = "128KB"]
    #[inline(always)]
    pub fn is_128kb(&self) -> bool {
        *self == Vuart0txbufSize::_128kb
    }
    #[doc = "256KB"]
    #[inline(always)]
    pub fn is_256kb(&self) -> bool {
        *self == Vuart0txbufSize::_256kb
    }
    #[doc = "512KB"]
    #[inline(always)]
    pub fn is_512kb(&self) -> bool {
        *self == Vuart0txbufSize::_512kb
    }
    #[doc = "1024KB"]
    #[inline(always)]
    pub fn is_1024kb(&self) -> bool {
        *self == Vuart0txbufSize::_1024kb
    }
    #[doc = "2048KB"]
    #[inline(always)]
    pub fn is_2048kb(&self) -> bool {
        *self == Vuart0txbufSize::_2048kb
    }
    #[doc = "4096KB"]
    #[inline(always)]
    pub fn is_4096kb(&self) -> bool {
        *self == Vuart0txbufSize::_4096kb
    }
    #[doc = "8192KB"]
    #[inline(always)]
    pub fn is_8192kb(&self) -> bool {
        *self == Vuart0txbufSize::_8192kb
    }
    #[doc = "16384KB"]
    #[inline(always)]
    pub fn is_16384kb(&self) -> bool {
        *self == Vuart0txbufSize::_16384kb
    }
}
#[doc = "Field `VUART0TXBufSize` writer - VUART0 TX buffer size"]
pub type Vuart0txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Vuart0txbufSize>;
impl<'a, REG> Vuart0txbufSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1KB"]
    #[inline(always)]
    pub fn _1kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0txbufSize::_1kb)
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0txbufSize::_4kb)
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn _16kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0txbufSize::_16kb)
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0txbufSize::_64kb)
    }
    #[doc = "128KB"]
    #[inline(always)]
    pub fn _128kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0txbufSize::_128kb)
    }
    #[doc = "256KB"]
    #[inline(always)]
    pub fn _256kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0txbufSize::_256kb)
    }
    #[doc = "512KB"]
    #[inline(always)]
    pub fn _512kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0txbufSize::_512kb)
    }
    #[doc = "1024KB"]
    #[inline(always)]
    pub fn _1024kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0txbufSize::_1024kb)
    }
    #[doc = "2048KB"]
    #[inline(always)]
    pub fn _2048kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0txbufSize::_2048kb)
    }
    #[doc = "4096KB"]
    #[inline(always)]
    pub fn _4096kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0txbufSize::_4096kb)
    }
    #[doc = "8192KB"]
    #[inline(always)]
    pub fn _8192kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0txbufSize::_8192kb)
    }
    #[doc = "16384KB"]
    #[inline(always)]
    pub fn _16384kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0txbufSize::_16384kb)
    }
}
#[doc = "VUART0 TX DMA time out disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vuart0txdmatimeOutDisable {
    #[doc = "0: The TX DMA will not start to transmit until at least a full double word data in TX buffer or time out."]
    TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut = 0,
    #[doc = "1: Time out is disabled"]
    TimeOutIsDisabled = 1,
}
impl From<Vuart0txdmatimeOutDisable> for bool {
    #[inline(always)]
    fn from(variant: Vuart0txdmatimeOutDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VUART0TXDMATimeOutDisable` reader - VUART0 TX DMA time out disable"]
pub type Vuart0txdmatimeOutDisableR = crate::BitReader<Vuart0txdmatimeOutDisable>;
impl Vuart0txdmatimeOutDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vuart0txdmatimeOutDisable {
        match self . bits { false => Vuart0txdmatimeOutDisable :: TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut , true => Vuart0txdmatimeOutDisable :: TimeOutIsDisabled , }
    }
    #[doc = "The TX DMA will not start to transmit until at least a full double word data in TX buffer or time out."]
    #[inline(always)]
    pub fn is_the_tx_dma_will_not_start_to_transmit_until_at_least_a_full_double_word_data_in_tx_buffer_or_time_out(
        &self,
    ) -> bool {
        * self == Vuart0txdmatimeOutDisable :: TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut
    }
    #[doc = "Time out is disabled"]
    #[inline(always)]
    pub fn is_time_out_is_disabled(&self) -> bool {
        *self == Vuart0txdmatimeOutDisable::TimeOutIsDisabled
    }
}
#[doc = "Field `VUART0TXDMATimeOutDisable` writer - VUART0 TX DMA time out disable"]
pub type Vuart0txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG, Vuart0txdmatimeOutDisable>;
impl<'a, REG> Vuart0txdmatimeOutDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The TX DMA will not start to transmit until at least a full double word data in TX buffer or time out."]
    #[inline(always)]
    pub fn the_tx_dma_will_not_start_to_transmit_until_at_least_a_full_double_word_data_in_tx_buffer_or_time_out(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Vuart0txdmatimeOutDisable :: TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut)
    }
    #[doc = "Time out is disabled"]
    #[inline(always)]
    pub fn time_out_is_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart0txdmatimeOutDisable::TimeOutIsDisabled)
    }
}
#[doc = "Field `Reserved01` reader - reserved(0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `Reserved01` writer - reserved(0)"]
pub type Reserved01W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VUART0TXBufHighBaseAddr` reader - VUART0 TX buffer high base address"]
pub type Vuart0txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `VUART0TXBufHighBaseAddr` writer - VUART0 TX buffer high base address"]
pub type Vuart0txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - VUART0 TX buffer size"]
    #[inline(always)]
    pub fn vuart0txbuf_size(&self) -> Vuart0txbufSizeR {
        Vuart0txbufSizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - VUART0 TX DMA time out disable"]
    #[inline(always)]
    pub fn vuart0txdmatime_out_disable(&self) -> Vuart0txdmatimeOutDisableR {
        Vuart0txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - VUART0 TX buffer high base address"]
    #[inline(always)]
    pub fn vuart0txbuf_high_base_addr(&self) -> Vuart0txbufHighBaseAddrR {
        Vuart0txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - VUART0 TX buffer size"]
    #[inline(always)]
    pub fn vuart0txbuf_size(&mut self) -> Vuart0txbufSizeW<Uartdma1ccSpec> {
        Vuart0txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - VUART0 TX DMA time out disable"]
    #[inline(always)]
    pub fn vuart0txdmatime_out_disable(&mut self) -> Vuart0txdmatimeOutDisableW<Uartdma1ccSpec> {
        Vuart0txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&mut self) -> Reserved01W<Uartdma1ccSpec> {
        Reserved01W::new(self, 5)
    }
    #[doc = "Bits 8:10 - VUART0 TX buffer high base address"]
    #[inline(always)]
    pub fn vuart0txbuf_high_base_addr(&mut self) -> Vuart0txbufHighBaseAddrW<Uartdma1ccSpec> {
        Vuart0txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "VUART0 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma1ccSpec;
impl crate::RegisterSpec for Uartdma1ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma1cc::R`](R) reader structure"]
impl crate::Readable for Uartdma1ccSpec {}
#[doc = "`write(|w| ..)` method takes [`uartdma1cc::W`](W) writer structure"]
impl crate::Writable for Uartdma1ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA1CC to value 0"]
impl crate::Resettable for Uartdma1ccSpec {}
