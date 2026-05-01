#[doc = "Register `UDMA140` reader"]
pub type R = crate::R<Udma140Spec>;
#[doc = "Register `UDMA140` writer"]
pub type W = crate::W<Udma140Spec>;
#[doc = "Field `UART9TXReadPointer` reader - UART9 TX read pointer"]
pub type Uart9txreadPointerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART9 TX read pointer"]
    #[inline(always)]
    pub fn uart9txread_pointer(&self) -> Uart9txreadPointerR {
        Uart9txreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART9 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma140::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma140::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma140Spec;
impl crate::RegisterSpec for Udma140Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma140::R`](R) reader structure"]
impl crate::Readable for Udma140Spec {}
#[doc = "`write(|w| ..)` method takes [`udma140::W`](W) writer structure"]
impl crate::Writable for Udma140Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA140 to value 0"]
impl crate::Resettable for Udma140Spec {}
