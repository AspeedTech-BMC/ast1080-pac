#[doc = "Register `UDMA0E0` reader"]
pub type R = crate::R<Udma0e0Spec>;
#[doc = "Register `UDMA0E0` writer"]
pub type W = crate::W<Udma0e0Spec>;
#[doc = "Field `UART6TXReadPointer` reader - UART6 TX read pointer"]
pub type Uart6txreadPointerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART6 TX read pointer"]
    #[inline(always)]
    pub fn uart6txread_pointer(&self) -> Uart6txreadPointerR {
        Uart6txreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART6 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0e0Spec;
impl crate::RegisterSpec for Udma0e0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0e0::R`](R) reader structure"]
impl crate::Readable for Udma0e0Spec {}
#[doc = "`write(|w| ..)` method takes [`udma0e0::W`](W) writer structure"]
impl crate::Writable for Udma0e0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0E0 to value 0"]
impl crate::Resettable for Udma0e0Spec {}
