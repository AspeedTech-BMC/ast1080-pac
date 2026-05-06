#[doc = "Register `UARTDMA124` reader"]
pub type R = crate::R<Uartdma124Spec>;
#[doc = "Register `UARTDMA124` writer"]
pub type W = crate::W<Uartdma124Spec>;
#[doc = "Field `UART8TXWrPointer` reader - UART8 TX write pointer"]
pub type Uart8txwrPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART8TXWrPointer` writer - UART8 TX write pointer"]
pub type Uart8txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART8 TX write pointer"]
    #[inline(always)]
    pub fn uart8txwr_pointer(&self) -> Uart8txwrPointerR {
        Uart8txwrPointerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART8 TX write pointer"]
    #[inline(always)]
    pub fn uart8txwr_pointer(&mut self) -> Uart8txwrPointerW<Uartdma124Spec> {
        Uart8txwrPointerW::new(self, 0)
    }
}
#[doc = "UART8 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma124::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma124::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma124Spec;
impl crate::RegisterSpec for Uartdma124Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma124::R`](R) reader structure"]
impl crate::Readable for Uartdma124Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma124::W`](W) writer structure"]
impl crate::Writable for Uartdma124Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA124 to value 0"]
impl crate::Resettable for Uartdma124Spec {}
