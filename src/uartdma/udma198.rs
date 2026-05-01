#[doc = "Register `UDMA198` reader"]
pub type R = crate::R<Udma198Spec>;
#[doc = "Register `UDMA198` writer"]
pub type W = crate::W<Udma198Spec>;
#[doc = "Field `UART11RXBufBaseAddr` reader - UART11 RX buffer base address"]
pub type Uart11rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART11RXBufBaseAddr` writer - UART11 RX buffer base address"]
pub type Uart11rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART11 RX buffer base address"]
    #[inline(always)]
    pub fn uart11rxbuf_base_addr(&self) -> Uart11rxbufBaseAddrR {
        Uart11rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART11 RX buffer base address"]
    #[inline(always)]
    pub fn uart11rxbuf_base_addr(&mut self) -> Uart11rxbufBaseAddrW<Udma198Spec> {
        Uart11rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART11 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma198::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma198::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma198Spec;
impl crate::RegisterSpec for Udma198Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma198::R`](R) reader structure"]
impl crate::Readable for Udma198Spec {}
#[doc = "`write(|w| ..)` method takes [`udma198::W`](W) writer structure"]
impl crate::Writable for Udma198Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA198 to value 0"]
impl crate::Resettable for Udma198Spec {}
