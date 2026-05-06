#[doc = "Register `UARTDMA168` reader"]
pub type R = crate::R<Uartdma168Spec>;
#[doc = "Register `UARTDMA168` writer"]
pub type W = crate::W<Uartdma168Spec>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `UART10TXBufBaseAddr` reader - UART10 TX buffer base address"]
pub type Uart10txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART10TXBufBaseAddr` writer - UART10 TX buffer base address"]
pub type Uart10txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - UART10 TX buffer base address"]
    #[inline(always)]
    pub fn uart10txbuf_base_addr(&self) -> Uart10txbufBaseAddrR {
        Uart10txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART10 TX buffer base address"]
    #[inline(always)]
    pub fn uart10txbuf_base_addr(&mut self) -> Uart10txbufBaseAddrW<Uartdma168Spec> {
        Uart10txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART10 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma168::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma168::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma168Spec;
impl crate::RegisterSpec for Uartdma168Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma168::R`](R) reader structure"]
impl crate::Readable for Uartdma168Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma168::W`](W) writer structure"]
impl crate::Writable for Uartdma168Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA168 to value 0"]
impl crate::Resettable for Uartdma168Spec {}
