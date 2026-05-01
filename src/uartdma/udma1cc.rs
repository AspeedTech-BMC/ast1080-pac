#[doc = "Register `UDMA1CC` reader"]
pub type R = crate::R<Udma1ccSpec>;
#[doc = "Register `UDMA1CC` writer"]
pub type W = crate::W<Udma1ccSpec>;
#[doc = "Field `VUART0TXBufSize` reader - VUART0 TX buffer size"]
pub type Vuart0txbufSizeR = crate::FieldReader;
#[doc = "Field `VUART0TXBufSize` writer - VUART0 TX buffer size"]
pub type Vuart0txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VUART0TXDMATimeOutDisable` reader - VUART0 TX DMA time out disable"]
pub type Vuart0txdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `VUART0TXDMATimeOutDisable` writer - VUART0 TX DMA time out disable"]
pub type Vuart0txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VUART0TXBufHighBaseAddr` reader - VUART0 TX buffer high base address"]
pub type Vuart0txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `VUART0TXBufHighBaseAddr` writer - VUART0 TX buffer high base address"]
pub type Vuart0txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - VUART0 TX buffer size"]
    #[inline(always)]
    pub fn vuart0txbuf_size(&self) -> Vuart0txbufSizeR {
        Vuart0txbufSizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - VUART0 TX DMA time out disable"]
    #[inline(always)]
    pub fn vuart0txdmatime_out_disable(&self) -> Vuart0txdmatimeOutDisableR {
        Vuart0txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - VUART0 TX buffer high base address"]
    #[inline(always)]
    pub fn vuart0txbuf_high_base_addr(&self) -> Vuart0txbufHighBaseAddrR {
        Vuart0txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - VUART0 TX buffer size"]
    #[inline(always)]
    pub fn vuart0txbuf_size(&mut self) -> Vuart0txbufSizeW<Udma1ccSpec> {
        Vuart0txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - VUART0 TX DMA time out disable"]
    #[inline(always)]
    pub fn vuart0txdmatime_out_disable(&mut self) -> Vuart0txdmatimeOutDisableW<Udma1ccSpec> {
        Vuart0txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma1ccSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bits 8:10 - VUART0 TX buffer high base address"]
    #[inline(always)]
    pub fn vuart0txbuf_high_base_addr(&mut self) -> Vuart0txbufHighBaseAddrW<Udma1ccSpec> {
        Vuart0txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "VUART0 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1ccSpec;
impl crate::RegisterSpec for Udma1ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1cc::R`](R) reader structure"]
impl crate::Readable for Udma1ccSpec {}
#[doc = "`write(|w| ..)` method takes [`udma1cc::W`](W) writer structure"]
impl crate::Writable for Udma1ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1CC to value 0"]
impl crate::Resettable for Udma1ccSpec {}
