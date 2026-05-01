#[doc = "Register `UDMA13C` reader"]
pub type R = crate::R<Udma13cSpec>;
#[doc = "Register `UDMA13C` writer"]
pub type W = crate::W<Udma13cSpec>;
#[doc = "Field `UART8RXBufSize` reader - UART8 RX buffer size"]
pub type Uart8rxbufSizeR = crate::FieldReader;
#[doc = "Field `UART8RXBufSize` writer - UART8 RX buffer size"]
pub type Uart8rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART8RXDMATimeOutDisable` reader - UART8 RX DMA time out disable"]
pub type Uart8rxdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART8RXDMATimeOutDisable` writer - UART8 RX DMA time out disable"]
pub type Uart8rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART8RXDMAFullMode` reader - UART8 RX DMA full mode"]
pub type Uart8rxdmafullModeR = crate::BitReader;
#[doc = "Field `UART8RXDMAFullMode` writer - UART8 RX DMA full mode"]
pub type Uart8rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART8RXBufHighBaseAddr` reader - UART8 RX buffer high base address"]
pub type Uart8rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART8RXBufHighBaseAddr` writer - UART8 RX buffer high base address"]
pub type Uart8rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART8 RX buffer size"]
    #[inline(always)]
    pub fn uart8rxbuf_size(&self) -> Uart8rxbufSizeR {
        Uart8rxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART8 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart8rxdmatime_out_disable(&self) -> Uart8rxdmatimeOutDisableR {
        Uart8rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART8 RX DMA full mode"]
    #[inline(always)]
    pub fn uart8rxdmafull_mode(&self) -> Uart8rxdmafullModeR {
        Uart8rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - UART8 RX buffer high base address"]
    #[inline(always)]
    pub fn uart8rxbuf_high_base_addr(&self) -> Uart8rxbufHighBaseAddrR {
        Uart8rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART8 RX buffer size"]
    #[inline(always)]
    pub fn uart8rxbuf_size(&mut self) -> Uart8rxbufSizeW<Udma13cSpec> {
        Uart8rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART8 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart8rxdmatime_out_disable(&mut self) -> Uart8rxdmatimeOutDisableW<Udma13cSpec> {
        Uart8rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - UART8 RX DMA full mode"]
    #[inline(always)]
    pub fn uart8rxdmafull_mode(&mut self) -> Uart8rxdmafullModeW<Udma13cSpec> {
        Uart8rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma13cSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bits 8:10 - UART8 RX buffer high base address"]
    #[inline(always)]
    pub fn uart8rxbuf_high_base_addr(&mut self) -> Uart8rxbufHighBaseAddrW<Udma13cSpec> {
        Uart8rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART8 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma13c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma13c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma13cSpec;
impl crate::RegisterSpec for Udma13cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma13c::R`](R) reader structure"]
impl crate::Readable for Udma13cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma13c::W`](W) writer structure"]
impl crate::Writable for Udma13cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA13C to value 0"]
impl crate::Resettable for Udma13cSpec {}
