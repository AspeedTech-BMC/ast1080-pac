#[doc = "Register `UDMA0C0` reader"]
pub type R = crate::R<Udma0c0Spec>;
#[doc = "Register `UDMA0C0` writer"]
pub type W = crate::W<Udma0c0Spec>;
#[doc = "Field `UART5TXReadPointer` reader - UART5 TX read pointer"]
pub type Uart5txreadPointerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART5 TX read pointer"]
    #[inline(always)]
    pub fn uart5txread_pointer(&self) -> Uart5txreadPointerR {
        Uart5txreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART5 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0c0Spec;
impl crate::RegisterSpec for Udma0c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0c0::R`](R) reader structure"]
impl crate::Readable for Udma0c0Spec {}
#[doc = "`write(|w| ..)` method takes [`udma0c0::W`](W) writer structure"]
impl crate::Writable for Udma0c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0C0 to value 0"]
impl crate::Resettable for Udma0c0Spec {}
