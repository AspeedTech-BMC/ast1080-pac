#[doc = "Register `UDMA138` reader"]
pub type R = crate::R<Udma138Spec>;
#[doc = "Register `UDMA138` writer"]
pub type W = crate::W<Udma138Spec>;
#[doc = "Field `UART8RXBufBaseAddr` reader - UART8 RX buffer base address"]
pub type Uart8rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART8RXBufBaseAddr` writer - UART8 RX buffer base address"]
pub type Uart8rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART8 RX buffer base address"]
    #[inline(always)]
    pub fn uart8rxbuf_base_addr(&self) -> Uart8rxbufBaseAddrR {
        Uart8rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART8 RX buffer base address"]
    #[inline(always)]
    pub fn uart8rxbuf_base_addr(&mut self) -> Uart8rxbufBaseAddrW<Udma138Spec> {
        Uart8rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART8 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma138::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma138::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma138Spec;
impl crate::RegisterSpec for Udma138Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma138::R`](R) reader structure"]
impl crate::Readable for Udma138Spec {}
#[doc = "`write(|w| ..)` method takes [`udma138::W`](W) writer structure"]
impl crate::Writable for Udma138Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA138 to value 0"]
impl crate::Resettable for Udma138Spec {}
