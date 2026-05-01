#[doc = "Register `UDMA17C` reader"]
pub type R = crate::R<Udma17cSpec>;
#[doc = "Register `UDMA17C` writer"]
pub type W = crate::W<Udma17cSpec>;
#[doc = "Field `UART10RXBufSize` reader - UART10 RX buffer size"]
pub type Uart10rxbufSizeR = crate::FieldReader;
#[doc = "Field `UART10RXBufSize` writer - UART10 RX buffer size"]
pub type Uart10rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART10RXDMATimeOutDisable` reader - UART10 RX DMA time out disable"]
pub type Uart10rxdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART10RXDMATimeOutDisable` writer - UART10 RX DMA time out disable"]
pub type Uart10rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART10RXDMAFullMode` reader - UART10 RX DMA full mode"]
pub type Uart10rxdmafullModeR = crate::BitReader;
#[doc = "Field `UART10RXDMAFullMode` writer - UART10 RX DMA full mode"]
pub type Uart10rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART10RXBufHighBaseAddr` reader - UART10 RX buffer high base address"]
pub type Uart10rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART10RXBufHighBaseAddr` writer - UART10 RX buffer high base address"]
pub type Uart10rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART10 RX buffer size"]
    #[inline(always)]
    pub fn uart10rxbuf_size(&self) -> Uart10rxbufSizeR {
        Uart10rxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART10 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart10rxdmatime_out_disable(&self) -> Uart10rxdmatimeOutDisableR {
        Uart10rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART10 RX DMA full mode"]
    #[inline(always)]
    pub fn uart10rxdmafull_mode(&self) -> Uart10rxdmafullModeR {
        Uart10rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - UART10 RX buffer high base address"]
    #[inline(always)]
    pub fn uart10rxbuf_high_base_addr(&self) -> Uart10rxbufHighBaseAddrR {
        Uart10rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART10 RX buffer size"]
    #[inline(always)]
    pub fn uart10rxbuf_size(&mut self) -> Uart10rxbufSizeW<Udma17cSpec> {
        Uart10rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART10 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart10rxdmatime_out_disable(&mut self) -> Uart10rxdmatimeOutDisableW<Udma17cSpec> {
        Uart10rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - UART10 RX DMA full mode"]
    #[inline(always)]
    pub fn uart10rxdmafull_mode(&mut self) -> Uart10rxdmafullModeW<Udma17cSpec> {
        Uart10rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma17cSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bits 8:10 - UART10 RX buffer high base address"]
    #[inline(always)]
    pub fn uart10rxbuf_high_base_addr(&mut self) -> Uart10rxbufHighBaseAddrW<Udma17cSpec> {
        Uart10rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART10 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma17c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma17c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma17cSpec;
impl crate::RegisterSpec for Udma17cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma17c::R`](R) reader structure"]
impl crate::Readable for Udma17cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma17c::W`](W) writer structure"]
impl crate::Writable for Udma17cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA17C to value 0"]
impl crate::Resettable for Udma17cSpec {}
