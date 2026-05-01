#[doc = "Register `UDMA088` reader"]
pub type R = crate::R<Udma088Spec>;
#[doc = "Register `UDMA088` writer"]
pub type W = crate::W<Udma088Spec>;
#[doc = "Field `UART2TXBufBaseAddr` reader - UART2 TX buffer base address"]
pub type Uart2txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART2TXBufBaseAddr` writer - UART2 TX buffer base address"]
pub type Uart2txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART2 TX buffer base address"]
    #[inline(always)]
    pub fn uart2txbuf_base_addr(&self) -> Uart2txbufBaseAddrR {
        Uart2txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART2 TX buffer base address"]
    #[inline(always)]
    pub fn uart2txbuf_base_addr(&mut self) -> Uart2txbufBaseAddrW<Udma088Spec> {
        Uart2txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART2 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma088Spec;
impl crate::RegisterSpec for Udma088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma088::R`](R) reader structure"]
impl crate::Readable for Udma088Spec {}
#[doc = "`write(|w| ..)` method takes [`udma088::W`](W) writer structure"]
impl crate::Writable for Udma088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA088 to value 0"]
impl crate::Resettable for Udma088Spec {}
