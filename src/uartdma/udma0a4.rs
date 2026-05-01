#[doc = "Register `UDMA0A4` reader"]
pub type R = crate::R<Udma0a4Spec>;
#[doc = "Register `UDMA0A4` writer"]
pub type W = crate::W<Udma0a4Spec>;
#[doc = "Field `UART3TXWrPointer` reader - UART3 TX write pointer"]
pub type Uart3txwrPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART3TXWrPointer` writer - UART3 TX write pointer"]
pub type Uart3txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART3 TX write pointer"]
    #[inline(always)]
    pub fn uart3txwr_pointer(&self) -> Uart3txwrPointerR {
        Uart3txwrPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART3 TX write pointer"]
    #[inline(always)]
    pub fn uart3txwr_pointer(&mut self) -> Uart3txwrPointerW<Udma0a4Spec> {
        Uart3txwrPointerW::new(self, 0)
    }
}
#[doc = "UART3 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0a4Spec;
impl crate::RegisterSpec for Udma0a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0a4::R`](R) reader structure"]
impl crate::Readable for Udma0a4Spec {}
#[doc = "`write(|w| ..)` method takes [`udma0a4::W`](W) writer structure"]
impl crate::Writable for Udma0a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0A4 to value 0"]
impl crate::Resettable for Udma0a4Spec {}
