#[doc = "Register `UDMA23C` reader"]
pub type R = crate::R<Udma23cSpec>;
#[doc = "Register `UDMA23C` writer"]
pub type W = crate::W<Udma23cSpec>;
#[doc = "Field `VUART3RXBufSize` reader - VUART3 RX buffer size"]
pub type Vuart3rxbufSizeR = crate::FieldReader;
#[doc = "Field `VUART3RXBufSize` writer - VUART3 RX buffer size"]
pub type Vuart3rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VUART3RXDMATimeOutDisable` reader - VUART3 RX DMA time out disable"]
pub type Vuart3rxdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `VUART3RXDMATimeOutDisable` writer - VUART3 RX DMA time out disable"]
pub type Vuart3rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART3RXDMAFullMode` reader - VUART3 RX DMA full mode"]
pub type Vuart3rxdmafullModeR = crate::BitReader;
#[doc = "Field `VUART3RXDMAFullMode` writer - VUART3 RX DMA full mode"]
pub type Vuart3rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VUART3RXBufHighBaseAddr` reader - VUART3 RX buffer high base address"]
pub type Vuart3rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `VUART3RXBufHighBaseAddr` writer - VUART3 RX buffer high base address"]
pub type Vuart3rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - VUART3 RX buffer size"]
    #[inline(always)]
    pub fn vuart3rxbuf_size(&self) -> Vuart3rxbufSizeR {
        Vuart3rxbufSizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - VUART3 RX DMA time out disable"]
    #[inline(always)]
    pub fn vuart3rxdmatime_out_disable(&self) -> Vuart3rxdmatimeOutDisableR {
        Vuart3rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VUART3 RX DMA full mode"]
    #[inline(always)]
    pub fn vuart3rxdmafull_mode(&self) -> Vuart3rxdmafullModeR {
        Vuart3rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - VUART3 RX buffer high base address"]
    #[inline(always)]
    pub fn vuart3rxbuf_high_base_addr(&self) -> Vuart3rxbufHighBaseAddrR {
        Vuart3rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - VUART3 RX buffer size"]
    #[inline(always)]
    pub fn vuart3rxbuf_size(&mut self) -> Vuart3rxbufSizeW<Udma23cSpec> {
        Vuart3rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - VUART3 RX DMA time out disable"]
    #[inline(always)]
    pub fn vuart3rxdmatime_out_disable(&mut self) -> Vuart3rxdmatimeOutDisableW<Udma23cSpec> {
        Vuart3rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - VUART3 RX DMA full mode"]
    #[inline(always)]
    pub fn vuart3rxdmafull_mode(&mut self) -> Vuart3rxdmafullModeW<Udma23cSpec> {
        Vuart3rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma23cSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bits 8:10 - VUART3 RX buffer high base address"]
    #[inline(always)]
    pub fn vuart3rxbuf_high_base_addr(&mut self) -> Vuart3rxbufHighBaseAddrW<Udma23cSpec> {
        Vuart3rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "VUART3 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma23c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma23c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma23cSpec;
impl crate::RegisterSpec for Udma23cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma23c::R`](R) reader structure"]
impl crate::Readable for Udma23cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma23c::W`](W) writer structure"]
impl crate::Writable for Udma23cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA23C to value 0"]
impl crate::Resettable for Udma23cSpec {}
