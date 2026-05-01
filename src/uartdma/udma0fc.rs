#[doc = "Register `UDMA0FC` reader"]
pub type R = crate::R<Udma0fcSpec>;
#[doc = "Register `UDMA0FC` writer"]
pub type W = crate::W<Udma0fcSpec>;
#[doc = "Field `UART6RXBufSize` reader - UART6 RX buffer size"]
pub type Uart6rxbufSizeR = crate::FieldReader;
#[doc = "Field `UART6RXBufSize` writer - UART6 RX buffer size"]
pub type Uart6rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART6RXDMATimeOutDisable` reader - UART6 RX DMA time out disable"]
pub type Uart6rxdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART6RXDMATimeOutDisable` writer - UART6 RX DMA time out disable"]
pub type Uart6rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART6RXDMAFullMode` reader - UART6 RX DMA full mode"]
pub type Uart6rxdmafullModeR = crate::BitReader;
#[doc = "Field `UART6RXDMAFullMode` writer - UART6 RX DMA full mode"]
pub type Uart6rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART6RXBufHighBaseAddr` reader - UART6 RX buffer high base address"]
pub type Uart6rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART6RXBufHighBaseAddr` writer - UART6 RX buffer high base address"]
pub type Uart6rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART6 RX buffer size"]
    #[inline(always)]
    pub fn uart6rxbuf_size(&self) -> Uart6rxbufSizeR {
        Uart6rxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART6 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart6rxdmatime_out_disable(&self) -> Uart6rxdmatimeOutDisableR {
        Uart6rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART6 RX DMA full mode"]
    #[inline(always)]
    pub fn uart6rxdmafull_mode(&self) -> Uart6rxdmafullModeR {
        Uart6rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - UART6 RX buffer high base address"]
    #[inline(always)]
    pub fn uart6rxbuf_high_base_addr(&self) -> Uart6rxbufHighBaseAddrR {
        Uart6rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART6 RX buffer size"]
    #[inline(always)]
    pub fn uart6rxbuf_size(&mut self) -> Uart6rxbufSizeW<Udma0fcSpec> {
        Uart6rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART6 RX DMA time out disable"]
    #[inline(always)]
    pub fn uart6rxdmatime_out_disable(&mut self) -> Uart6rxdmatimeOutDisableW<Udma0fcSpec> {
        Uart6rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - UART6 RX DMA full mode"]
    #[inline(always)]
    pub fn uart6rxdmafull_mode(&mut self) -> Uart6rxdmafullModeW<Udma0fcSpec> {
        Uart6rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma0fcSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bits 8:10 - UART6 RX buffer high base address"]
    #[inline(always)]
    pub fn uart6rxbuf_high_base_addr(&mut self) -> Uart6rxbufHighBaseAddrW<Udma0fcSpec> {
        Uart6rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART6 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0fcSpec;
impl crate::RegisterSpec for Udma0fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0fc::R`](R) reader structure"]
impl crate::Readable for Udma0fcSpec {}
#[doc = "`write(|w| ..)` method takes [`udma0fc::W`](W) writer structure"]
impl crate::Writable for Udma0fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0FC to value 0"]
impl crate::Resettable for Udma0fcSpec {}
