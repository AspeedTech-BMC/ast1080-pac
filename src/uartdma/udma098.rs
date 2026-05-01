#[doc = "Register `UDMA098` reader"]
pub type R = crate::R<Udma098Spec>;
#[doc = "Register `UDMA098` writer"]
pub type W = crate::W<Udma098Spec>;
#[doc = "Field `UART2RXBufBaseAddr` reader - UART2 RX buffer base address"]
pub type Uart2rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART2RXBufBaseAddr` writer - UART2 RX buffer base address"]
pub type Uart2rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART2 RX buffer base address"]
    #[inline(always)]
    pub fn uart2rxbuf_base_addr(&self) -> Uart2rxbufBaseAddrR {
        Uart2rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART2 RX buffer base address"]
    #[inline(always)]
    pub fn uart2rxbuf_base_addr(&mut self) -> Uart2rxbufBaseAddrW<Udma098Spec> {
        Uart2rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART2 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma098::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma098::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma098Spec;
impl crate::RegisterSpec for Udma098Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma098::R`](R) reader structure"]
impl crate::Readable for Udma098Spec {}
#[doc = "`write(|w| ..)` method takes [`udma098::W`](W) writer structure"]
impl crate::Writable for Udma098Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA098 to value 0"]
impl crate::Resettable for Udma098Spec {}
