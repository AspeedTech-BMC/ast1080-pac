#[doc = "Register `UARTDMA044` reader"]
pub type R = crate::R<Uartdma044Spec>;
#[doc = "Register `UARTDMA044` writer"]
pub type W = crate::W<Uartdma044Spec>;
#[doc = "Field `UART0TXWrPointer` reader - UART0 TX write pointer"]
pub type Uart0txwrPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART0TXWrPointer` writer - UART0 TX write pointer"]
pub type Uart0txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART0 TX write pointer"]
    #[inline(always)]
    pub fn uart0txwr_pointer(&self) -> Uart0txwrPointerR {
        Uart0txwrPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART0 TX write pointer"]
    #[inline(always)]
    pub fn uart0txwr_pointer(&mut self) -> Uart0txwrPointerW<Uartdma044Spec> {
        Uart0txwrPointerW::new(self, 0)
    }
}
#[doc = "UART0 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma044::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma044::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma044Spec;
impl crate::RegisterSpec for Uartdma044Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma044::R`](R) reader structure"]
impl crate::Readable for Uartdma044Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma044::W`](W) writer structure"]
impl crate::Writable for Uartdma044Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA044 to value 0"]
impl crate::Resettable for Uartdma044Spec {}
