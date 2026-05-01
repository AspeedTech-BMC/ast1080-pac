#[doc = "Register `UDMA04C` reader"]
pub type R = crate::R<Udma04cSpec>;
#[doc = "Register `UDMA04C` writer"]
pub type W = crate::W<Udma04cSpec>;
#[doc = "Field `UART0TXBufSize` reader - UART0 TX buffer size"]
pub type Uart0txbufSizeR = crate::FieldReader;
#[doc = "Field `UART0TXBufSize` writer - UART0 TX buffer size"]
pub type Uart0txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART0TXDMATimeOutDisable` reader - UART0 TX DMA time out disable"]
pub type Uart0txdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART0TXDMATimeOutDisable` writer - UART0 TX DMA time out disable"]
pub type Uart0txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART0TXBufHighBaseAddr` reader - UART0 TX buffer high base address"]
pub type Uart0txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART0TXBufHighBaseAddr` writer - UART0 TX buffer high base address"]
pub type Uart0txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART0 TX buffer size"]
    #[inline(always)]
    pub fn uart0txbuf_size(&self) -> Uart0txbufSizeR {
        Uart0txbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART0 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart0txdmatime_out_disable(&self) -> Uart0txdmatimeOutDisableR {
        Uart0txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - UART0 TX buffer high base address"]
    #[inline(always)]
    pub fn uart0txbuf_high_base_addr(&self) -> Uart0txbufHighBaseAddrR {
        Uart0txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART0 TX buffer size"]
    #[inline(always)]
    pub fn uart0txbuf_size(&mut self) -> Uart0txbufSizeW<Udma04cSpec> {
        Uart0txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART0 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart0txdmatime_out_disable(&mut self) -> Uart0txdmatimeOutDisableW<Udma04cSpec> {
        Uart0txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma04cSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bits 8:10 - UART0 TX buffer high base address"]
    #[inline(always)]
    pub fn uart0txbuf_high_base_addr(&mut self) -> Uart0txbufHighBaseAddrW<Udma04cSpec> {
        Uart0txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART0 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma04c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma04c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma04cSpec;
impl crate::RegisterSpec for Udma04cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma04c::R`](R) reader structure"]
impl crate::Readable for Udma04cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma04c::W`](W) writer structure"]
impl crate::Writable for Udma04cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA04C to value 0"]
impl crate::Resettable for Udma04cSpec {}
