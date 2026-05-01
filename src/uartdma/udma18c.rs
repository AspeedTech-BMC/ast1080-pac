#[doc = "Register `UDMA18C` reader"]
pub type R = crate::R<Udma18cSpec>;
#[doc = "Register `UDMA18C` writer"]
pub type W = crate::W<Udma18cSpec>;
#[doc = "Field `UART11TXBufSize` reader - UART11 TX buffer size"]
pub type Uart11txbufSizeR = crate::FieldReader;
#[doc = "Field `UART11TXBufSize` writer - UART11 TX buffer size"]
pub type Uart11txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART11TXDMATimeOutDisable` reader - UART11 TX DMA time out disable"]
pub type Uart11txdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART11TXDMATimeOutDisable` writer - UART11 TX DMA time out disable"]
pub type Uart11txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART11TXBufHighBaseAddr` reader - UART11 TX buffer high base address"]
pub type Uart11txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART11TXBufHighBaseAddr` writer - UART11 TX buffer high base address"]
pub type Uart11txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART11 TX buffer size"]
    #[inline(always)]
    pub fn uart11txbuf_size(&self) -> Uart11txbufSizeR {
        Uart11txbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART11 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart11txdmatime_out_disable(&self) -> Uart11txdmatimeOutDisableR {
        Uart11txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - UART11 TX buffer high base address"]
    #[inline(always)]
    pub fn uart11txbuf_high_base_addr(&self) -> Uart11txbufHighBaseAddrR {
        Uart11txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART11 TX buffer size"]
    #[inline(always)]
    pub fn uart11txbuf_size(&mut self) -> Uart11txbufSizeW<Udma18cSpec> {
        Uart11txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART11 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart11txdmatime_out_disable(&mut self) -> Uart11txdmatimeOutDisableW<Udma18cSpec> {
        Uart11txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma18cSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bits 8:10 - UART11 TX buffer high base address"]
    #[inline(always)]
    pub fn uart11txbuf_high_base_addr(&mut self) -> Uart11txbufHighBaseAddrW<Udma18cSpec> {
        Uart11txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART11 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma18c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma18c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma18cSpec;
impl crate::RegisterSpec for Udma18cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma18c::R`](R) reader structure"]
impl crate::Readable for Udma18cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma18c::W`](W) writer structure"]
impl crate::Writable for Udma18cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA18C to value 0"]
impl crate::Resettable for Udma18cSpec {}
