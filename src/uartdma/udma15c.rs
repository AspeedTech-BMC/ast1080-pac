#[doc = "Register `UDMA15C` reader"]
pub type R = crate::R<Udma15cSpec>;
#[doc = "Register `UDMA15C` writer"]
pub type W = crate::W<Udma15cSpec>;
#[doc = "Field `UART9RXBufSize` reader - UART9 RX buffer size"]
pub type Uart9rxbufSizeR = crate::FieldReader;
#[doc = "Field `UART9RXBufSize` writer - UART9 RX buffer size"]
pub type Uart9rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART9RXDMATimeOutDisable` reader - UART9 RX DMA time out disable"]
pub type Uart9rxdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART9RXDMATimeOutDisable` writer - UART9 RX DMA time out disable"]
pub type Uart9rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART9RXDMAFullMode` reader - UART9 RX DMA full mode"]
pub type Uart9rxdmafullModeR = crate::BitReader;
#[doc = "Field `UART9RXDMAFullMode` writer - UART9 RX DMA full mode"]
pub type Uart9rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART9RXBufHighBaseAddr` reader - UART9 RX buffer high base address"]
pub type Uart9rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART9RXBufHighBaseAddr` writer - UART9 RX buffer high base address"]
pub type Uart9rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART9 RX buffer size"]
    #[inline(always)]
    pub fn uart9rxbuf_size(&self) -> Uart9rxbufSizeR {
        Uart9rxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART9 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart9rxdmatime_out_disable(&self) -> Uart9rxdmatimeOutDisableR {
        Uart9rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART9 RX DMA full mode"]
    #[inline(always)]
    pub fn uart9rxdmafull_mode(&self) -> Uart9rxdmafullModeR {
        Uart9rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - UART9 RX buffer high base address"]
    #[inline(always)]
    pub fn uart9rxbuf_high_base_addr(&self) -> Uart9rxbufHighBaseAddrR {
        Uart9rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART9 RX buffer size"]
    #[inline(always)]
    pub fn uart9rxbuf_size(&mut self) -> Uart9rxbufSizeW<Udma15cSpec> {
        Uart9rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART9 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart9rxdmatime_out_disable(&mut self) -> Uart9rxdmatimeOutDisableW<Udma15cSpec> {
        Uart9rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - UART9 RX DMA full mode"]
    #[inline(always)]
    pub fn uart9rxdmafull_mode(&mut self) -> Uart9rxdmafullModeW<Udma15cSpec> {
        Uart9rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma15cSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bits 8:10 - UART9 RX buffer high base address"]
    #[inline(always)]
    pub fn uart9rxbuf_high_base_addr(&mut self) -> Uart9rxbufHighBaseAddrW<Udma15cSpec> {
        Uart9rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART9 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma15c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma15c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma15cSpec;
impl crate::RegisterSpec for Udma15cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma15c::R`](R) reader structure"]
impl crate::Readable for Udma15cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma15c::W`](W) writer structure"]
impl crate::Writable for Udma15cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA15C to value 0"]
impl crate::Resettable for Udma15cSpec {}
