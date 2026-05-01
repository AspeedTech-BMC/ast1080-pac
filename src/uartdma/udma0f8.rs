#[doc = "Register `UDMA0F8` reader"]
pub type R = crate::R<Udma0f8Spec>;
#[doc = "Register `UDMA0F8` writer"]
pub type W = crate::W<Udma0f8Spec>;
#[doc = "Field `UART6RXBufBaseAddr` reader - UART6 RX buffer base address"]
pub type Uart6rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART6RXBufBaseAddr` writer - UART6 RX buffer base address"]
pub type Uart6rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART6 RX buffer base address"]
    #[inline(always)]
    pub fn uart6rxbuf_base_addr(&self) -> Uart6rxbufBaseAddrR {
        Uart6rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART6 RX buffer base address"]
    #[inline(always)]
    pub fn uart6rxbuf_base_addr(&mut self) -> Uart6rxbufBaseAddrW<Udma0f8Spec> {
        Uart6rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART6 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0f8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0f8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0f8Spec;
impl crate::RegisterSpec for Udma0f8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0f8::R`](R) reader structure"]
impl crate::Readable for Udma0f8Spec {}
#[doc = "`write(|w| ..)` method takes [`udma0f8::W`](W) writer structure"]
impl crate::Writable for Udma0f8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0F8 to value 0"]
impl crate::Resettable for Udma0f8Spec {}
