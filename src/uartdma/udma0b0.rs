#[doc = "Register `UDMA0B0` reader"]
pub type R = crate::R<Udma0b0Spec>;
#[doc = "Register `UDMA0B0` writer"]
pub type W = crate::W<Udma0b0Spec>;
#[doc = "Field `UART3RXReadPointer` reader - UART3 RX read pointer"]
pub type Uart3rxreadPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART3RXReadPointer` writer - UART3 RX read pointer"]
pub type Uart3rxreadPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART3 RX read pointer"]
    #[inline(always)]
    pub fn uart3rxread_pointer(&self) -> Uart3rxreadPointerR {
        Uart3rxreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART3 RX read pointer"]
    #[inline(always)]
    pub fn uart3rxread_pointer(&mut self) -> Uart3rxreadPointerW<Udma0b0Spec> {
        Uart3rxreadPointerW::new(self, 0)
    }
}
#[doc = "UART3 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0b0Spec;
impl crate::RegisterSpec for Udma0b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0b0::R`](R) reader structure"]
impl crate::Readable for Udma0b0Spec {}
#[doc = "`write(|w| ..)` method takes [`udma0b0::W`](W) writer structure"]
impl crate::Writable for Udma0b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0B0 to value 0"]
impl crate::Resettable for Udma0b0Spec {}
