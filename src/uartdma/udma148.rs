#[doc = "Register `UDMA148` reader"]
pub type R = crate::R<Udma148Spec>;
#[doc = "Register `UDMA148` writer"]
pub type W = crate::W<Udma148Spec>;
#[doc = "Field `UART9TXBufBaseAddr` reader - UART9 TX buffer base address"]
pub type Uart9txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART9TXBufBaseAddr` writer - UART9 TX buffer base address"]
pub type Uart9txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART9 TX buffer base address"]
    #[inline(always)]
    pub fn uart9txbuf_base_addr(&self) -> Uart9txbufBaseAddrR {
        Uart9txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART9 TX buffer base address"]
    #[inline(always)]
    pub fn uart9txbuf_base_addr(&mut self) -> Uart9txbufBaseAddrW<Udma148Spec> {
        Uart9txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART9 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma148::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma148::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma148Spec;
impl crate::RegisterSpec for Udma148Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma148::R`](R) reader structure"]
impl crate::Readable for Udma148Spec {}
#[doc = "`write(|w| ..)` method takes [`udma148::W`](W) writer structure"]
impl crate::Writable for Udma148Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA148 to value 0"]
impl crate::Resettable for Udma148Spec {}
