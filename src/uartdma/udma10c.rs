#[doc = "Register `UDMA10C` reader"]
pub type R = crate::R<Udma10cSpec>;
#[doc = "Register `UDMA10C` writer"]
pub type W = crate::W<Udma10cSpec>;
#[doc = "Field `UART7TXBufSize` reader - UART7 TX buffer size"]
pub type Uart7txbufSizeR = crate::FieldReader;
#[doc = "Field `UART7TXBufSize` writer - UART7 TX buffer size"]
pub type Uart7txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART7TXDMATimeOutDisable` reader - UART7 TX DMA time out disable"]
pub type Uart7txdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART7TXDMATimeOutDisable` writer - UART7 TX DMA time out disable"]
pub type Uart7txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART7TXBufHighBaseAddr` reader - UART7 TX buffer high base address"]
pub type Uart7txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART7TXBufHighBaseAddr` writer - UART7 TX buffer high base address"]
pub type Uart7txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART7 TX buffer size"]
    #[inline(always)]
    pub fn uart7txbuf_size(&self) -> Uart7txbufSizeR {
        Uart7txbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART7 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart7txdmatime_out_disable(&self) -> Uart7txdmatimeOutDisableR {
        Uart7txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - UART7 TX buffer high base address"]
    #[inline(always)]
    pub fn uart7txbuf_high_base_addr(&self) -> Uart7txbufHighBaseAddrR {
        Uart7txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART7 TX buffer size"]
    #[inline(always)]
    pub fn uart7txbuf_size(&mut self) -> Uart7txbufSizeW<Udma10cSpec> {
        Uart7txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART7 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart7txdmatime_out_disable(&mut self) -> Uart7txdmatimeOutDisableW<Udma10cSpec> {
        Uart7txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma10cSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bits 8:10 - UART7 TX buffer high base address"]
    #[inline(always)]
    pub fn uart7txbuf_high_base_addr(&mut self) -> Uart7txbufHighBaseAddrW<Udma10cSpec> {
        Uart7txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART7 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma10c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma10c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma10cSpec;
impl crate::RegisterSpec for Udma10cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma10c::R`](R) reader structure"]
impl crate::Readable for Udma10cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma10c::W`](W) writer structure"]
impl crate::Writable for Udma10cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA10C to value 0"]
impl crate::Resettable for Udma10cSpec {}
