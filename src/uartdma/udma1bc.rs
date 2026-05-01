#[doc = "Register `UDMA1BC` reader"]
pub type R = crate::R<Udma1bcSpec>;
#[doc = "Register `UDMA1BC` writer"]
pub type W = crate::W<Udma1bcSpec>;
#[doc = "Field `UARTBMCRXBufSize` reader - UART-BMC RX buffer size"]
pub type UartbmcrxbufSizeR = crate::FieldReader;
#[doc = "Field `UARTBMCRXBufSize` writer - UART-BMC RX buffer size"]
pub type UartbmcrxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UARTBMCRXDMATimeOutDisable` reader - UART-BMC RX DMA time out disable"]
pub type UartbmcrxdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UARTBMCRXDMATimeOutDisable` writer - UART-BMC RX DMA time out disable"]
pub type UartbmcrxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UARTBMCRXDMAFullMode` reader - UART-BMC RX DMA full mode"]
pub type UartbmcrxdmafullModeR = crate::BitReader;
#[doc = "Field `UARTBMCRXDMAFullMode` writer - UART-BMC RX DMA full mode"]
pub type UartbmcrxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UARTBMCRXBufHighBaseAddr` reader - UART-BMC RX buffer high base address"]
pub type UartbmcrxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UARTBMCRXBufHighBaseAddr` writer - UART-BMC RX buffer high base address"]
pub type UartbmcrxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART-BMC RX buffer size"]
    #[inline(always)]
    pub fn uartbmcrxbuf_size(&self) -> UartbmcrxbufSizeR {
        UartbmcrxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART-BMC RX DMA time out disable"]
    #[inline(always)]
    pub fn uartbmcrxdmatime_out_disable(&self) -> UartbmcrxdmatimeOutDisableR {
        UartbmcrxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART-BMC RX DMA full mode"]
    #[inline(always)]
    pub fn uartbmcrxdmafull_mode(&self) -> UartbmcrxdmafullModeR {
        UartbmcrxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - UART-BMC RX buffer high base address"]
    #[inline(always)]
    pub fn uartbmcrxbuf_high_base_addr(&self) -> UartbmcrxbufHighBaseAddrR {
        UartbmcrxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART-BMC RX buffer size"]
    #[inline(always)]
    pub fn uartbmcrxbuf_size(&mut self) -> UartbmcrxbufSizeW<Udma1bcSpec> {
        UartbmcrxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART-BMC RX DMA time out disable"]
    #[inline(always)]
    pub fn uartbmcrxdmatime_out_disable(&mut self) -> UartbmcrxdmatimeOutDisableW<Udma1bcSpec> {
        UartbmcrxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - UART-BMC RX DMA full mode"]
    #[inline(always)]
    pub fn uartbmcrxdmafull_mode(&mut self) -> UartbmcrxdmafullModeW<Udma1bcSpec> {
        UartbmcrxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma1bcSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bits 8:10 - UART-BMC RX buffer high base address"]
    #[inline(always)]
    pub fn uartbmcrxbuf_high_base_addr(&mut self) -> UartbmcrxbufHighBaseAddrW<Udma1bcSpec> {
        UartbmcrxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART-BMC RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1bcSpec;
impl crate::RegisterSpec for Udma1bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1bc::R`](R) reader structure"]
impl crate::Readable for Udma1bcSpec {}
#[doc = "`write(|w| ..)` method takes [`udma1bc::W`](W) writer structure"]
impl crate::Writable for Udma1bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1BC to value 0"]
impl crate::Resettable for Udma1bcSpec {}
