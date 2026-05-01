#[doc = "Register `UDMA09C` reader"]
pub type R = crate::R<Udma09cSpec>;
#[doc = "Register `UDMA09C` writer"]
pub type W = crate::W<Udma09cSpec>;
#[doc = "Field `UART2RXBufSize` reader - UART2 RX buffer size"]
pub type Uart2rxbufSizeR = crate::FieldReader;
#[doc = "Field `UART2RXBufSize` writer - UART2 RX buffer size"]
pub type Uart2rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART2RXDMATimeOutDisable` reader - UART2 RX DMA time out disable"]
pub type Uart2rxdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART2RXDMATimeOutDisable` writer - UART2 RX DMA time out disable"]
pub type Uart2rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2RXDMAFullMode` reader - UART2 RX DMA full mode"]
pub type Uart2rxdmafullModeR = crate::BitReader;
#[doc = "Field `UART2RXDMAFullMode` writer - UART2 RX DMA full mode"]
pub type Uart2rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART2RXBufHighBaseAddr` reader - UART2 RX buffer high base address"]
pub type Uart2rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART2RXBufHighBaseAddr` writer - UART2 RX buffer high base address"]
pub type Uart2rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART2 RX buffer size"]
    #[inline(always)]
    pub fn uart2rxbuf_size(&self) -> Uart2rxbufSizeR {
        Uart2rxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART2 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart2rxdmatime_out_disable(&self) -> Uart2rxdmatimeOutDisableR {
        Uart2rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART2 RX DMA full mode"]
    #[inline(always)]
    pub fn uart2rxdmafull_mode(&self) -> Uart2rxdmafullModeR {
        Uart2rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - UART2 RX buffer high base address"]
    #[inline(always)]
    pub fn uart2rxbuf_high_base_addr(&self) -> Uart2rxbufHighBaseAddrR {
        Uart2rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART2 RX buffer size"]
    #[inline(always)]
    pub fn uart2rxbuf_size(&mut self) -> Uart2rxbufSizeW<Udma09cSpec> {
        Uart2rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART2 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart2rxdmatime_out_disable(&mut self) -> Uart2rxdmatimeOutDisableW<Udma09cSpec> {
        Uart2rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - UART2 RX DMA full mode"]
    #[inline(always)]
    pub fn uart2rxdmafull_mode(&mut self) -> Uart2rxdmafullModeW<Udma09cSpec> {
        Uart2rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma09cSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bits 8:10 - UART2 RX buffer high base address"]
    #[inline(always)]
    pub fn uart2rxbuf_high_base_addr(&mut self) -> Uart2rxbufHighBaseAddrW<Udma09cSpec> {
        Uart2rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART2 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma09c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma09c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma09cSpec;
impl crate::RegisterSpec for Udma09cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma09c::R`](R) reader structure"]
impl crate::Readable for Udma09cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma09c::W`](W) writer structure"]
impl crate::Writable for Udma09cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA09C to value 0"]
impl crate::Resettable for Udma09cSpec {}
