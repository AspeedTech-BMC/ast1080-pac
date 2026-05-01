#[doc = "Register `UDMA074` reader"]
pub type R = crate::R<Udma074Spec>;
#[doc = "Register `UDMA074` writer"]
pub type W = crate::W<Udma074Spec>;
#[doc = "Field `UART1RXWrPointer` reader - UART1 RX write pointer"]
pub type Uart1rxwrPointerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - UART1 RX write pointer"]
    #[inline(always)]
    pub fn uart1rxwr_pointer(&self) -> Uart1rxwrPointerR {
        Uart1rxwrPointerR::new(self.bits & 0x0001_ffff)
    }
}
impl W {}
#[doc = "UART1 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma074::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma074::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma074Spec;
impl crate::RegisterSpec for Udma074Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma074::R`](R) reader structure"]
impl crate::Readable for Udma074Spec {}
#[doc = "`write(|w| ..)` method takes [`udma074::W`](W) writer structure"]
impl crate::Writable for Udma074Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA074 to value 0"]
impl crate::Resettable for Udma074Spec {}
