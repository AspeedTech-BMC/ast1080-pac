#[doc = "Register `UDMA0EC` reader"]
pub type R = crate::R<Udma0ecSpec>;
#[doc = "Register `UDMA0EC` writer"]
pub type W = crate::W<Udma0ecSpec>;
#[doc = "Field `UART6TXBufSize` reader - UART6 TX buffer size"]
pub type Uart6txbufSizeR = crate::FieldReader;
#[doc = "Field `UART6TXBufSize` writer - UART6 TX buffer size"]
pub type Uart6txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART6TXDMATimeOutDisable` reader - UART6 TX DMA time out disable"]
pub type Uart6txdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART6TXDMATimeOutDisable` writer - UART6 TX DMA time out disable"]
pub type Uart6txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART6TXBufHighBaseAddr` reader - UART6 TX buffer high base address"]
pub type Uart6txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART6TXBufHighBaseAddr` writer - UART6 TX buffer high base address"]
pub type Uart6txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART6 TX buffer size"]
    #[inline(always)]
    pub fn uart6txbuf_size(&self) -> Uart6txbufSizeR {
        Uart6txbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART6 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart6txdmatime_out_disable(&self) -> Uart6txdmatimeOutDisableR {
        Uart6txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - UART6 TX buffer high base address"]
    #[inline(always)]
    pub fn uart6txbuf_high_base_addr(&self) -> Uart6txbufHighBaseAddrR {
        Uart6txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART6 TX buffer size"]
    #[inline(always)]
    pub fn uart6txbuf_size(&mut self) -> Uart6txbufSizeW<Udma0ecSpec> {
        Uart6txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART6 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart6txdmatime_out_disable(&mut self) -> Uart6txdmatimeOutDisableW<Udma0ecSpec> {
        Uart6txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma0ecSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bits 8:10 - UART6 TX buffer high base address"]
    #[inline(always)]
    pub fn uart6txbuf_high_base_addr(&mut self) -> Uart6txbufHighBaseAddrW<Udma0ecSpec> {
        Uart6txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART6 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0ecSpec;
impl crate::RegisterSpec for Udma0ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0ec::R`](R) reader structure"]
impl crate::Readable for Udma0ecSpec {}
#[doc = "`write(|w| ..)` method takes [`udma0ec::W`](W) writer structure"]
impl crate::Writable for Udma0ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0EC to value 0"]
impl crate::Resettable for Udma0ecSpec {}
