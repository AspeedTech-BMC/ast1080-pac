#[doc = "Register `UDMA164` reader"]
pub type R = crate::R<Udma164Spec>;
#[doc = "Register `UDMA164` writer"]
pub type W = crate::W<Udma164Spec>;
#[doc = "Field `UART10TXWrPointer` reader - UART10 TX write pointer"]
pub type Uart10txwrPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART10TXWrPointer` writer - UART10 TX write pointer"]
pub type Uart10txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART10 TX write pointer"]
    #[inline(always)]
    pub fn uart10txwr_pointer(&self) -> Uart10txwrPointerR {
        Uart10txwrPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART10 TX write pointer"]
    #[inline(always)]
    pub fn uart10txwr_pointer(&mut self) -> Uart10txwrPointerW<Udma164Spec> {
        Uart10txwrPointerW::new(self, 0)
    }
}
#[doc = "UART10 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma164::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma164::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma164Spec;
impl crate::RegisterSpec for Udma164Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma164::R`](R) reader structure"]
impl crate::Readable for Udma164Spec {}
#[doc = "`write(|w| ..)` method takes [`udma164::W`](W) writer structure"]
impl crate::Writable for Udma164Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA164 to value 0"]
impl crate::Resettable for Udma164Spec {}
