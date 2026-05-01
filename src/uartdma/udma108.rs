#[doc = "Register `UDMA108` reader"]
pub type R = crate::R<Udma108Spec>;
#[doc = "Register `UDMA108` writer"]
pub type W = crate::W<Udma108Spec>;
#[doc = "Field `UART7TXBufBaseAddr` reader - UART7 TX buffer base address"]
pub type Uart7txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART7TXBufBaseAddr` writer - UART7 TX buffer base address"]
pub type Uart7txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART7 TX buffer base address"]
    #[inline(always)]
    pub fn uart7txbuf_base_addr(&self) -> Uart7txbufBaseAddrR {
        Uart7txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART7 TX buffer base address"]
    #[inline(always)]
    pub fn uart7txbuf_base_addr(&mut self) -> Uart7txbufBaseAddrW<Udma108Spec> {
        Uart7txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART7 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma108::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma108::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma108Spec;
impl crate::RegisterSpec for Udma108Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma108::R`](R) reader structure"]
impl crate::Readable for Udma108Spec {}
#[doc = "`write(|w| ..)` method takes [`udma108::W`](W) writer structure"]
impl crate::Writable for Udma108Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA108 to value 0"]
impl crate::Resettable for Udma108Spec {}
