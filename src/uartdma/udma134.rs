#[doc = "Register `UDMA134` reader"]
pub type R = crate::R<Udma134Spec>;
#[doc = "Register `UDMA134` writer"]
pub type W = crate::W<Udma134Spec>;
#[doc = "Field `UART8RXWrPointer` reader - UART8 RX write pointer"]
pub type Uart8rxwrPointerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - UART8 RX write pointer"]
    #[inline(always)]
    pub fn uart8rxwr_pointer(&self) -> Uart8rxwrPointerR {
        Uart8rxwrPointerR::new(self.bits & 0x0001_ffff)
    }
}
impl W {}
#[doc = "UART8 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma134::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma134::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma134Spec;
impl crate::RegisterSpec for Udma134Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma134::R`](R) reader structure"]
impl crate::Readable for Udma134Spec {}
#[doc = "`write(|w| ..)` method takes [`udma134::W`](W) writer structure"]
impl crate::Writable for Udma134Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA134 to value 0"]
impl crate::Resettable for Udma134Spec {}
