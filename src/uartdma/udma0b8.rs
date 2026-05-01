#[doc = "Register `UDMA0B8` reader"]
pub type R = crate::R<Udma0b8Spec>;
#[doc = "Register `UDMA0B8` writer"]
pub type W = crate::W<Udma0b8Spec>;
#[doc = "Field `UART3RXBufBaseAddr` reader - UART3 RX buffer base address"]
pub type Uart3rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART3RXBufBaseAddr` writer - UART3 RX buffer base address"]
pub type Uart3rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART3 RX buffer base address"]
    #[inline(always)]
    pub fn uart3rxbuf_base_addr(&self) -> Uart3rxbufBaseAddrR {
        Uart3rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART3 RX buffer base address"]
    #[inline(always)]
    pub fn uart3rxbuf_base_addr(&mut self) -> Uart3rxbufBaseAddrW<Udma0b8Spec> {
        Uart3rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART3 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0b8Spec;
impl crate::RegisterSpec for Udma0b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0b8::R`](R) reader structure"]
impl crate::Readable for Udma0b8Spec {}
#[doc = "`write(|w| ..)` method takes [`udma0b8::W`](W) writer structure"]
impl crate::Writable for Udma0b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0B8 to value 0"]
impl crate::Resettable for Udma0b8Spec {}
