#[doc = "Register `UDMA14C` reader"]
pub type R = crate::R<Udma14cSpec>;
#[doc = "Register `UDMA14C` writer"]
pub type W = crate::W<Udma14cSpec>;
#[doc = "Field `UART9TXBufSize` reader - UART9 TX buffer size"]
pub type Uart9txbufSizeR = crate::FieldReader;
#[doc = "Field `UART9TXBufSize` writer - UART9 TX buffer size"]
pub type Uart9txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART9TXDMATimeOutDisable` reader - UART9 TX DMA time out disable"]
pub type Uart9txdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART9TXDMATimeOutDisable` writer - UART9 TX DMA time out disable"]
pub type Uart9txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART9TXBufHighBaseAddr` reader - UART9 TX buffer high base address"]
pub type Uart9txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART9TXBufHighBaseAddr` writer - UART9 TX buffer high base address"]
pub type Uart9txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART9 TX buffer size"]
    #[inline(always)]
    pub fn uart9txbuf_size(&self) -> Uart9txbufSizeR {
        Uart9txbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART9 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart9txdmatime_out_disable(&self) -> Uart9txdmatimeOutDisableR {
        Uart9txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - UART9 TX buffer high base address"]
    #[inline(always)]
    pub fn uart9txbuf_high_base_addr(&self) -> Uart9txbufHighBaseAddrR {
        Uart9txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART9 TX buffer size"]
    #[inline(always)]
    pub fn uart9txbuf_size(&mut self) -> Uart9txbufSizeW<Udma14cSpec> {
        Uart9txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART9 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart9txdmatime_out_disable(&mut self) -> Uart9txdmatimeOutDisableW<Udma14cSpec> {
        Uart9txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma14cSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bits 8:10 - UART9 TX buffer high base address"]
    #[inline(always)]
    pub fn uart9txbuf_high_base_addr(&mut self) -> Uart9txbufHighBaseAddrW<Udma14cSpec> {
        Uart9txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART9 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma14c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma14c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma14cSpec;
impl crate::RegisterSpec for Udma14cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma14c::R`](R) reader structure"]
impl crate::Readable for Udma14cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma14c::W`](W) writer structure"]
impl crate::Writable for Udma14cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA14C to value 0"]
impl crate::Resettable for Udma14cSpec {}
