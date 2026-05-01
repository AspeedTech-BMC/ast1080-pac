#[doc = "Register `UDMA1FC` reader"]
pub type R = crate::R<Udma1fcSpec>;
#[doc = "Register `UDMA1FC` writer"]
pub type W = crate::W<Udma1fcSpec>;
#[doc = "Field `VUART1RXBufSize` reader - VUART1 RX buffer size"]
pub type Vuart1rxbufSizeR = crate::FieldReader;
#[doc = "Field `VUART1RXBufSize` writer - VUART1 RX buffer size"]
pub type Vuart1rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VUART1RXDMATimeOutDisable` reader - VUART1 RX DMA time out disable"]
pub type Vuart1rxdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `VUART1RXDMATimeOutDisable` writer - VUART1 RX DMA time out disable"]
pub type Vuart1rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART1RXDMAFullMode` reader - VUART1 RX DMA full mode"]
pub type Vuart1rxdmafullModeR = crate::BitReader;
#[doc = "Field `VUART1RXDMAFullMode` writer - VUART1 RX DMA full mode"]
pub type Vuart1rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VUART1RXBufHighBaseAddr` reader - VUART1 RX buffer high base address"]
pub type Vuart1rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `VUART1RXBufHighBaseAddr` writer - VUART1 RX buffer high base address"]
pub type Vuart1rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - VUART1 RX buffer size"]
    #[inline(always)]
    pub fn vuart1rxbuf_size(&self) -> Vuart1rxbufSizeR {
        Vuart1rxbufSizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - VUART1 RX DMA time out disable"]
    #[inline(always)]
    pub fn vuart1rxdmatime_out_disable(&self) -> Vuart1rxdmatimeOutDisableR {
        Vuart1rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VUART1 RX DMA full mode"]
    #[inline(always)]
    pub fn vuart1rxdmafull_mode(&self) -> Vuart1rxdmafullModeR {
        Vuart1rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - VUART1 RX buffer high base address"]
    #[inline(always)]
    pub fn vuart1rxbuf_high_base_addr(&self) -> Vuart1rxbufHighBaseAddrR {
        Vuart1rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - VUART1 RX buffer size"]
    #[inline(always)]
    pub fn vuart1rxbuf_size(&mut self) -> Vuart1rxbufSizeW<Udma1fcSpec> {
        Vuart1rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - VUART1 RX DMA time out disable"]
    #[inline(always)]
    pub fn vuart1rxdmatime_out_disable(&mut self) -> Vuart1rxdmatimeOutDisableW<Udma1fcSpec> {
        Vuart1rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - VUART1 RX DMA full mode"]
    #[inline(always)]
    pub fn vuart1rxdmafull_mode(&mut self) -> Vuart1rxdmafullModeW<Udma1fcSpec> {
        Vuart1rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma1fcSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bits 8:10 - VUART1 RX buffer high base address"]
    #[inline(always)]
    pub fn vuart1rxbuf_high_base_addr(&mut self) -> Vuart1rxbufHighBaseAddrW<Udma1fcSpec> {
        Vuart1rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "VUART1 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1fcSpec;
impl crate::RegisterSpec for Udma1fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1fc::R`](R) reader structure"]
impl crate::Readable for Udma1fcSpec {}
#[doc = "`write(|w| ..)` method takes [`udma1fc::W`](W) writer structure"]
impl crate::Writable for Udma1fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1FC to value 0"]
impl crate::Resettable for Udma1fcSpec {}
