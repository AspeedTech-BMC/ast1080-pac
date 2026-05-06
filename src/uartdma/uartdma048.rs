#[doc = "Register `UARTDMA048` reader"]
pub type R = crate::R<Uartdma048Spec>;
#[doc = "Register `UARTDMA048` writer"]
pub type W = crate::W<Uartdma048Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `UART0TXBufBaseAddr` reader - UART0 TX buffer base address"]
pub type Uart0txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART0TXBufBaseAddr` writer - UART0 TX buffer base address"]
pub type Uart0txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - UART0 TX buffer base address"]
    #[inline(always)]
    pub fn uart0txbuf_base_addr(&self) -> Uart0txbufBaseAddrR {
        Uart0txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART0 TX buffer base address"]
    #[inline(always)]
    pub fn uart0txbuf_base_addr(&mut self) -> Uart0txbufBaseAddrW<Uartdma048Spec> {
        Uart0txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART0 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma048::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma048::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma048Spec;
impl crate::RegisterSpec for Uartdma048Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma048::R`](R) reader structure"]
impl crate::Readable for Uartdma048Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma048::W`](W) writer structure"]
impl crate::Writable for Uartdma048Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA048 to value 0"]
impl crate::Resettable for Uartdma048Spec {}
