#[doc = "Register `UDMA064` reader"]
pub type R = crate::R<Udma064Spec>;
#[doc = "Register `UDMA064` writer"]
pub type W = crate::W<Udma064Spec>;
#[doc = "Field `UART1TXWrPointer` reader - UART1 TX write pointer"]
pub type Uart1txwrPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART1TXWrPointer` writer - UART1 TX write pointer"]
pub type Uart1txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART1 TX write pointer"]
    #[inline(always)]
    pub fn uart1txwr_pointer(&self) -> Uart1txwrPointerR {
        Uart1txwrPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART1 TX write pointer"]
    #[inline(always)]
    pub fn uart1txwr_pointer(&mut self) -> Uart1txwrPointerW<Udma064Spec> {
        Uart1txwrPointerW::new(self, 0)
    }
}
#[doc = "UART1 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma064::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma064::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma064Spec;
impl crate::RegisterSpec for Udma064Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma064::R`](R) reader structure"]
impl crate::Readable for Udma064Spec {}
#[doc = "`write(|w| ..)` method takes [`udma064::W`](W) writer structure"]
impl crate::Writable for Udma064Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA064 to value 0"]
impl crate::Resettable for Udma064Spec {}
