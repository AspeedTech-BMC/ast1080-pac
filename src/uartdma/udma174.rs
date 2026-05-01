#[doc = "Register `UDMA174` reader"]
pub type R = crate::R<Udma174Spec>;
#[doc = "Register `UDMA174` writer"]
pub type W = crate::W<Udma174Spec>;
#[doc = "Field `UART10RXWrPointer` reader - UART10 RX write pointer"]
pub type Uart10rxwrPointerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - UART10 RX write pointer"]
    #[inline(always)]
    pub fn uart10rxwr_pointer(&self) -> Uart10rxwrPointerR {
        Uart10rxwrPointerR::new(self.bits & 0x0001_ffff)
    }
}
impl W {}
#[doc = "UART10 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma174::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma174::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma174Spec;
impl crate::RegisterSpec for Udma174Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma174::R`](R) reader structure"]
impl crate::Readable for Udma174Spec {}
#[doc = "`write(|w| ..)` method takes [`udma174::W`](W) writer structure"]
impl crate::Writable for Udma174Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA174 to value 0"]
impl crate::Resettable for Udma174Spec {}
