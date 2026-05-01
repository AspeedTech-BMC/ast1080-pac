#[doc = "Register `UDMA130` reader"]
pub type R = crate::R<Udma130Spec>;
#[doc = "Register `UDMA130` writer"]
pub type W = crate::W<Udma130Spec>;
#[doc = "Field `UART8RXReadPointer` reader - UART8 RX read pointer"]
pub type Uart8rxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART8RXReadPointer` writer - UART8 RX read pointer"]
pub type Uart8rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART8 RX read pointer"]
    #[inline(always)]
    pub fn uart8rxread_pointer(&self) -> Uart8rxreadPointerR {
        Uart8rxreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART8 RX read pointer"]
    #[inline(always)]
    pub fn uart8rxread_pointer(&mut self) -> Uart8rxreadPointerW<Udma130Spec> {
        Uart8rxreadPointerW::new(self, 0)
    }
}
#[doc = "UART8 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma130::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma130::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma130Spec;
impl crate::RegisterSpec for Udma130Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma130::R`](R) reader structure"]
impl crate::Readable for Udma130Spec {}
#[doc = "`write(|w| ..)` method takes [`udma130::W`](W) writer structure"]
impl crate::Writable for Udma130Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA130 to value 0"]
impl crate::Resettable for Udma130Spec {}
