#[doc = "Register `UDMA0F4` reader"]
pub type R = crate::R<Udma0f4Spec>;
#[doc = "Register `UDMA0F4` writer"]
pub type W = crate::W<Udma0f4Spec>;
#[doc = "Field `UART6RXWrPointer` reader - UART6 RX write pointer"]
pub type Uart6rxwrPointerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - UART6 RX write pointer"]
    #[inline(always)]
    pub fn uart6rxwr_pointer(&self) -> Uart6rxwrPointerR {
        Uart6rxwrPointerR::new(self.bits & 0x0001_ffff)
    }
}
impl W {}
#[doc = "UART6 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0f4Spec;
impl crate::RegisterSpec for Udma0f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0f4::R`](R) reader structure"]
impl crate::Readable for Udma0f4Spec {}
#[doc = "`write(|w| ..)` method takes [`udma0f4::W`](W) writer structure"]
impl crate::Writable for Udma0f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0F4 to value 0"]
impl crate::Resettable for Udma0f4Spec {}
