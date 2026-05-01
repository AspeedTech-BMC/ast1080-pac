#[doc = "Register `UDMA178` reader"]
pub type R = crate::R<Udma178Spec>;
#[doc = "Register `UDMA178` writer"]
pub type W = crate::W<Udma178Spec>;
#[doc = "Field `UART10RXBufBaseAddr` reader - UART10 RX buffer base address"]
pub type Uart10rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART10RXBufBaseAddr` writer - UART10 RX buffer base address"]
pub type Uart10rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART10 RX buffer base address"]
    #[inline(always)]
    pub fn uart10rxbuf_base_addr(&self) -> Uart10rxbufBaseAddrR {
        Uart10rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART10 RX buffer base address"]
    #[inline(always)]
    pub fn uart10rxbuf_base_addr(&mut self) -> Uart10rxbufBaseAddrW<Udma178Spec> {
        Uart10rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART10 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma178::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma178::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma178Spec;
impl crate::RegisterSpec for Udma178Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma178::R`](R) reader structure"]
impl crate::Readable for Udma178Spec {}
#[doc = "`write(|w| ..)` method takes [`udma178::W`](W) writer structure"]
impl crate::Writable for Udma178Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA178 to value 0"]
impl crate::Resettable for Udma178Spec {}
