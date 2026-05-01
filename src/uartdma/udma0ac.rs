#[doc = "Register `UDMA0AC` reader"]
pub type R = crate::R<Udma0acSpec>;
#[doc = "Register `UDMA0AC` writer"]
pub type W = crate::W<Udma0acSpec>;
#[doc = "Field `UART3TXBufSize` reader - UART3 TX buffer size"]
pub type Uart3txbufSizeR = crate::FieldReader;
#[doc = "Field `UART3TXBufSize` writer - UART3 TX buffer size"]
pub type Uart3txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART3TXDMATimeOutDisable` reader - UART3 TX DMA time out disable"]
pub type Uart3txdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART3TXDMATimeOutDisable` writer - UART3 TX DMA time out disable"]
pub type Uart3txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART3TXBufHighBaseAddr` reader - UART3 TX buffer high base address"]
pub type Uart3txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART3TXBufHighBaseAddr` writer - UART3 TX buffer high base address"]
pub type Uart3txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART3 TX buffer size"]
    #[inline(always)]
    pub fn uart3txbuf_size(&self) -> Uart3txbufSizeR {
        Uart3txbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART3 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart3txdmatime_out_disable(&self) -> Uart3txdmatimeOutDisableR {
        Uart3txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - UART3 TX buffer high base address"]
    #[inline(always)]
    pub fn uart3txbuf_high_base_addr(&self) -> Uart3txbufHighBaseAddrR {
        Uart3txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART3 TX buffer size"]
    #[inline(always)]
    pub fn uart3txbuf_size(&mut self) -> Uart3txbufSizeW<Udma0acSpec> {
        Uart3txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART3 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart3txdmatime_out_disable(&mut self) -> Uart3txdmatimeOutDisableW<Udma0acSpec> {
        Uart3txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma0acSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bits 8:10 - UART3 TX buffer high base address"]
    #[inline(always)]
    pub fn uart3txbuf_high_base_addr(&mut self) -> Uart3txbufHighBaseAddrW<Udma0acSpec> {
        Uart3txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART3 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0acSpec;
impl crate::RegisterSpec for Udma0acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0ac::R`](R) reader structure"]
impl crate::Readable for Udma0acSpec {}
#[doc = "`write(|w| ..)` method takes [`udma0ac::W`](W) writer structure"]
impl crate::Writable for Udma0acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0AC to value 0"]
impl crate::Resettable for Udma0acSpec {}
