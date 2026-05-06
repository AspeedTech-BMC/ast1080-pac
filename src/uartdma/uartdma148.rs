#[doc = "Register `UARTDMA148` reader"]
pub type R = crate::R<Uartdma148Spec>;
#[doc = "Register `UARTDMA148` writer"]
pub type W = crate::W<Uartdma148Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `UART9TXBufBaseAddr` reader - UART9 TX buffer base address"]
pub type Uart9txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART9TXBufBaseAddr` writer - UART9 TX buffer base address"]
pub type Uart9txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - UART9 TX buffer base address"]
    #[inline(always)]
    pub fn uart9txbuf_base_addr(&self) -> Uart9txbufBaseAddrR {
        Uart9txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART9 TX buffer base address"]
    #[inline(always)]
    pub fn uart9txbuf_base_addr(&mut self) -> Uart9txbufBaseAddrW<Uartdma148Spec> {
        Uart9txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART9 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma148::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma148::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma148Spec;
impl crate::RegisterSpec for Uartdma148Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma148::R`](R) reader structure"]
impl crate::Readable for Uartdma148Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma148::W`](W) writer structure"]
impl crate::Writable for Uartdma148Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA148 to value 0"]
impl crate::Resettable for Uartdma148Spec {}
