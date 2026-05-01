#[doc = "Register `UDMA0DC` reader"]
pub type R = crate::R<Udma0dcSpec>;
#[doc = "Register `UDMA0DC` writer"]
pub type W = crate::W<Udma0dcSpec>;
#[doc = "Field `UART5RXBufSize` reader - UART5 RX buffer size"]
pub type Uart5rxbufSizeR = crate::FieldReader;
#[doc = "Field `UART5RXBufSize` writer - UART5 RX buffer size"]
pub type Uart5rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART5RXDMATimeOutDisable` reader - UART5 RX DMA time out disable"]
pub type Uart5rxdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART5RXDMATimeOutDisable` writer - UART5 RX DMA time out disable"]
pub type Uart5rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5RXDMAFullMode` reader - UART5 RX DMA full mode"]
pub type Uart5rxdmafullModeR = crate::BitReader;
#[doc = "Field `UART5RXDMAFullMode` writer - UART5 RX DMA full mode"]
pub type Uart5rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART5RXBufHighBaseAddr` reader - UART5 RX buffer high base address"]
pub type Uart5rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART5RXBufHighBaseAddr` writer - UART5 RX buffer high base address"]
pub type Uart5rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART5 RX buffer size"]
    #[inline(always)]
    pub fn uart5rxbuf_size(&self) -> Uart5rxbufSizeR {
        Uart5rxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART5 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart5rxdmatime_out_disable(&self) -> Uart5rxdmatimeOutDisableR {
        Uart5rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART5 RX DMA full mode"]
    #[inline(always)]
    pub fn uart5rxdmafull_mode(&self) -> Uart5rxdmafullModeR {
        Uart5rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - UART5 RX buffer high base address"]
    #[inline(always)]
    pub fn uart5rxbuf_high_base_addr(&self) -> Uart5rxbufHighBaseAddrR {
        Uart5rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART5 RX buffer size"]
    #[inline(always)]
    pub fn uart5rxbuf_size(&mut self) -> Uart5rxbufSizeW<Udma0dcSpec> {
        Uart5rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART5 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart5rxdmatime_out_disable(&mut self) -> Uart5rxdmatimeOutDisableW<Udma0dcSpec> {
        Uart5rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - UART5 RX DMA full mode"]
    #[inline(always)]
    pub fn uart5rxdmafull_mode(&mut self) -> Uart5rxdmafullModeW<Udma0dcSpec> {
        Uart5rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma0dcSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bits 8:10 - UART5 RX buffer high base address"]
    #[inline(always)]
    pub fn uart5rxbuf_high_base_addr(&mut self) -> Uart5rxbufHighBaseAddrW<Udma0dcSpec> {
        Uart5rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART5 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0dcSpec;
impl crate::RegisterSpec for Udma0dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0dc::R`](R) reader structure"]
impl crate::Readable for Udma0dcSpec {}
#[doc = "`write(|w| ..)` method takes [`udma0dc::W`](W) writer structure"]
impl crate::Writable for Udma0dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0DC to value 0"]
impl crate::Resettable for Udma0dcSpec {}
