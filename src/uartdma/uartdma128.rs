#[doc = "Register `UARTDMA128` reader"]
pub type R = crate::R<Uartdma128Spec>;
#[doc = "Register `UARTDMA128` writer"]
pub type W = crate::W<Uartdma128Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `UART8TXBufBaseAddr` reader - UART8 TX buffer base address"]
pub type Uart8txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART8TXBufBaseAddr` writer - UART8 TX buffer base address"]
pub type Uart8txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - UART8 TX buffer base address"]
    #[inline(always)]
    pub fn uart8txbuf_base_addr(&self) -> Uart8txbufBaseAddrR {
        Uart8txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART8 TX buffer base address"]
    #[inline(always)]
    pub fn uart8txbuf_base_addr(&mut self) -> Uart8txbufBaseAddrW<Uartdma128Spec> {
        Uart8txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART8 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma128::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma128::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma128Spec;
impl crate::RegisterSpec for Uartdma128Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma128::R`](R) reader structure"]
impl crate::Readable for Uartdma128Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma128::W`](W) writer structure"]
impl crate::Writable for Uartdma128Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA128 to value 0"]
impl crate::Resettable for Uartdma128Spec {}
