#[doc = "Register `UDMA19C` reader"]
pub type R = crate::R<Udma19cSpec>;
#[doc = "Register `UDMA19C` writer"]
pub type W = crate::W<Udma19cSpec>;
#[doc = "Field `UART11RXBufSize` reader - UART11 RX buffer size"]
pub type Uart11rxbufSizeR = crate::FieldReader;
#[doc = "Field `UART11RXBufSize` writer - UART11 RX buffer size"]
pub type Uart11rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART11RXDMATimeOutDisable` reader - UART11 RX DMA time out disable"]
pub type Uart11rxdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART11RXDMATimeOutDisable` writer - UART11 RX DMA time out disable"]
pub type Uart11rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART11RXDMAFullMode` reader - UART11 RX DMA full mode"]
pub type Uart11rxdmafullModeR = crate::BitReader;
#[doc = "Field `UART11RXDMAFullMode` writer - UART11 RX DMA full mode"]
pub type Uart11rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART11RXBufHighBaseAddr` reader - UART11 RX buffer high base address"]
pub type Uart11rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART11RXBufHighBaseAddr` writer - UART11 RX buffer high base address"]
pub type Uart11rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART11 RX buffer size"]
    #[inline(always)]
    pub fn uart11rxbuf_size(&self) -> Uart11rxbufSizeR {
        Uart11rxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART11 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart11rxdmatime_out_disable(&self) -> Uart11rxdmatimeOutDisableR {
        Uart11rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART11 RX DMA full mode"]
    #[inline(always)]
    pub fn uart11rxdmafull_mode(&self) -> Uart11rxdmafullModeR {
        Uart11rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - UART11 RX buffer high base address"]
    #[inline(always)]
    pub fn uart11rxbuf_high_base_addr(&self) -> Uart11rxbufHighBaseAddrR {
        Uart11rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART11 RX buffer size"]
    #[inline(always)]
    pub fn uart11rxbuf_size(&mut self) -> Uart11rxbufSizeW<Udma19cSpec> {
        Uart11rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART11 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart11rxdmatime_out_disable(&mut self) -> Uart11rxdmatimeOutDisableW<Udma19cSpec> {
        Uart11rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - UART11 RX DMA full mode"]
    #[inline(always)]
    pub fn uart11rxdmafull_mode(&mut self) -> Uart11rxdmafullModeW<Udma19cSpec> {
        Uart11rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma19cSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bits 8:10 - UART11 RX buffer high base address"]
    #[inline(always)]
    pub fn uart11rxbuf_high_base_addr(&mut self) -> Uart11rxbufHighBaseAddrW<Udma19cSpec> {
        Uart11rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART11 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma19c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma19c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma19cSpec;
impl crate::RegisterSpec for Udma19cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma19c::R`](R) reader structure"]
impl crate::Readable for Udma19cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma19c::W`](W) writer structure"]
impl crate::Writable for Udma19cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA19C to value 0"]
impl crate::Resettable for Udma19cSpec {}
