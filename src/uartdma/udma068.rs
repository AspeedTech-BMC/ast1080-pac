#[doc = "Register `UDMA068` reader"]
pub type R = crate::R<Udma068Spec>;
#[doc = "Register `UDMA068` writer"]
pub type W = crate::W<Udma068Spec>;
#[doc = "Field `UART1TXBufBaseAddr` reader - UART1 TX buffer base address"]
pub type Uart1txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART1TXBufBaseAddr` writer - UART1 TX buffer base address"]
pub type Uart1txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART1 TX buffer base address"]
    #[inline(always)]
    pub fn uart1txbuf_base_addr(&self) -> Uart1txbufBaseAddrR {
        Uart1txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART1 TX buffer base address"]
    #[inline(always)]
    pub fn uart1txbuf_base_addr(&mut self) -> Uart1txbufBaseAddrW<Udma068Spec> {
        Uart1txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART1 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma068::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma068::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma068Spec;
impl crate::RegisterSpec for Udma068Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma068::R`](R) reader structure"]
impl crate::Readable for Udma068Spec {}
#[doc = "`write(|w| ..)` method takes [`udma068::W`](W) writer structure"]
impl crate::Writable for Udma068Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA068 to value 0"]
impl crate::Resettable for Udma068Spec {}
