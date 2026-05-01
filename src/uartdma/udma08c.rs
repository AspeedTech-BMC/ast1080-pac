#[doc = "Register `UDMA08C` reader"]
pub type R = crate::R<Udma08cSpec>;
#[doc = "Register `UDMA08C` writer"]
pub type W = crate::W<Udma08cSpec>;
#[doc = "Field `UART2TXBufSize` reader - UART2 TX buffer size"]
pub type Uart2txbufSizeR = crate::FieldReader;
#[doc = "Field `UART2TXBufSize` writer - UART2 TX buffer size"]
pub type Uart2txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `UART2TXDMATimeOutDisable` reader - UART2 TX DMA time out disable"]
pub type Uart2txdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `UART2TXDMATimeOutDisable` writer - UART2 TX DMA time out disable"]
pub type Uart2txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UART2TXBufHighBaseAddr` reader - UART2 TX buffer high base address"]
pub type Uart2txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `UART2TXBufHighBaseAddr` writer - UART2 TX buffer high base address"]
pub type Uart2txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - UART2 TX buffer size"]
    #[inline(always)]
    pub fn uart2txbuf_size(&self) -> Uart2txbufSizeR {
        Uart2txbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - UART2 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart2txdmatime_out_disable(&self) -> Uart2txdmatimeOutDisableR {
        Uart2txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - UART2 TX buffer high base address"]
    #[inline(always)]
    pub fn uart2txbuf_high_base_addr(&self) -> Uart2txbufHighBaseAddrR {
        Uart2txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART2 TX buffer size"]
    #[inline(always)]
    pub fn uart2txbuf_size(&mut self) -> Uart2txbufSizeW<Udma08cSpec> {
        Uart2txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - UART2 TX DMA time out disable"]
    #[inline(always)]
    pub fn uart2txdmatime_out_disable(&mut self) -> Uart2txdmatimeOutDisableW<Udma08cSpec> {
        Uart2txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma08cSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bits 8:10 - UART2 TX buffer high base address"]
    #[inline(always)]
    pub fn uart2txbuf_high_base_addr(&mut self) -> Uart2txbufHighBaseAddrW<Udma08cSpec> {
        Uart2txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "UART2 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma08c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma08c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma08cSpec;
impl crate::RegisterSpec for Udma08cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma08c::R`](R) reader structure"]
impl crate::Readable for Udma08cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma08c::W`](W) writer structure"]
impl crate::Writable for Udma08cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA08C to value 0"]
impl crate::Resettable for Udma08cSpec {}
