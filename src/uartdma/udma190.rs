#[doc = "Register `UDMA190` reader"]
pub type R = crate::R<Udma190Spec>;
#[doc = "Register `UDMA190` writer"]
pub type W = crate::W<Udma190Spec>;
#[doc = "Field `UART11RXReadPointer` reader - UART11 RX read pointer"]
pub type Uart11rxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART11RXReadPointer` writer - UART11 RX read pointer"]
pub type Uart11rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART11 RX read pointer"]
    #[inline(always)]
    pub fn uart11rxread_pointer(&self) -> Uart11rxreadPointerR {
        Uart11rxreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART11 RX read pointer"]
    #[inline(always)]
    pub fn uart11rxread_pointer(&mut self) -> Uart11rxreadPointerW<Udma190Spec> {
        Uart11rxreadPointerW::new(self, 0)
    }
}
#[doc = "UART11 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma190::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma190::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma190Spec;
impl crate::RegisterSpec for Udma190Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma190::R`](R) reader structure"]
impl crate::Readable for Udma190Spec {}
#[doc = "`write(|w| ..)` method takes [`udma190::W`](W) writer structure"]
impl crate::Writable for Udma190Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA190 to value 0"]
impl crate::Resettable for Udma190Spec {}
