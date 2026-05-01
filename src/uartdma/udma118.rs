#[doc = "Register `UDMA118` reader"]
pub type R = crate::R<Udma118Spec>;
#[doc = "Register `UDMA118` writer"]
pub type W = crate::W<Udma118Spec>;
#[doc = "Field `UART7RXBufBaseAddr` reader - UART7 RX buffer base address"]
pub type Uart7rxbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART7RXBufBaseAddr` writer - UART7 RX buffer base address"]
pub type Uart7rxbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART7 RX buffer base address"]
    #[inline(always)]
    pub fn uart7rxbuf_base_addr(&self) -> Uart7rxbufBaseAddrR {
        Uart7rxbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART7 RX buffer base address"]
    #[inline(always)]
    pub fn uart7rxbuf_base_addr(&mut self) -> Uart7rxbufBaseAddrW<Udma118Spec> {
        Uart7rxbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART7 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma118::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma118::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma118Spec;
impl crate::RegisterSpec for Udma118Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma118::R`](R) reader structure"]
impl crate::Readable for Udma118Spec {}
#[doc = "`write(|w| ..)` method takes [`udma118::W`](W) writer structure"]
impl crate::Writable for Udma118Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA118 to value 0"]
impl crate::Resettable for Udma118Spec {}
