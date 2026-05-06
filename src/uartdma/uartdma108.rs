#[doc = "Register `UARTDMA108` reader"]
pub type R = crate::R<Uartdma108Spec>;
#[doc = "Register `UARTDMA108` writer"]
pub type W = crate::W<Uartdma108Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `UART7TXBufBaseAddr` reader - UART7 TX buffer base address"]
pub type Uart7txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART7TXBufBaseAddr` writer - UART7 TX buffer base address"]
pub type Uart7txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - UART7 TX buffer base address"]
    #[inline(always)]
    pub fn uart7txbuf_base_addr(&self) -> Uart7txbufBaseAddrR {
        Uart7txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART7 TX buffer base address"]
    #[inline(always)]
    pub fn uart7txbuf_base_addr(&mut self) -> Uart7txbufBaseAddrW<Uartdma108Spec> {
        Uart7txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART7 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma108::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma108::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma108Spec;
impl crate::RegisterSpec for Uartdma108Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma108::R`](R) reader structure"]
impl crate::Readable for Uartdma108Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma108::W`](W) writer structure"]
impl crate::Writable for Uartdma108Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA108 to value 0"]
impl crate::Resettable for Uartdma108Spec {}
