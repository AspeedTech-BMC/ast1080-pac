#[doc = "Register `UARTDMA084` reader"]
pub type R = crate::R<Uartdma084Spec>;
#[doc = "Register `UARTDMA084` writer"]
pub type W = crate::W<Uartdma084Spec>;
#[doc = "Field `UART2TXWrPointer` reader - UART2 TX write pointer"]
pub type Uart2txwrPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART2TXWrPointer` writer - UART2 TX write pointer"]
pub type Uart2txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART2 TX write pointer"]
    #[inline(always)]
    pub fn uart2txwr_pointer(&self) -> Uart2txwrPointerR {
        Uart2txwrPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART2 TX write pointer"]
    #[inline(always)]
    pub fn uart2txwr_pointer(&mut self) -> Uart2txwrPointerW<Uartdma084Spec> {
        Uart2txwrPointerW::new(self, 0)
    }
}
#[doc = "UART2 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma084Spec;
impl crate::RegisterSpec for Uartdma084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma084::R`](R) reader structure"]
impl crate::Readable for Uartdma084Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma084::W`](W) writer structure"]
impl crate::Writable for Uartdma084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA084 to value 0"]
impl crate::Resettable for Uartdma084Spec {}
