#[doc = "Register `UARTDMA0C4` reader"]
pub type R = crate::R<Uartdma0c4Spec>;
#[doc = "Register `UARTDMA0C4` writer"]
pub type W = crate::W<Uartdma0c4Spec>;
#[doc = "Field `UART5TXWrPointer` reader - UART5 TX write pointer"]
pub type Uart5txwrPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART5TXWrPointer` writer - UART5 TX write pointer"]
pub type Uart5txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART5 TX write pointer"]
    #[inline(always)]
    pub fn uart5txwr_pointer(&self) -> Uart5txwrPointerR {
        Uart5txwrPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART5 TX write pointer"]
    #[inline(always)]
    pub fn uart5txwr_pointer(&mut self) -> Uart5txwrPointerW<Uartdma0c4Spec> {
        Uart5txwrPointerW::new(self, 0)
    }
}
#[doc = "UART5 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma0c4Spec;
impl crate::RegisterSpec for Uartdma0c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma0c4::R`](R) reader structure"]
impl crate::Readable for Uartdma0c4Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma0c4::W`](W) writer structure"]
impl crate::Writable for Uartdma0c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA0C4 to value 0"]
impl crate::Resettable for Uartdma0c4Spec {}
