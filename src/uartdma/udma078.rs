#[doc = "Register `UDMA078` reader"]
pub type R = crate::R<Udma078Spec>;
#[doc = "Register `UDMA078` writer"]
pub type W = crate::W<Udma078Spec>;
#[doc = "Field `UART1RXBufBaseAddr` reader - UART1 RX buffer base address"]
pub type Uart1rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART1RXBufBaseAddr` writer - UART1 RX buffer base address"]
pub type Uart1rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART1 RX buffer base address"]
    #[inline(always)]
    pub fn uart1rxbuf_base_addr(&self) -> Uart1rxbufBaseAddrR {
        Uart1rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART1 RX buffer base address"]
    #[inline(always)]
    pub fn uart1rxbuf_base_addr(&mut self) -> Uart1rxbufBaseAddrW<Udma078Spec> {
        Uart1rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART1 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma078::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma078::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma078Spec;
impl crate::RegisterSpec for Udma078Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma078::R`](R) reader structure"]
impl crate::Readable for Udma078Spec {}
#[doc = "`write(|w| ..)` method takes [`udma078::W`](W) writer structure"]
impl crate::Writable for Udma078Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA078 to value 0"]
impl crate::Resettable for Udma078Spec {}
