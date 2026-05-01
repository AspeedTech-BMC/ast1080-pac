#[doc = "Register `UDMA050` reader"]
pub type R = crate::R<Udma050Spec>;
#[doc = "Register `UDMA050` writer"]
pub type W = crate::W<Udma050Spec>;
#[doc = "Field `UART0RXReadPointer` reader - UART0 RX read pointer"]
pub type Uart0rxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART0RXReadPointer` writer - UART0 RX read pointer"]
pub type Uart0rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART0 RX read pointer"]
    #[inline(always)]
    pub fn uart0rxread_pointer(&self) -> Uart0rxreadPointerR {
        Uart0rxreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART0 RX read pointer"]
    #[inline(always)]
    pub fn uart0rxread_pointer(&mut self) -> Uart0rxreadPointerW<Udma050Spec> {
        Uart0rxreadPointerW::new(self, 0)
    }
}
#[doc = "UART0 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma050::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma050::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma050Spec;
impl crate::RegisterSpec for Udma050Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma050::R`](R) reader structure"]
impl crate::Readable for Udma050Spec {}
#[doc = "`write(|w| ..)` method takes [`udma050::W`](W) writer structure"]
impl crate::Writable for Udma050Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA050 to value 0"]
impl crate::Resettable for Udma050Spec {}
