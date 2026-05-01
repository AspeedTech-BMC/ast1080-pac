#[doc = "Register `UDMA0A0` reader"]
pub type R = crate::R<Udma0a0Spec>;
#[doc = "Register `UDMA0A0` writer"]
pub type W = crate::W<Udma0a0Spec>;
#[doc = "Field `UART3TXReadPointer` reader - UART3 TX read pointer"]
pub type Uart3txreadPointerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART3 TX read pointer"]
    #[inline(always)]
    pub fn uart3txread_pointer(&self) -> Uart3txreadPointerR {
        Uart3txreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART3 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0a0Spec;
impl crate::RegisterSpec for Udma0a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0a0::R`](R) reader structure"]
impl crate::Readable for Udma0a0Spec {}
#[doc = "`write(|w| ..)` method takes [`udma0a0::W`](W) writer structure"]
impl crate::Writable for Udma0a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0A0 to value 0"]
impl crate::Resettable for Udma0a0Spec {}
