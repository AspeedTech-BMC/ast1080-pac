#[doc = "Register `UDMA06C` reader"]
pub type R = crate::R<Udma06cSpec>;
#[doc = "Register `UDMA06C` writer"]
pub type W = crate::W<Udma06cSpec>;
#[doc = "Field `UART1TXBufSize` reader - UART1 TX buffer size"]
pub type Uart1txbufSizeR = crate::FieldReader;
#[doc = "Field `UART1TXBufSize` writer - UART1 TX buffer size"]
pub type Uart1txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART1TXDMATimeOutDisable` reader - UART1 TX DMA time out disable"]
pub type Uart1txdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART1TXDMATimeOutDisable` writer - UART1 TX DMA time out disable"]
pub type Uart1txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART1TXBufHighBaseAddr` reader - UART1 TX buffer high base address"]
pub type Uart1txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART1TXBufHighBaseAddr` writer - UART1 TX buffer high base address"]
pub type Uart1txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART1 TX buffer size"]
    #[inline(always)]
    pub fn uart1txbuf_size(&self) -> Uart1txbufSizeR {
        Uart1txbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART1 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart1txdmatime_out_disable(&self) -> Uart1txdmatimeOutDisableR {
        Uart1txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - UART1 TX buffer high base address"]
    #[inline(always)]
    pub fn uart1txbuf_high_base_addr(&self) -> Uart1txbufHighBaseAddrR {
        Uart1txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART1 TX buffer size"]
    #[inline(always)]
    pub fn uart1txbuf_size(&mut self) -> Uart1txbufSizeW<Udma06cSpec> {
        Uart1txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART1 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart1txdmatime_out_disable(&mut self) -> Uart1txdmatimeOutDisableW<Udma06cSpec> {
        Uart1txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma06cSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bits 8:10 - UART1 TX buffer high base address"]
    #[inline(always)]
    pub fn uart1txbuf_high_base_addr(&mut self) -> Uart1txbufHighBaseAddrW<Udma06cSpec> {
        Uart1txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART1 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma06c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma06c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma06cSpec;
impl crate::RegisterSpec for Udma06cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma06c::R`](R) reader structure"]
impl crate::Readable for Udma06cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma06c::W`](W) writer structure"]
impl crate::Writable for Udma06cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA06C to value 0"]
impl crate::Resettable for Udma06cSpec {}
