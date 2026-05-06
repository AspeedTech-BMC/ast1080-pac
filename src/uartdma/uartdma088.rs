#[doc = "Register `UARTDMA088` reader"]
pub type R = crate::R<Uartdma088Spec>;
#[doc = "Register `UARTDMA088` writer"]
pub type W = crate::W<Uartdma088Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `UART2TXBufBaseAddr` reader - UART2 TX buffer base address"]
pub type Uart2txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART2TXBufBaseAddr` writer - UART2 TX buffer base address"]
pub type Uart2txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - UART2 TX buffer base address"]
    #[inline(always)]
    pub fn uart2txbuf_base_addr(&self) -> Uart2txbufBaseAddrR {
        Uart2txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART2 TX buffer base address"]
    #[inline(always)]
    pub fn uart2txbuf_base_addr(&mut self) -> Uart2txbufBaseAddrW<Uartdma088Spec> {
        Uart2txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART2 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma088Spec;
impl crate::RegisterSpec for Uartdma088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma088::R`](R) reader structure"]
impl crate::Readable for Uartdma088Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma088::W`](W) writer structure"]
impl crate::Writable for Uartdma088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA088 to value 0"]
impl crate::Resettable for Uartdma088Spec {}
