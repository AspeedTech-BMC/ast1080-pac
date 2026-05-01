#[doc = "Register `UDMA16C` reader"]
pub type R = crate::R<Udma16cSpec>;
#[doc = "Register `UDMA16C` writer"]
pub type W = crate::W<Udma16cSpec>;
#[doc = "Field `UART10TXBufSize` reader - UART10 TX buffer size"]
pub type Uart10txbufSizeR = crate::FieldReader;
#[doc = "Field `UART10TXBufSize` writer - UART10 TX buffer size"]
pub type Uart10txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART10TXDMATimeOutDisable` reader - UART10 TX DMA time out disable"]
pub type Uart10txdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART10TXDMATimeOutDisable` writer - UART10 TX DMA time out disable"]
pub type Uart10txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART10TXBufHighBaseAddr` reader - UART10 TX buffer high base address"]
pub type Uart10txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART10TXBufHighBaseAddr` writer - UART10 TX buffer high base address"]
pub type Uart10txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART10 TX buffer size"]
    #[inline(always)]
    pub fn uart10txbuf_size(&self) -> Uart10txbufSizeR {
        Uart10txbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART10 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart10txdmatime_out_disable(&self) -> Uart10txdmatimeOutDisableR {
        Uart10txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - UART10 TX buffer high base address"]
    #[inline(always)]
    pub fn uart10txbuf_high_base_addr(&self) -> Uart10txbufHighBaseAddrR {
        Uart10txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART10 TX buffer size"]
    #[inline(always)]
    pub fn uart10txbuf_size(&mut self) -> Uart10txbufSizeW<Udma16cSpec> {
        Uart10txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART10 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart10txdmatime_out_disable(&mut self) -> Uart10txdmatimeOutDisableW<Udma16cSpec> {
        Uart10txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma16cSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bits 8:10 - UART10 TX buffer high base address"]
    #[inline(always)]
    pub fn uart10txbuf_high_base_addr(&mut self) -> Uart10txbufHighBaseAddrW<Udma16cSpec> {
        Uart10txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART10 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma16c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma16c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma16cSpec;
impl crate::RegisterSpec for Udma16cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma16c::R`](R) reader structure"]
impl crate::Readable for Udma16cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma16c::W`](W) writer structure"]
impl crate::Writable for Udma16cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA16C to value 0"]
impl crate::Resettable for Udma16cSpec {}
