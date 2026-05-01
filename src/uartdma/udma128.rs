#[doc = "Register `UDMA128` reader"]
pub type R = crate::R<Udma128Spec>;
#[doc = "Register `UDMA128` writer"]
pub type W = crate::W<Udma128Spec>;
#[doc = "Field `UART8TXBufBaseAddr` reader - UART8 TX buffer base address"]
pub type Uart8txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART8TXBufBaseAddr` writer - UART8 TX buffer base address"]
pub type Uart8txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART8 TX buffer base address"]
    #[inline(always)]
    pub fn uart8txbuf_base_addr(&self) -> Uart8txbufBaseAddrR {
        Uart8txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART8 TX buffer base address"]
    #[inline(always)]
    pub fn uart8txbuf_base_addr(&mut self) -> Uart8txbufBaseAddrW<Udma128Spec> {
        Uart8txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART8 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma128::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma128::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma128Spec;
impl crate::RegisterSpec for Udma128Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma128::R`](R) reader structure"]
impl crate::Readable for Udma128Spec {}
#[doc = "`write(|w| ..)` method takes [`udma128::W`](W) writer structure"]
impl crate::Writable for Udma128Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA128 to value 0"]
impl crate::Resettable for Udma128Spec {}
