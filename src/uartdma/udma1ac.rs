#[doc = "Register `UDMA1AC` reader"]
pub type R = crate::R<Udma1acSpec>;
#[doc = "Register `UDMA1AC` writer"]
pub type W = crate::W<Udma1acSpec>;
#[doc = "Field `UARTBMCTXBufSize` reader - UART-BMC TX buffer size"]
pub type UartbmctxbufSizeR = crate::FieldReader;
#[doc = "Field `UARTBMCTXBufSize` writer - UART-BMC TX buffer size"]
pub type UartbmctxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UARTBMCTXDMATimeOutDisable` reader - UART-BMC TX DMA time out disable"]
pub type UartbmctxdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UARTBMCTXDMATimeOutDisable` writer - UART-BMC TX DMA time out disable"]
pub type UartbmctxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UARTBMCTXBufHighBaseAddr` reader - UART-BMC TX buffer high base address"]
pub type UartbmctxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UARTBMCTXBufHighBaseAddr` writer - UART-BMC TX buffer high base address"]
pub type UartbmctxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART-BMC TX buffer size"]
    #[inline(always)]
    pub fn uartbmctxbuf_size(&self) -> UartbmctxbufSizeR {
        UartbmctxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART-BMC TX DMA time out disable"]
    #[inline(always)]
    pub fn uartbmctxdmatime_out_disable(&self) -> UartbmctxdmatimeOutDisableR {
        UartbmctxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - UART-BMC TX buffer high base address"]
    #[inline(always)]
    pub fn uartbmctxbuf_high_base_addr(&self) -> UartbmctxbufHighBaseAddrR {
        UartbmctxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART-BMC TX buffer size"]
    #[inline(always)]
    pub fn uartbmctxbuf_size(&mut self) -> UartbmctxbufSizeW<Udma1acSpec> {
        UartbmctxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART-BMC TX DMA time out disable"]
    #[inline(always)]
    pub fn uartbmctxdmatime_out_disable(&mut self) -> UartbmctxdmatimeOutDisableW<Udma1acSpec> {
        UartbmctxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma1acSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bits 8:10 - UART-BMC TX buffer high base address"]
    #[inline(always)]
    pub fn uartbmctxbuf_high_base_addr(&mut self) -> UartbmctxbufHighBaseAddrW<Udma1acSpec> {
        UartbmctxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART-BMC TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1acSpec;
impl crate::RegisterSpec for Udma1acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1ac::R`](R) reader structure"]
impl crate::Readable for Udma1acSpec {}
#[doc = "`write(|w| ..)` method takes [`udma1ac::W`](W) writer structure"]
impl crate::Writable for Udma1acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1AC to value 0"]
impl crate::Resettable for Udma1acSpec {}
