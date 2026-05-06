#[doc = "Register `UARTDMA20C` reader"]
pub type R = crate::R<Uartdma20cSpec>;
#[doc = "Register `UARTDMA20C` writer"]
pub type W = crate::W<Uartdma20cSpec>;
#[doc = "VUART2 TX buffer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vuart2txbufSize {
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
impl From<Vuart2txbufSize> for u8 {
    #[inline(always)]
    fn from(variant: Vuart2txbufSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vuart2txbufSize {
    type Ux = u8;
}
impl crate::IsEnum for Vuart2txbufSize {}
#[doc = "Field `VUART2TXBufSize` reader - VUART2 TX buffer size"]
pub type Vuart2txbufSizeR = crate::FieldReader<Vuart2txbufSize>;
impl Vuart2txbufSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vuart2txbufSize> {
        match self.bits {
            0 => Some(Vuart2txbufSize::_1kb),
            1 => Some(Vuart2txbufSize::_4kb),
            2 => Some(Vuart2txbufSize::_16kb),
            3 => Some(Vuart2txbufSize::_64kb),
            4 => Some(Vuart2txbufSize::_128kb),
            5 => Some(Vuart2txbufSize::_256kb),
            6 => Some(Vuart2txbufSize::_512kb),
            7 => Some(Vuart2txbufSize::_1024kb),
            8 => Some(Vuart2txbufSize::_2048kb),
            9 => Some(Vuart2txbufSize::_4096kb),
            10 => Some(Vuart2txbufSize::_8192kb),
            11 => Some(Vuart2txbufSize::_16384kb),
            _ => None,
        }
    }
    #[doc = "1KB"]
    #[inline(always)]
    pub fn is_1kb(&self) -> bool {
        *self == Vuart2txbufSize::_1kb
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == Vuart2txbufSize::_4kb
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn is_16kb(&self) -> bool {
        *self == Vuart2txbufSize::_16kb
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn is_64kb(&self) -> bool {
        *self == Vuart2txbufSize::_64kb
    }
    #[doc = "128KB"]
    #[inline(always)]
    pub fn is_128kb(&self) -> bool {
        *self == Vuart2txbufSize::_128kb
    }
    #[doc = "256KB"]
    #[inline(always)]
    pub fn is_256kb(&self) -> bool {
        *self == Vuart2txbufSize::_256kb
    }
    #[doc = "512KB"]
    #[inline(always)]
    pub fn is_512kb(&self) -> bool {
        *self == Vuart2txbufSize::_512kb
    }
    #[doc = "1024KB"]
    #[inline(always)]
    pub fn is_1024kb(&self) -> bool {
        *self == Vuart2txbufSize::_1024kb
    }
    #[doc = "2048KB"]
    #[inline(always)]
    pub fn is_2048kb(&self) -> bool {
        *self == Vuart2txbufSize::_2048kb
    }
    #[doc = "4096KB"]
    #[inline(always)]
    pub fn is_4096kb(&self) -> bool {
        *self == Vuart2txbufSize::_4096kb
    }
    #[doc = "8192KB"]
    #[inline(always)]
    pub fn is_8192kb(&self) -> bool {
        *self == Vuart2txbufSize::_8192kb
    }
    #[doc = "16384KB"]
    #[inline(always)]
    pub fn is_16384kb(&self) -> bool {
        *self == Vuart2txbufSize::_16384kb
    }
}
#[doc = "Field `VUART2TXBufSize` writer - VUART2 TX buffer size"]
pub type Vuart2txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Vuart2txbufSize>;
impl<'a, REG> Vuart2txbufSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1KB"]
    #[inline(always)]
    pub fn _1kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart2txbufSize::_1kb)
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart2txbufSize::_4kb)
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn _16kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart2txbufSize::_16kb)
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn _64kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart2txbufSize::_64kb)
    }
    #[doc = "128KB"]
    #[inline(always)]
    pub fn _128kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart2txbufSize::_128kb)
    }
    #[doc = "256KB"]
    #[inline(always)]
    pub fn _256kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart2txbufSize::_256kb)
    }
    #[doc = "512KB"]
    #[inline(always)]
    pub fn _512kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart2txbufSize::_512kb)
    }
    #[doc = "1024KB"]
    #[inline(always)]
    pub fn _1024kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart2txbufSize::_1024kb)
    }
    #[doc = "2048KB"]
    #[inline(always)]
    pub fn _2048kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart2txbufSize::_2048kb)
    }
    #[doc = "4096KB"]
    #[inline(always)]
    pub fn _4096kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart2txbufSize::_4096kb)
    }
    #[doc = "8192KB"]
    #[inline(always)]
    pub fn _8192kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart2txbufSize::_8192kb)
    }
    #[doc = "16384KB"]
    #[inline(always)]
    pub fn _16384kb(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart2txbufSize::_16384kb)
    }
}
#[doc = "VUART2 TX DMA time out disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vuart2txdmatimeOutDisable {
    #[doc = "0: The TX DMA will not start to transmit until at least a full double word data in TX buffer or time out."]
    TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut = 0,
    #[doc = "1: Time out is disabled"]
    TimeOutIsDisabled = 1,
}
impl From<Vuart2txdmatimeOutDisable> for bool {
    #[inline(always)]
    fn from(variant: Vuart2txdmatimeOutDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VUART2TXDMATimeOutDisable` reader - VUART2 TX DMA time out disable"]
pub type Vuart2txdmatimeOutDisableR = crate::BitReader<Vuart2txdmatimeOutDisable>;
impl Vuart2txdmatimeOutDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vuart2txdmatimeOutDisable {
        match self . bits { false => Vuart2txdmatimeOutDisable :: TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut , true => Vuart2txdmatimeOutDisable :: TimeOutIsDisabled , }
    }
    #[doc = "The TX DMA will not start to transmit until at least a full double word data in TX buffer or time out."]
    #[inline(always)]
    pub fn is_the_tx_dma_will_not_start_to_transmit_until_at_least_a_full_double_word_data_in_tx_buffer_or_time_out(
        &self,
    ) -> bool {
        * self == Vuart2txdmatimeOutDisable :: TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut
    }
    #[doc = "Time out is disabled"]
    #[inline(always)]
    pub fn is_time_out_is_disabled(&self) -> bool {
        *self == Vuart2txdmatimeOutDisable::TimeOutIsDisabled
    }
}
#[doc = "Field `VUART2TXDMATimeOutDisable` writer - VUART2 TX DMA time out disable"]
pub type Vuart2txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG, Vuart2txdmatimeOutDisable>;
impl<'a, REG> Vuart2txdmatimeOutDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The TX DMA will not start to transmit until at least a full double word data in TX buffer or time out."]
    #[inline(always)]
    pub fn the_tx_dma_will_not_start_to_transmit_until_at_least_a_full_double_word_data_in_tx_buffer_or_time_out(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (Vuart2txdmatimeOutDisable :: TheTxDmaWillNotStartToTransmitUntilAtLeastAFullDoubleWordDataInTxBufferOrTimeOut)
    }
    #[doc = "Time out is disabled"]
    #[inline(always)]
    pub fn time_out_is_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Vuart2txdmatimeOutDisable::TimeOutIsDisabled)
    }
}
#[doc = "Field `Reserved01` reader - reserved(0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `Reserved01` writer - reserved(0)"]
pub type Reserved01W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VUART2TXBufHighBaseAddr` reader - VUART2 TX buffer high base address"]
pub type Vuart2txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `VUART2TXBufHighBaseAddr` writer - VUART2 TX buffer high base address"]
pub type Vuart2txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - VUART2 TX buffer size"]
    #[inline(always)]
    pub fn vuart2txbuf_size(&self) -> Vuart2txbufSizeR {
        Vuart2txbufSizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - VUART2 TX DMA time out disable"]
    #[inline(always)]
    pub fn vuart2txdmatime_out_disable(&self) -> Vuart2txdmatimeOutDisableR {
        Vuart2txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - VUART2 TX buffer high base address"]
    #[inline(always)]
    pub fn vuart2txbuf_high_base_addr(&self) -> Vuart2txbufHighBaseAddrR {
        Vuart2txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - VUART2 TX buffer size"]
    #[inline(always)]
    pub fn vuart2txbuf_size(&mut self) -> Vuart2txbufSizeW<Uartdma20cSpec> {
        Vuart2txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - VUART2 TX DMA time out disable"]
    #[inline(always)]
    pub fn vuart2txdmatime_out_disable(&mut self) -> Vuart2txdmatimeOutDisableW<Uartdma20cSpec> {
        Vuart2txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&mut self) -> Reserved01W<Uartdma20cSpec> {
        Reserved01W::new(self, 5)
    }
    #[doc = "Bits 8:10 - VUART2 TX buffer high base address"]
    #[inline(always)]
    pub fn vuart2txbuf_high_base_addr(&mut self) -> Vuart2txbufHighBaseAddrW<Uartdma20cSpec> {
        Vuart2txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "VUART2 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma20c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma20c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma20cSpec;
impl crate::RegisterSpec for Uartdma20cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma20c::R`](R) reader structure"]
impl crate::Readable for Uartdma20cSpec {}
#[doc = "`write(|w| ..)` method takes [`uartdma20c::W`](W) writer structure"]
impl crate::Writable for Uartdma20cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA20C to value 0"]
impl crate::Resettable for Uartdma20cSpec {}
