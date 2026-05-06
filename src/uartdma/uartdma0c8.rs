#[doc = "Register `UARTDMA0C8` reader"]
pub type R = crate::R<Uartdma0c8Spec>;
#[doc = "Register `UARTDMA0C8` writer"]
pub type W = crate::W<Uartdma0c8Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `UART5TXBufBaseAddr` reader - UART5 TX buffer base address"]
pub type Uart5txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART5TXBufBaseAddr` writer - UART5 TX buffer base address"]
pub type Uart5txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - UART5 TX buffer base address"]
    #[inline(always)]
    pub fn uart5txbuf_base_addr(&self) -> Uart5txbufBaseAddrR {
        Uart5txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART5 TX buffer base address"]
    #[inline(always)]
    pub fn uart5txbuf_base_addr(&mut self) -> Uart5txbufBaseAddrW<Uartdma0c8Spec> {
        Uart5txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART5 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma0c8Spec;
impl crate::RegisterSpec for Uartdma0c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma0c8::R`](R) reader structure"]
impl crate::Readable for Uartdma0c8Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma0c8::W`](W) writer structure"]
impl crate::Writable for Uartdma0c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA0C8 to value 0"]
impl crate::Resettable for Uartdma0c8Spec {}
