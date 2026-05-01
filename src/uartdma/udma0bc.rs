#[doc = "Register `UDMA0BC` reader"]
pub type R = crate::R<Udma0bcSpec>;
#[doc = "Register `UDMA0BC` writer"]
pub type W = crate::W<Udma0bcSpec>;
#[doc = "Field `UART3RXBufSize` reader - UART3 RX buffer size"]
pub type Uart3rxbufSizeR = crate::FieldReader;
#[doc = "Field `UART3RXBufSize` writer - UART3 RX buffer size"]
pub type Uart3rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART3RXDMATimeOutDisable` reader - UART3 RX DMA time out disable"]
pub type Uart3rxdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART3RXDMATimeOutDisable` writer - UART3 RX DMA time out disable"]
pub type Uart3rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3RXDMAFullMode` reader - UART3 RX DMA full mode"]
pub type Uart3rxdmafullModeR = crate::BitReader;
#[doc = "Field `UART3RXDMAFullMode` writer - UART3 RX DMA full mode"]
pub type Uart3rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART3RXBufHighBaseAddr` reader - UART3 RX buffer high base address"]
pub type Uart3rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART3RXBufHighBaseAddr` writer - UART3 RX buffer high base address"]
pub type Uart3rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART3 RX buffer size"]
    #[inline(always)]
    pub fn uart3rxbuf_size(&self) -> Uart3rxbufSizeR {
        Uart3rxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART3 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart3rxdmatime_out_disable(&self) -> Uart3rxdmatimeOutDisableR {
        Uart3rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART3 RX DMA full mode"]
    #[inline(always)]
    pub fn uart3rxdmafull_mode(&self) -> Uart3rxdmafullModeR {
        Uart3rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - UART3 RX buffer high base address"]
    #[inline(always)]
    pub fn uart3rxbuf_high_base_addr(&self) -> Uart3rxbufHighBaseAddrR {
        Uart3rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART3 RX buffer size"]
    #[inline(always)]
    pub fn uart3rxbuf_size(&mut self) -> Uart3rxbufSizeW<Udma0bcSpec> {
        Uart3rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART3 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart3rxdmatime_out_disable(&mut self) -> Uart3rxdmatimeOutDisableW<Udma0bcSpec> {
        Uart3rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - UART3 RX DMA full mode"]
    #[inline(always)]
    pub fn uart3rxdmafull_mode(&mut self) -> Uart3rxdmafullModeW<Udma0bcSpec> {
        Uart3rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma0bcSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bits 8:10 - UART3 RX buffer high base address"]
    #[inline(always)]
    pub fn uart3rxbuf_high_base_addr(&mut self) -> Uart3rxbufHighBaseAddrW<Udma0bcSpec> {
        Uart3rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART3 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0bcSpec;
impl crate::RegisterSpec for Udma0bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0bc::R`](R) reader structure"]
impl crate::Readable for Udma0bcSpec {}
#[doc = "`write(|w| ..)` method takes [`udma0bc::W`](W) writer structure"]
impl crate::Writable for Udma0bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0BC to value 0"]
impl crate::Resettable for Udma0bcSpec {}
