#[doc = "Register `UDMA11C` reader"]
pub type R = crate::R<Udma11cSpec>;
#[doc = "Register `UDMA11C` writer"]
pub type W = crate::W<Udma11cSpec>;
#[doc = "Field `UART7RXBufSize` reader - UART7 RX buffer size"]
pub type Uart7rxbufSizeR = crate::FieldReader;
#[doc = "Field `UART7RXBufSize` writer - UART7 RX buffer size"]
pub type Uart7rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART7RXDMATimeOutDisable` reader - UART7 RX DMA time out disable"]
pub type Uart7rxdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART7RXDMATimeOutDisable` writer - UART7 RX DMA time out disable"]
pub type Uart7rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART7RXDMAFullMode` reader - UART7 RX DMA full mode"]
pub type Uart7rxdmafullModeR = crate::BitReader;
#[doc = "Field `UART7RXDMAFullMode` writer - UART7 RX DMA full mode"]
pub type Uart7rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART7RXBufHighBaseAddr` reader - UART7 RX buffer high base address"]
pub type Uart7rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART7RXBufHighBaseAddr` writer - UART7 RX buffer high base address"]
pub type Uart7rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART7 RX buffer size"]
    #[inline(always)]
    pub fn uart7rxbuf_size(&self) -> Uart7rxbufSizeR {
        Uart7rxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART7 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart7rxdmatime_out_disable(&self) -> Uart7rxdmatimeOutDisableR {
        Uart7rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART7 RX DMA full mode"]
    #[inline(always)]
    pub fn uart7rxdmafull_mode(&self) -> Uart7rxdmafullModeR {
        Uart7rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - UART7 RX buffer high base address"]
    #[inline(always)]
    pub fn uart7rxbuf_high_base_addr(&self) -> Uart7rxbufHighBaseAddrR {
        Uart7rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART7 RX buffer size"]
    #[inline(always)]
    pub fn uart7rxbuf_size(&mut self) -> Uart7rxbufSizeW<Udma11cSpec> {
        Uart7rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART7 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart7rxdmatime_out_disable(&mut self) -> Uart7rxdmatimeOutDisableW<Udma11cSpec> {
        Uart7rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - UART7 RX DMA full mode"]
    #[inline(always)]
    pub fn uart7rxdmafull_mode(&mut self) -> Uart7rxdmafullModeW<Udma11cSpec> {
        Uart7rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma11cSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bits 8:10 - UART7 RX buffer high base address"]
    #[inline(always)]
    pub fn uart7rxbuf_high_base_addr(&mut self) -> Uart7rxbufHighBaseAddrW<Udma11cSpec> {
        Uart7rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART7 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma11c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma11c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma11cSpec;
impl crate::RegisterSpec for Udma11cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma11c::R`](R) reader structure"]
impl crate::Readable for Udma11cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma11c::W`](W) writer structure"]
impl crate::Writable for Udma11cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA11C to value 0"]
impl crate::Resettable for Udma11cSpec {}
