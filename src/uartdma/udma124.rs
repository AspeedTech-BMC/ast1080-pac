#[doc = "Register `UDMA124` reader"]
pub type R = crate::R<Udma124Spec>;
#[doc = "Register `UDMA124` writer"]
pub type W = crate::W<Udma124Spec>;
#[doc = "Field `UART8TXWrPointer` reader - UART8 TX write pointer"]
pub type Uart8txwrPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART8TXWrPointer` writer - UART8 TX write pointer"]
pub type Uart8txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART8 TX write pointer"]
    #[inline(always)]
    pub fn uart8txwr_pointer(&self) -> Uart8txwrPointerR {
        Uart8txwrPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART8 TX write pointer"]
    #[inline(always)]
    pub fn uart8txwr_pointer(&mut self) -> Uart8txwrPointerW<Udma124Spec> {
        Uart8txwrPointerW::new(self, 0)
    }
}
#[doc = "UART8 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma124::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma124::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma124Spec;
impl crate::RegisterSpec for Udma124Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma124::R`](R) reader structure"]
impl crate::Readable for Udma124Spec {}
#[doc = "`write(|w| ..)` method takes [`udma124::W`](W) writer structure"]
impl crate::Writable for Udma124Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA124 to value 0"]
impl crate::Resettable for Udma124Spec {}
