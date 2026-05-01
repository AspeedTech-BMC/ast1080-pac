#[doc = "Register `UDMA120` reader"]
pub type R = crate::R<Udma120Spec>;
#[doc = "Register `UDMA120` writer"]
pub type W = crate::W<Udma120Spec>;
#[doc = "Field `UART8TXReadPointer` reader - UART8 TX read pointer"]
pub type Uart8txreadPointerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART8 TX read pointer"]
    #[inline(always)]
    pub fn uart8txread_pointer(&self) -> Uart8txreadPointerR {
        Uart8txreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART8 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma120::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma120::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma120Spec;
impl crate::RegisterSpec for Udma120Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma120::R`](R) reader structure"]
impl crate::Readable for Udma120Spec {}
#[doc = "`write(|w| ..)` method takes [`udma120::W`](W) writer structure"]
impl crate::Writable for Udma120Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA120 to value 0"]
impl crate::Resettable for Udma120Spec {}
