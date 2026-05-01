#[doc = "Register `UDMA110` reader"]
pub type R = crate::R<Udma110Spec>;
#[doc = "Register `UDMA110` writer"]
pub type W = crate::W<Udma110Spec>;
#[doc = "Field `UART7RXReadPointer` reader - UART7 RX read pointer"]
pub type Uart7rxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART7RXReadPointer` writer - UART7 RX read pointer"]
pub type Uart7rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART7 RX read pointer"]
    #[inline(always)]
    pub fn uart7rxread_pointer(&self) -> Uart7rxreadPointerR {
        Uart7rxreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART7 RX read pointer"]
    #[inline(always)]
    pub fn uart7rxread_pointer(&mut self) -> Uart7rxreadPointerW<Udma110Spec> {
        Uart7rxreadPointerW::new(self, 0)
    }
}
#[doc = "UART7 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma110::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma110::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma110Spec;
impl crate::RegisterSpec for Udma110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma110::R`](R) reader structure"]
impl crate::Readable for Udma110Spec {}
#[doc = "`write(|w| ..)` method takes [`udma110::W`](W) writer structure"]
impl crate::Writable for Udma110Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA110 to value 0"]
impl crate::Resettable for Udma110Spec {}
