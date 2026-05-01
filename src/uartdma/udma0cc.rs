#[doc = "Register `UDMA0CC` reader"]
pub type R = crate::R<Udma0ccSpec>;
#[doc = "Register `UDMA0CC` writer"]
pub type W = crate::W<Udma0ccSpec>;
#[doc = "Field `UART5TXBufSize` reader - UART5 TX buffer size"]
pub type Uart5txbufSizeR = crate::FieldReader;
#[doc = "Field `UART5TXBufSize` writer - UART5 TX buffer size"]
pub type Uart5txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART5TXDMATimeOutDisable` reader - UART5 TX DMA time out disable"]
pub type Uart5txdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART5TXDMATimeOutDisable` writer - UART5 TX DMA time out disable"]
pub type Uart5txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART5TXBufHighBaseAddr` reader - UART5 TX buffer high base address"]
pub type Uart5txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART5TXBufHighBaseAddr` writer - UART5 TX buffer high base address"]
pub type Uart5txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART5 TX buffer size"]
    #[inline(always)]
    pub fn uart5txbuf_size(&self) -> Uart5txbufSizeR {
        Uart5txbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART5 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart5txdmatime_out_disable(&self) -> Uart5txdmatimeOutDisableR {
        Uart5txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - UART5 TX buffer high base address"]
    #[inline(always)]
    pub fn uart5txbuf_high_base_addr(&self) -> Uart5txbufHighBaseAddrR {
        Uart5txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART5 TX buffer size"]
    #[inline(always)]
    pub fn uart5txbuf_size(&mut self) -> Uart5txbufSizeW<Udma0ccSpec> {
        Uart5txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART5 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart5txdmatime_out_disable(&mut self) -> Uart5txdmatimeOutDisableW<Udma0ccSpec> {
        Uart5txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma0ccSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bits 8:10 - UART5 TX buffer high base address"]
    #[inline(always)]
    pub fn uart5txbuf_high_base_addr(&mut self) -> Uart5txbufHighBaseAddrW<Udma0ccSpec> {
        Uart5txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART5 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0ccSpec;
impl crate::RegisterSpec for Udma0ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0cc::R`](R) reader structure"]
impl crate::Readable for Udma0ccSpec {}
#[doc = "`write(|w| ..)` method takes [`udma0cc::W`](W) writer structure"]
impl crate::Writable for Udma0ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0CC to value 0"]
impl crate::Resettable for Udma0ccSpec {}
