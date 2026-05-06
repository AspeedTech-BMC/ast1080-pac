#[doc = "Register `UARTDMA164` reader"]
pub type R = crate::R<Uartdma164Spec>;
#[doc = "Register `UARTDMA164` writer"]
pub type W = crate::W<Uartdma164Spec>;
#[doc = "Field `UART10TXWrPointer` reader - UART10 TX write pointer"]
pub type Uart10txwrPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART10TXWrPointer` writer - UART10 TX write pointer"]
pub type Uart10txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART10 TX write pointer"]
    #[inline(always)]
    pub fn uart10txwr_pointer(&self) -> Uart10txwrPointerR {
        Uart10txwrPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART10 TX write pointer"]
    #[inline(always)]
    pub fn uart10txwr_pointer(&mut self) -> Uart10txwrPointerW<Uartdma164Spec> {
        Uart10txwrPointerW::new(self, 0)
    }
}
#[doc = "UART10 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma164::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma164::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma164Spec;
impl crate::RegisterSpec for Uartdma164Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma164::R`](R) reader structure"]
impl crate::Readable for Uartdma164Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma164::W`](W) writer structure"]
impl crate::Writable for Uartdma164Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA164 to value 0"]
impl crate::Resettable for Uartdma164Spec {}
