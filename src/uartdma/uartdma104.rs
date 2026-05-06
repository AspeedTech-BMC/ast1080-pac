#[doc = "Register `UARTDMA104` reader"]
pub type R = crate::R<Uartdma104Spec>;
#[doc = "Register `UARTDMA104` writer"]
pub type W = crate::W<Uartdma104Spec>;
#[doc = "Field `UART7TXWrPointer` reader - UART7 TX write pointer"]
pub type Uart7txwrPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART7TXWrPointer` writer - UART7 TX write pointer"]
pub type Uart7txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART7 TX write pointer"]
    #[inline(always)]
    pub fn uart7txwr_pointer(&self) -> Uart7txwrPointerR {
        Uart7txwrPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART7 TX write pointer"]
    #[inline(always)]
    pub fn uart7txwr_pointer(&mut self) -> Uart7txwrPointerW<Uartdma104Spec> {
        Uart7txwrPointerW::new(self, 0)
    }
}
#[doc = "UART7 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma104::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma104::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma104Spec;
impl crate::RegisterSpec for Uartdma104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma104::R`](R) reader structure"]
impl crate::Readable for Uartdma104Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma104::W`](W) writer structure"]
impl crate::Writable for Uartdma104Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA104 to value 0"]
impl crate::Resettable for Uartdma104Spec {}
