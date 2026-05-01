#[doc = "Register `UDMA160` reader"]
pub type R = crate::R<Udma160Spec>;
#[doc = "Register `UDMA160` writer"]
pub type W = crate::W<Udma160Spec>;
#[doc = "Field `UART10TXReadPointer` reader - UART10 TX read pointer"]
pub type Uart10txreadPointerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART10 TX read pointer"]
    #[inline(always)]
    pub fn uart10txread_pointer(&self) -> Uart10txreadPointerR {
        Uart10txreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART10 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma160::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma160::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma160Spec;
impl crate::RegisterSpec for Udma160Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma160::R`](R) reader structure"]
impl crate::Readable for Udma160Spec {}
#[doc = "`write(|w| ..)` method takes [`udma160::W`](W) writer structure"]
impl crate::Writable for Udma160Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA160 to value 0"]
impl crate::Resettable for Udma160Spec {}
