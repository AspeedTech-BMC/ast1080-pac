#[doc = "Register `UDMA188` reader"]
pub type R = crate::R<Udma188Spec>;
#[doc = "Register `UDMA188` writer"]
pub type W = crate::W<Udma188Spec>;
#[doc = "Field `UART11TXBufBaseAddr` reader - UART11 TX buffer base address"]
pub type Uart11txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART11TXBufBaseAddr` writer - UART11 TX buffer base address"]
pub type Uart11txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART11 TX buffer base address"]
    #[inline(always)]
    pub fn uart11txbuf_base_addr(&self) -> Uart11txbufBaseAddrR {
        Uart11txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART11 TX buffer base address"]
    #[inline(always)]
    pub fn uart11txbuf_base_addr(&mut self) -> Uart11txbufBaseAddrW<Udma188Spec> {
        Uart11txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART11 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma188::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma188::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma188Spec;
impl crate::RegisterSpec for Udma188Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma188::R`](R) reader structure"]
impl crate::Readable for Udma188Spec {}
#[doc = "`write(|w| ..)` method takes [`udma188::W`](W) writer structure"]
impl crate::Writable for Udma188Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA188 to value 0"]
impl crate::Resettable for Udma188Spec {}
