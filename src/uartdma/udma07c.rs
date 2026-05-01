#[doc = "Register `UDMA07C` reader"]
pub type R = crate::R<Udma07cSpec>;
#[doc = "Register `UDMA07C` writer"]
pub type W = crate::W<Udma07cSpec>;
#[doc = "Field `UART1RXBufSize` reader - UART1 RX buffer size"]
pub type Uart1rxbufSizeR = crate::FieldReader;
#[doc = "Field `UART1RXBufSize` writer - UART1 RX buffer size"]
pub type Uart1rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART1RXDMATimeOutDisable` reader - UART1 RX DMA time out disable"]
pub type Uart1rxdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART1RXDMATimeOutDisable` writer - UART1 RX DMA time out disable"]
pub type Uart1rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1RXDMAFullMode` reader - UART1 RX DMA full mode"]
pub type Uart1rxdmafullModeR = crate::BitReader;
#[doc = "Field `UART1RXDMAFullMode` writer - UART1 RX DMA full mode"]
pub type Uart1rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART1RXBufHighBaseAddr` reader - UART1 RX buffer high base address"]
pub type Uart1rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART1RXBufHighBaseAddr` writer - UART1 RX buffer high base address"]
pub type Uart1rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART1 RX buffer size"]
    #[inline(always)]
    pub fn uart1rxbuf_size(&self) -> Uart1rxbufSizeR {
        Uart1rxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART1 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart1rxdmatime_out_disable(&self) -> Uart1rxdmatimeOutDisableR {
        Uart1rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART1 RX DMA full mode"]
    #[inline(always)]
    pub fn uart1rxdmafull_mode(&self) -> Uart1rxdmafullModeR {
        Uart1rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - UART1 RX buffer high base address"]
    #[inline(always)]
    pub fn uart1rxbuf_high_base_addr(&self) -> Uart1rxbufHighBaseAddrR {
        Uart1rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART1 RX buffer size"]
    #[inline(always)]
    pub fn uart1rxbuf_size(&mut self) -> Uart1rxbufSizeW<Udma07cSpec> {
        Uart1rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART1 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart1rxdmatime_out_disable(&mut self) -> Uart1rxdmatimeOutDisableW<Udma07cSpec> {
        Uart1rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - UART1 RX DMA full mode"]
    #[inline(always)]
    pub fn uart1rxdmafull_mode(&mut self) -> Uart1rxdmafullModeW<Udma07cSpec> {
        Uart1rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma07cSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bits 8:10 - UART1 RX buffer high base address"]
    #[inline(always)]
    pub fn uart1rxbuf_high_base_addr(&mut self) -> Uart1rxbufHighBaseAddrW<Udma07cSpec> {
        Uart1rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART1 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma07c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma07c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma07cSpec;
impl crate::RegisterSpec for Udma07cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma07c::R`](R) reader structure"]
impl crate::Readable for Udma07cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma07c::W`](W) writer structure"]
impl crate::Writable for Udma07cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA07C to value 0"]
impl crate::Resettable for Udma07cSpec {}
