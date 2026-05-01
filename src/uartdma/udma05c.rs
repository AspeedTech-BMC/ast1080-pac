#[doc = "Register `UDMA05C` reader"]
pub type R = crate::R<Udma05cSpec>;
#[doc = "Register `UDMA05C` writer"]
pub type W = crate::W<Udma05cSpec>;
#[doc = "Field `UART0RXBufSize` reader - UART0 RX buffer size"]
pub type Uart0rxbufSizeR = crate::FieldReader;
#[doc = "Field `UART0RXBufSize` writer - UART0 RX buffer size"]
pub type Uart0rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART0RXDMATimeOutDisable` reader - UART0 RX DMA time out disable"]
pub type Uart0rxdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART0RXDMATimeOutDisable` writer - UART0 RX DMA time out disable"]
pub type Uart0rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0RXDMAFullMode` reader - UART0 RX DMA full mode"]
pub type Uart0rxdmafullModeR = crate::BitReader;
#[doc = "Field `UART0RXDMAFullMode` writer - UART0 RX DMA full mode"]
pub type Uart0rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART0RXBufHighBaseAddr` reader - UART0 RX buffer high base address"]
pub type Uart0rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART0RXBufHighBaseAddr` writer - UART0 RX buffer high base address"]
pub type Uart0rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART0 RX buffer size"]
    #[inline(always)]
    pub fn uart0rxbuf_size(&self) -> Uart0rxbufSizeR {
        Uart0rxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART0 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart0rxdmatime_out_disable(&self) -> Uart0rxdmatimeOutDisableR {
        Uart0rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART0 RX DMA full mode"]
    #[inline(always)]
    pub fn uart0rxdmafull_mode(&self) -> Uart0rxdmafullModeR {
        Uart0rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - UART0 RX buffer high base address"]
    #[inline(always)]
    pub fn uart0rxbuf_high_base_addr(&self) -> Uart0rxbufHighBaseAddrR {
        Uart0rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART0 RX buffer size"]
    #[inline(always)]
    pub fn uart0rxbuf_size(&mut self) -> Uart0rxbufSizeW<Udma05cSpec> {
        Uart0rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART0 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart0rxdmatime_out_disable(&mut self) -> Uart0rxdmatimeOutDisableW<Udma05cSpec> {
        Uart0rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - UART0 RX DMA full mode"]
    #[inline(always)]
    pub fn uart0rxdmafull_mode(&mut self) -> Uart0rxdmafullModeW<Udma05cSpec> {
        Uart0rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma05cSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bits 8:10 - UART0 RX buffer high base address"]
    #[inline(always)]
    pub fn uart0rxbuf_high_base_addr(&mut self) -> Uart0rxbufHighBaseAddrW<Udma05cSpec> {
        Uart0rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART0 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma05c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma05c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma05cSpec;
impl crate::RegisterSpec for Udma05cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma05c::R`](R) reader structure"]
impl crate::Readable for Udma05cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma05c::W`](W) writer structure"]
impl crate::Writable for Udma05cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA05C to value 0"]
impl crate::Resettable for Udma05cSpec {}
