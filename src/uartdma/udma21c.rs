#[doc = "Register `UDMA21C` reader"]
pub type R = crate::R<Udma21cSpec>;
#[doc = "Register `UDMA21C` writer"]
pub type W = crate::W<Udma21cSpec>;
#[doc = "Field `VUART2RXBufSize` reader - VUART2 RX buffer size"]
pub type Vuart2rxbufSizeR = crate::FieldReader;
#[doc = "Field `VUART2RXBufSize` writer - VUART2 RX buffer size"]
pub type Vuart2rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VUART2RXDMATimeOutDisable` reader - VUART2 RX DMA time out disable"]
pub type Vuart2rxdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `VUART2RXDMATimeOutDisable` writer - VUART2 RX DMA time out disable"]
pub type Vuart2rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART2RXDMAFullMode` reader - VUART2 RX DMA full mode"]
pub type Vuart2rxdmafullModeR = crate::BitReader;
#[doc = "Field `VUART2RXDMAFullMode` writer - VUART2 RX DMA full mode"]
pub type Vuart2rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VUART2RXBufHighBaseAddr` reader - VUART2 RX buffer high base address"]
pub type Vuart2rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `VUART2RXBufHighBaseAddr` writer - VUART2 RX buffer high base address"]
pub type Vuart2rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - VUART2 RX buffer size"]
    #[inline(always)]
    pub fn vuart2rxbuf_size(&self) -> Vuart2rxbufSizeR {
        Vuart2rxbufSizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - VUART2 RX DMA time out disable"]
    #[inline(always)]
    pub fn vuart2rxdmatime_out_disable(&self) -> Vuart2rxdmatimeOutDisableR {
        Vuart2rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VUART2 RX DMA full mode"]
    #[inline(always)]
    pub fn vuart2rxdmafull_mode(&self) -> Vuart2rxdmafullModeR {
        Vuart2rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - VUART2 RX buffer high base address"]
    #[inline(always)]
    pub fn vuart2rxbuf_high_base_addr(&self) -> Vuart2rxbufHighBaseAddrR {
        Vuart2rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - VUART2 RX buffer size"]
    #[inline(always)]
    pub fn vuart2rxbuf_size(&mut self) -> Vuart2rxbufSizeW<Udma21cSpec> {
        Vuart2rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - VUART2 RX DMA time out disable"]
    #[inline(always)]
    pub fn vuart2rxdmatime_out_disable(&mut self) -> Vuart2rxdmatimeOutDisableW<Udma21cSpec> {
        Vuart2rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - VUART2 RX DMA full mode"]
    #[inline(always)]
    pub fn vuart2rxdmafull_mode(&mut self) -> Vuart2rxdmafullModeW<Udma21cSpec> {
        Vuart2rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma21cSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bits 8:10 - VUART2 RX buffer high base address"]
    #[inline(always)]
    pub fn vuart2rxbuf_high_base_addr(&mut self) -> Vuart2rxbufHighBaseAddrW<Udma21cSpec> {
        Vuart2rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "VUART2 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma21c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma21c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma21cSpec;
impl crate::RegisterSpec for Udma21cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma21c::R`](R) reader structure"]
impl crate::Readable for Udma21cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma21c::W`](W) writer structure"]
impl crate::Writable for Udma21cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA21C to value 0"]
impl crate::Resettable for Udma21cSpec {}
