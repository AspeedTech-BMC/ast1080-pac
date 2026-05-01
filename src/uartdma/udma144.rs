#[doc = "Register `UDMA144` reader"]
pub type R = crate::R<Udma144Spec>;
#[doc = "Register `UDMA144` writer"]
pub type W = crate::W<Udma144Spec>;
#[doc = "Field `UART9TXWrPointer` reader - UART9 TX write pointer"]
pub type Uart9txwrPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART9TXWrPointer` writer - UART9 TX write pointer"]
pub type Uart9txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART9 TX write pointer"]
    #[inline(always)]
    pub fn uart9txwr_pointer(&self) -> Uart9txwrPointerR {
        Uart9txwrPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART9 TX write pointer"]
    #[inline(always)]
    pub fn uart9txwr_pointer(&mut self) -> Uart9txwrPointerW<Udma144Spec> {
        Uart9txwrPointerW::new(self, 0)
    }
}
#[doc = "UART9 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma144::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma144::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma144Spec;
impl crate::RegisterSpec for Udma144Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma144::R`](R) reader structure"]
impl crate::Readable for Udma144Spec {}
#[doc = "`write(|w| ..)` method takes [`udma144::W`](W) writer structure"]
impl crate::Writable for Udma144Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA144 to value 0"]
impl crate::Resettable for Udma144Spec {}
