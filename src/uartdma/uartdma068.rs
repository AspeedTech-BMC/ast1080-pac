#[doc = "Register `UARTDMA068` reader"]
pub type R = crate::R<Uartdma068Spec>;
#[doc = "Register `UARTDMA068` writer"]
pub type W = crate::W<Uartdma068Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `UART1TXBufBaseAddr` reader - UART1 TX buffer base address"]
pub type Uart1txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART1TXBufBaseAddr` writer - UART1 TX buffer base address"]
pub type Uart1txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - UART1 TX buffer base address"]
    #[inline(always)]
    pub fn uart1txbuf_base_addr(&self) -> Uart1txbufBaseAddrR {
        Uart1txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART1 TX buffer base address"]
    #[inline(always)]
    pub fn uart1txbuf_base_addr(&mut self) -> Uart1txbufBaseAddrW<Uartdma068Spec> {
        Uart1txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART1 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma068::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma068::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma068Spec;
impl crate::RegisterSpec for Uartdma068Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma068::R`](R) reader structure"]
impl crate::Readable for Uartdma068Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma068::W`](W) writer structure"]
impl crate::Writable for Uartdma068Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA068 to value 0"]
impl crate::Resettable for Uartdma068Spec {}
