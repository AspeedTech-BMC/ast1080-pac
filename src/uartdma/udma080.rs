#[doc = "Register `UDMA080` reader"]
pub type R = crate::R<Udma080Spec>;
#[doc = "Register `UDMA080` writer"]
pub type W = crate::W<Udma080Spec>;
#[doc = "Field `UART2TXReadPointer` reader - UART2 TX read pointer"]
pub type Uart2txreadPointerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART2 TX read pointer"]
    #[inline(always)]
    pub fn uart2txread_pointer(&self) -> Uart2txreadPointerR {
        Uart2txreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART2 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma080Spec;
impl crate::RegisterSpec for Udma080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma080::R`](R) reader structure"]
impl crate::Readable for Udma080Spec {}
#[doc = "`write(|w| ..)` method takes [`udma080::W`](W) writer structure"]
impl crate::Writable for Udma080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA080 to value 0"]
impl crate::Resettable for Udma080Spec {}
