#[doc = "Register `UDMA0D4` reader"]
pub type R = crate::R<Udma0d4Spec>;
#[doc = "Register `UDMA0D4` writer"]
pub type W = crate::W<Udma0d4Spec>;
#[doc = "Field `UART5RXWrPointer` reader - UART5 RX write pointer"]
pub type Uart5rxwrPointerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - UART5 RX write pointer"]
    #[inline(always)]
    pub fn uart5rxwr_pointer(&self) -> Uart5rxwrPointerR {
        Uart5rxwrPointerR::new(self.bits & 0x0001_ffff)
    }
}
impl W {}
#[doc = "UART5 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0d4Spec;
impl crate::RegisterSpec for Udma0d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0d4::R`](R) reader structure"]
impl crate::Readable for Udma0d4Spec {}
#[doc = "`write(|w| ..)` method takes [`udma0d4::W`](W) writer structure"]
impl crate::Writable for Udma0d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0D4 to value 0"]
impl crate::Resettable for Udma0d4Spec {}
