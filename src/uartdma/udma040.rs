#[doc = "Register `UDMA040` reader"]
pub type R = crate::R<Udma040Spec>;
#[doc = "Register `UDMA040` writer"]
pub type W = crate::W<Udma040Spec>;
#[doc = "Field `UART0TXReadPointer` reader - UART0 TX read pointer"]
pub type Uart0txreadPointerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART0 TX read pointer"]
    #[inline(always)]
    pub fn uart0txread_pointer(&self) -> Uart0txreadPointerR {
        Uart0txreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART0 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma040Spec;
impl crate::RegisterSpec for Udma040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma040::R`](R) reader structure"]
impl crate::Readable for Udma040Spec {}
#[doc = "`write(|w| ..)` method takes [`udma040::W`](W) writer structure"]
impl crate::Writable for Udma040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA040 to value 0"]
impl crate::Resettable for Udma040Spec {}
