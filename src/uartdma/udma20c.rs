#[doc = "Register `UDMA20C` reader"]
pub type R = crate::R<Udma20cSpec>;
#[doc = "Register `UDMA20C` writer"]
pub type W = crate::W<Udma20cSpec>;
#[doc = "Field `VUART2TXBufSize` reader - VUART2 TX buffer size"]
pub type Vuart2txbufSizeR = crate::FieldReader;
#[doc = "Field `VUART2TXBufSize` writer - VUART2 TX buffer size"]
pub type Vuart2txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VUART2TXDMATimeOutDisable` reader - VUART2 TX DMA time out disable"]
pub type Vuart2txdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `VUART2TXDMATimeOutDisable` writer - VUART2 TX DMA time out disable"]
pub type Vuart2txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VUART2TXBufHighBaseAddr` reader - VUART2 TX buffer high base address"]
pub type Vuart2txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `VUART2TXBufHighBaseAddr` writer - VUART2 TX buffer high base address"]
pub type Vuart2txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - VUART2 TX buffer size"]
    #[inline(always)]
    pub fn vuart2txbuf_size(&self) -> Vuart2txbufSizeR {
        Vuart2txbufSizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - VUART2 TX DMA time out disable"]
    #[inline(always)]
    pub fn vuart2txdmatime_out_disable(&self) -> Vuart2txdmatimeOutDisableR {
        Vuart2txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - VUART2 TX buffer high base address"]
    #[inline(always)]
    pub fn vuart2txbuf_high_base_addr(&self) -> Vuart2txbufHighBaseAddrR {
        Vuart2txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - VUART2 TX buffer size"]
    #[inline(always)]
    pub fn vuart2txbuf_size(&mut self) -> Vuart2txbufSizeW<Udma20cSpec> {
        Vuart2txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - VUART2 TX DMA time out disable"]
    #[inline(always)]
    pub fn vuart2txdmatime_out_disable(&mut self) -> Vuart2txdmatimeOutDisableW<Udma20cSpec> {
        Vuart2txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma20cSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bits 8:10 - VUART2 TX buffer high base address"]
    #[inline(always)]
    pub fn vuart2txbuf_high_base_addr(&mut self) -> Vuart2txbufHighBaseAddrW<Udma20cSpec> {
        Vuart2txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "VUART2 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma20c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma20c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma20cSpec;
impl crate::RegisterSpec for Udma20cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma20c::R`](R) reader structure"]
impl crate::Readable for Udma20cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma20c::W`](W) writer structure"]
impl crate::Writable for Udma20cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA20C to value 0"]
impl crate::Resettable for Udma20cSpec {}
