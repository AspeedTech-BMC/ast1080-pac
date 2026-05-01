#[doc = "Register `UDMA22C` reader"]
pub type R = crate::R<Udma22cSpec>;
#[doc = "Register `UDMA22C` writer"]
pub type W = crate::W<Udma22cSpec>;
#[doc = "Field `VUART3TXBufSize` reader - VUART3 TX buffer size"]
pub type Vuart3txbufSizeR = crate::FieldReader;
#[doc = "Field `VUART3TXBufSize` writer - VUART3 TX buffer size"]
pub type Vuart3txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VUART3TXDMATimeOutDisable` reader - VUART3 TX DMA time out disable"]
pub type Vuart3txdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `VUART3TXDMATimeOutDisable` writer - VUART3 TX DMA time out disable"]
pub type Vuart3txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VUART3TXBufHighBaseAddr` reader - VUART3 TX buffer high base address"]
pub type Vuart3txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `VUART3TXBufHighBaseAddr` writer - VUART3 TX buffer high base address"]
pub type Vuart3txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - VUART3 TX buffer size"]
    #[inline(always)]
    pub fn vuart3txbuf_size(&self) -> Vuart3txbufSizeR {
        Vuart3txbufSizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - VUART3 TX DMA time out disable"]
    #[inline(always)]
    pub fn vuart3txdmatime_out_disable(&self) -> Vuart3txdmatimeOutDisableR {
        Vuart3txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - VUART3 TX buffer high base address"]
    #[inline(always)]
    pub fn vuart3txbuf_high_base_addr(&self) -> Vuart3txbufHighBaseAddrR {
        Vuart3txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - VUART3 TX buffer size"]
    #[inline(always)]
    pub fn vuart3txbuf_size(&mut self) -> Vuart3txbufSizeW<Udma22cSpec> {
        Vuart3txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - VUART3 TX DMA time out disable"]
    #[inline(always)]
    pub fn vuart3txdmatime_out_disable(&mut self) -> Vuart3txdmatimeOutDisableW<Udma22cSpec> {
        Vuart3txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma22cSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bits 8:10 - VUART3 TX buffer high base address"]
    #[inline(always)]
    pub fn vuart3txbuf_high_base_addr(&mut self) -> Vuart3txbufHighBaseAddrW<Udma22cSpec> {
        Vuart3txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "VUART3 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma22c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma22c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma22cSpec;
impl crate::RegisterSpec for Udma22cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma22c::R`](R) reader structure"]
impl crate::Readable for Udma22cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma22c::W`](W) writer structure"]
impl crate::Writable for Udma22cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA22C to value 0"]
impl crate::Resettable for Udma22cSpec {}
