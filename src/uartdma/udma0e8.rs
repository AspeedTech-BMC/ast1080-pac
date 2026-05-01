#[doc = "Register `UDMA0E8` reader"]
pub type R = crate::R<Udma0e8Spec>;
#[doc = "Register `UDMA0E8` writer"]
pub type W = crate::W<Udma0e8Spec>;
#[doc = "Field `UART6TXBufBaseAddr` reader - UART6 TX buffer base address"]
pub type Uart6txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART6TXBufBaseAddr` writer - UART6 TX buffer base address"]
pub type Uart6txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART6 TX buffer base address"]
    #[inline(always)]
    pub fn uart6txbuf_base_addr(&self) -> Uart6txbufBaseAddrR {
        Uart6txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART6 TX buffer base address"]
    #[inline(always)]
    pub fn uart6txbuf_base_addr(&mut self) -> Uart6txbufBaseAddrW<Udma0e8Spec> {
        Uart6txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART6 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0e8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0e8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0e8Spec;
impl crate::RegisterSpec for Udma0e8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0e8::R`](R) reader structure"]
impl crate::Readable for Udma0e8Spec {}
#[doc = "`write(|w| ..)` method takes [`udma0e8::W`](W) writer structure"]
impl crate::Writable for Udma0e8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0E8 to value 0"]
impl crate::Resettable for Udma0e8Spec {}
