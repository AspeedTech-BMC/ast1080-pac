#[doc = "Register `UDMA184` reader"]
pub type R = crate::R<Udma184Spec>;
#[doc = "Register `UDMA184` writer"]
pub type W = crate::W<Udma184Spec>;
#[doc = "Field `UART11TXWrPointer` reader - UART11 TX write pointer"]
pub type Uart11txwrPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART11TXWrPointer` writer - UART11 TX write pointer"]
pub type Uart11txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART11 TX write pointer"]
    #[inline(always)]
    pub fn uart11txwr_pointer(&self) -> Uart11txwrPointerR {
        Uart11txwrPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART11 TX write pointer"]
    #[inline(always)]
    pub fn uart11txwr_pointer(&mut self) -> Uart11txwrPointerW<Udma184Spec> {
        Uart11txwrPointerW::new(self, 0)
    }
}
#[doc = "UART11 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma184::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma184::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma184Spec;
impl crate::RegisterSpec for Udma184Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma184::R`](R) reader structure"]
impl crate::Readable for Udma184Spec {}
#[doc = "`write(|w| ..)` method takes [`udma184::W`](W) writer structure"]
impl crate::Writable for Udma184Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA184 to value 0"]
impl crate::Resettable for Udma184Spec {}
