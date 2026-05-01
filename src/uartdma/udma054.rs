#[doc = "Register `UDMA054` reader"]
pub type R = crate::R<Udma054Spec>;
#[doc = "Register `UDMA054` writer"]
pub type W = crate::W<Udma054Spec>;
#[doc = "Field `UART0RXWrPointer` reader - UART0 RX write pointer"]
pub type Uart0rxwrPointerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - UART0 RX write pointer"]
    #[inline(always)]
    pub fn uart0rxwr_pointer(&self) -> Uart0rxwrPointerR {
        Uart0rxwrPointerR::new(self.bits & 0x0001_ffff)
    }
}
impl W {}
#[doc = "UART0 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma054::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma054::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma054Spec;
impl crate::RegisterSpec for Udma054Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma054::R`](R) reader structure"]
impl crate::Readable for Udma054Spec {}
#[doc = "`write(|w| ..)` method takes [`udma054::W`](W) writer structure"]
impl crate::Writable for Udma054Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA054 to value 0"]
impl crate::Resettable for Udma054Spec {}
