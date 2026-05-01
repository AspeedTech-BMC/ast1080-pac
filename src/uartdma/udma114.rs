#[doc = "Register `UDMA114` reader"]
pub type R = crate::R<Udma114Spec>;
#[doc = "Register `UDMA114` writer"]
pub type W = crate::W<Udma114Spec>;
#[doc = "Field `UART7RXWrPointer` reader - UART7 RX write pointer"]
pub type Uart7rxwrPointerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - UART7 RX write pointer"]
    #[inline(always)]
    pub fn uart7rxwr_pointer(&self) -> Uart7rxwrPointerR {
        Uart7rxwrPointerR::new(self.bits & 0x0001_ffff)
    }
}
impl W {}
#[doc = "UART7 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma114::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma114::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma114Spec;
impl crate::RegisterSpec for Udma114Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma114::R`](R) reader structure"]
impl crate::Readable for Udma114Spec {}
#[doc = "`write(|w| ..)` method takes [`udma114::W`](W) writer structure"]
impl crate::Writable for Udma114Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA114 to value 0"]
impl crate::Resettable for Udma114Spec {}
