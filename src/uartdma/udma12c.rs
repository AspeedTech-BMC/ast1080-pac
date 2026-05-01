#[doc = "Register `UDMA12C` reader"]
pub type R = crate::R<Udma12cSpec>;
#[doc = "Register `UDMA12C` writer"]
pub type W = crate::W<Udma12cSpec>;
#[doc = "Field `UART8TXBufSize` reader - UART8 TX buffer size"]
pub type Uart8txbufSizeR = crate::FieldReader;
#[doc = "Field `UART8TXBufSize` writer - UART8 TX buffer size"]
pub type Uart8txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART8TXDMATimeOutDisable` reader - UART8 TX DMA time out disable"]
pub type Uart8txdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART8TXDMATimeOutDisable` writer - UART8 TX DMA time out disable"]
pub type Uart8txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART8TXBufHighBaseAddr` reader - UART8 TX buffer high base address"]
pub type Uart8txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART8TXBufHighBaseAddr` writer - UART8 TX buffer high base address"]
pub type Uart8txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART8 TX buffer size"]
    #[inline(always)]
    pub fn uart8txbuf_size(&self) -> Uart8txbufSizeR {
        Uart8txbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART8 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart8txdmatime_out_disable(&self) -> Uart8txdmatimeOutDisableR {
        Uart8txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - UART8 TX buffer high base address"]
    #[inline(always)]
    pub fn uart8txbuf_high_base_addr(&self) -> Uart8txbufHighBaseAddrR {
        Uart8txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART8 TX buffer size"]
    #[inline(always)]
    pub fn uart8txbuf_size(&mut self) -> Uart8txbufSizeW<Udma12cSpec> {
        Uart8txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART8 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart8txdmatime_out_disable(&mut self) -> Uart8txdmatimeOutDisableW<Udma12cSpec> {
        Uart8txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma12cSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bits 8:10 - UART8 TX buffer high base address"]
    #[inline(always)]
    pub fn uart8txbuf_high_base_addr(&mut self) -> Uart8txbufHighBaseAddrW<Udma12cSpec> {
        Uart8txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART8 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma12c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma12c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma12cSpec;
impl crate::RegisterSpec for Udma12cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma12c::R`](R) reader structure"]
impl crate::Readable for Udma12cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma12c::W`](W) writer structure"]
impl crate::Writable for Udma12cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA12C to value 0"]
impl crate::Resettable for Udma12cSpec {}
