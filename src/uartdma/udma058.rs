#[doc = "Register `UDMA058` reader"]
pub type R = crate::R<Udma058Spec>;
#[doc = "Register `UDMA058` writer"]
pub type W = crate::W<Udma058Spec>;
#[doc = "Field `UART0RXBufBaseAddr` reader - UART0 RX buffer base address"]
pub type Uart0rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART0RXBufBaseAddr` writer - UART0 RX buffer base address"]
pub type Uart0rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART0 RX buffer base address"]
    #[inline(always)]
    pub fn uart0rxbuf_base_addr(&self) -> Uart0rxbufBaseAddrR {
        Uart0rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART0 RX buffer base address"]
    #[inline(always)]
    pub fn uart0rxbuf_base_addr(&mut self) -> Uart0rxbufBaseAddrW<Udma058Spec> {
        Uart0rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART0 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma058::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma058::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma058Spec;
impl crate::RegisterSpec for Udma058Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma058::R`](R) reader structure"]
impl crate::Readable for Udma058Spec {}
#[doc = "`write(|w| ..)` method takes [`udma058::W`](W) writer structure"]
impl crate::Writable for Udma058Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA058 to value 0"]
impl crate::Resettable for Udma058Spec {}
