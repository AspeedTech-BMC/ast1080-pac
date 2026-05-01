#[doc = "Register `UDMA1EC` reader"]
pub type R = crate::R<Udma1ecSpec>;
#[doc = "Register `UDMA1EC` writer"]
pub type W = crate::W<Udma1ecSpec>;
#[doc = "Field `VUART1TXBufSize` reader - VUART1 TX buffer size"]
pub type Vuart1txbufSizeR = crate::FieldReader;
#[doc = "Field `VUART1TXBufSize` writer - VUART1 TX buffer size"]
pub type Vuart1txbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VUART1TXDMATimeOutDisable` reader - VUART1 TX DMA time out disable"]
pub type Vuart1txdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `VUART1TXDMATimeOutDisable` writer - VUART1 TX DMA time out disable"]
pub type Vuart1txdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VUART1TXBufHighBaseAddr` reader - VUART1 TX buffer high base address"]
pub type Vuart1txbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `VUART1TXBufHighBaseAddr` writer - VUART1 TX buffer high base address"]
pub type Vuart1txbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - VUART1 TX buffer size"]
    #[inline(always)]
    pub fn vuart1txbuf_size(&self) -> Vuart1txbufSizeR {
        Vuart1txbufSizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - VUART1 TX DMA time out disable"]
    #[inline(always)]
    pub fn vuart1txdmatime_out_disable(&self) -> Vuart1txdmatimeOutDisableR {
        Vuart1txdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - VUART1 TX buffer high base address"]
    #[inline(always)]
    pub fn vuart1txbuf_high_base_addr(&self) -> Vuart1txbufHighBaseAddrR {
        Vuart1txbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - VUART1 TX buffer size"]
    #[inline(always)]
    pub fn vuart1txbuf_size(&mut self) -> Vuart1txbufSizeW<Udma1ecSpec> {
        Vuart1txbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - VUART1 TX DMA time out disable"]
    #[inline(always)]
    pub fn vuart1txdmatime_out_disable(&mut self) -> Vuart1txdmatimeOutDisableW<Udma1ecSpec> {
        Vuart1txdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bits 5:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma1ecSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bits 8:10 - VUART1 TX buffer high base address"]
    #[inline(always)]
    pub fn vuart1txbuf_high_base_addr(&mut self) -> Vuart1txbufHighBaseAddrW<Udma1ecSpec> {
        Vuart1txbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "VUART1 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1ecSpec;
impl crate::RegisterSpec for Udma1ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1ec::R`](R) reader structure"]
impl crate::Readable for Udma1ecSpec {}
#[doc = "`write(|w| ..)` method takes [`udma1ec::W`](W) writer structure"]
impl crate::Writable for Udma1ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1EC to value 0"]
impl crate::Resettable for Udma1ecSpec {}
