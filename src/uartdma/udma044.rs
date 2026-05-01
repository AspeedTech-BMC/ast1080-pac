#[doc = "Register `UDMA044` reader"]
pub type R = crate::R<Udma044Spec>;
#[doc = "Register `UDMA044` writer"]
pub type W = crate::W<Udma044Spec>;
#[doc = "Field `UART0TXWrPointer` reader - UART0 TX write pointer"]
pub type Uart0txwrPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART0TXWrPointer` writer - UART0 TX write pointer"]
pub type Uart0txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART0 TX write pointer"]
    #[inline(always)]
    pub fn uart0txwr_pointer(&self) -> Uart0txwrPointerR {
        Uart0txwrPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART0 TX write pointer"]
    #[inline(always)]
    pub fn uart0txwr_pointer(&mut self) -> Uart0txwrPointerW<Udma044Spec> {
        Uart0txwrPointerW::new(self, 0)
    }
}
#[doc = "UART0 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma044::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma044::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma044Spec;
impl crate::RegisterSpec for Udma044Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma044::R`](R) reader structure"]
impl crate::Readable for Udma044Spec {}
#[doc = "`write(|w| ..)` method takes [`udma044::W`](W) writer structure"]
impl crate::Writable for Udma044Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA044 to value 0"]
impl crate::Resettable for Udma044Spec {}
