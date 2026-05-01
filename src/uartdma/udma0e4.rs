#[doc = "Register `UDMA0E4` reader"]
pub type R = crate::R<Udma0e4Spec>;
#[doc = "Register `UDMA0E4` writer"]
pub type W = crate::W<Udma0e4Spec>;
#[doc = "Field `UART6TXWrPointer` reader - UART6 TX write pointer"]
pub type Uart6txwrPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART6TXWrPointer` writer - UART6 TX write pointer"]
pub type Uart6txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART6 TX write pointer"]
    #[inline(always)]
    pub fn uart6txwr_pointer(&self) -> Uart6txwrPointerR {
        Uart6txwrPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART6 TX write pointer"]
    #[inline(always)]
    pub fn uart6txwr_pointer(&mut self) -> Uart6txwrPointerW<Udma0e4Spec> {
        Uart6txwrPointerW::new(self, 0)
    }
}
#[doc = "UART6 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0e4Spec;
impl crate::RegisterSpec for Udma0e4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0e4::R`](R) reader structure"]
impl crate::Readable for Udma0e4Spec {}
#[doc = "`write(|w| ..)` method takes [`udma0e4::W`](W) writer structure"]
impl crate::Writable for Udma0e4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0E4 to value 0"]
impl crate::Resettable for Udma0e4Spec {}
