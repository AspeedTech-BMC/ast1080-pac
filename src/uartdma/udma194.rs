#[doc = "Register `UDMA194` reader"]
pub type R = crate::R<Udma194Spec>;
#[doc = "Register `UDMA194` writer"]
pub type W = crate::W<Udma194Spec>;
#[doc = "Field `UART11RXWrPointer` reader - UART11 RX write pointer"]
pub type Uart11rxwrPointerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - UART11 RX write pointer"]
    #[inline(always)]
    pub fn uart11rxwr_pointer(&self) -> Uart11rxwrPointerR {
        Uart11rxwrPointerR::new(self.bits & 0x0001_ffff)
    }
}
impl W {}
#[doc = "UART11 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma194::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma194::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma194Spec;
impl crate::RegisterSpec for Udma194Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma194::R`](R) reader structure"]
impl crate::Readable for Udma194Spec {}
#[doc = "`write(|w| ..)` method takes [`udma194::W`](W) writer structure"]
impl crate::Writable for Udma194Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA194 to value 0"]
impl crate::Resettable for Udma194Spec {}
