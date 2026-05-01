#[doc = "Register `UDMA154` reader"]
pub type R = crate::R<Udma154Spec>;
#[doc = "Register `UDMA154` writer"]
pub type W = crate::W<Udma154Spec>;
#[doc = "Field `UART9RXWrPointer` reader - UART9 RX write pointer"]
pub type Uart9rxwrPointerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - UART9 RX write pointer"]
    #[inline(always)]
    pub fn uart9rxwr_pointer(&self) -> Uart9rxwrPointerR {
        Uart9rxwrPointerR::new(self.bits & 0x0001_ffff)
    }
}
impl W {}
#[doc = "UART9 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma154::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma154::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma154Spec;
impl crate::RegisterSpec for Udma154Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma154::R`](R) reader structure"]
impl crate::Readable for Udma154Spec {}
#[doc = "`write(|w| ..)` method takes [`udma154::W`](W) writer structure"]
impl crate::Writable for Udma154Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA154 to value 0"]
impl crate::Resettable for Udma154Spec {}
