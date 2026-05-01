#[doc = "Register `UDMA1DC` reader"]
pub type R = crate::R<Udma1dcSpec>;
#[doc = "Register `UDMA1DC` writer"]
pub type W = crate::W<Udma1dcSpec>;
#[doc = "Field `VUART0RXBufSize` reader - VUART0 RX buffer size"]
pub type Vuart0rxbufSizeR = crate::FieldReader;
#[doc = "Field `VUART0RXBufSize` writer - VUART0 RX buffer size"]
pub type Vuart0rxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VUART0RXDMATimeOutDisable` reader - VUART0 RX DMA time out disable"]
pub type Vuart0rxdmatimeOutDisableR = crate::BitReader;
#[doc = "Field `VUART0RXDMATimeOutDisable` writer - VUART0 RX DMA time out disable"]
pub type Vuart0rxdmatimeOutDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART0RXDMAFullMode` reader - VUART0 RX DMA full mode"]
pub type Vuart0rxdmafullModeR = crate::BitReader;
#[doc = "Field `VUART0RXDMAFullMode` writer - VUART0 RX DMA full mode"]
pub type Vuart0rxdmafullModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VUART0RXBufHighBaseAddr` reader - VUART0 RX buffer high base address"]
pub type Vuart0rxbufHighBaseAddrR = crate::FieldReader;
#[doc = "Field `VUART0RXBufHighBaseAddr` writer - VUART0 RX buffer high base address"]
pub type Vuart0rxbufHighBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - VUART0 RX buffer size"]
    #[inline(always)]
    pub fn vuart0rxbuf_size(&self) -> Vuart0rxbufSizeR {
        Vuart0rxbufSizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - VUART0 RX DMA time out disable"]
    #[inline(always)]
    pub fn vuart0rxdmatime_out_disable(&self) -> Vuart0rxdmatimeOutDisableR {
        Vuart0rxdmatimeOutDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VUART0 RX DMA full mode"]
    #[inline(always)]
    pub fn vuart0rxdmafull_mode(&self) -> Vuart0rxdmafullModeR {
        Vuart0rxdmafullModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - VUART0 RX buffer high base address"]
    #[inline(always)]
    pub fn vuart0rxbuf_high_base_addr(&self) -> Vuart0rxbufHighBaseAddrR {
        Vuart0rxbufHighBaseAddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - VUART0 RX buffer size"]
    #[inline(always)]
    pub fn vuart0rxbuf_size(&mut self) -> Vuart0rxbufSizeW<Udma1dcSpec> {
        Vuart0rxbufSizeW::new(self, 0)
    }
    #[doc = "Bit 4 - VUART0 RX DMA time out disable"]
    #[inline(always)]
    pub fn vuart0rxdmatime_out_disable(&mut self) -> Vuart0rxdmatimeOutDisableW<Udma1dcSpec> {
        Vuart0rxdmatimeOutDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - VUART0 RX DMA full mode"]
    #[inline(always)]
    pub fn vuart0rxdmafull_mode(&mut self) -> Vuart0rxdmafullModeW<Udma1dcSpec> {
        Vuart0rxdmafullModeW::new(self, 5)
    }
    #[doc = "Bits 6:7 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma1dcSpec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bits 8:10 - VUART0 RX buffer high base address"]
    #[inline(always)]
    pub fn vuart0rxbuf_high_base_addr(&mut self) -> Vuart0rxbufHighBaseAddrW<Udma1dcSpec> {
        Vuart0rxbufHighBaseAddrW::new(self, 8)
    }
}
#[doc = "VUART0 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1dcSpec;
impl crate::RegisterSpec for Udma1dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1dc::R`](R) reader structure"]
impl crate::Readable for Udma1dcSpec {}
#[doc = "`write(|w| ..)` method takes [`udma1dc::W`](W) writer structure"]
impl crate::Writable for Udma1dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1DC to value 0"]
impl crate::Resettable for Udma1dcSpec {}
