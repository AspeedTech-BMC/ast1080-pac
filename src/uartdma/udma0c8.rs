#[doc = "Register `UDMA0C8` reader"]
pub type R = crate::R<Udma0c8Spec>;
#[doc = "Register `UDMA0C8` writer"]
pub type W = crate::W<Udma0c8Spec>;
#[doc = "Field `UART5TXBufBaseAddr` reader - UART5 TX buffer base address"]
pub type Uart5txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART5TXBufBaseAddr` writer - UART5 TX buffer base address"]
pub type Uart5txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART5 TX buffer base address"]
    #[inline(always)]
    pub fn uart5txbuf_base_addr(&self) -> Uart5txbufBaseAddrR {
        Uart5txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART5 TX buffer base address"]
    #[inline(always)]
    pub fn uart5txbuf_base_addr(&mut self) -> Uart5txbufBaseAddrW<Udma0c8Spec> {
        Uart5txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART5 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0c8Spec;
impl crate::RegisterSpec for Udma0c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0c8::R`](R) reader structure"]
impl crate::Readable for Udma0c8Spec {}
#[doc = "`write(|w| ..)` method takes [`udma0c8::W`](W) writer structure"]
impl crate::Writable for Udma0c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0C8 to value 0"]
impl crate::Resettable for Udma0c8Spec {}
