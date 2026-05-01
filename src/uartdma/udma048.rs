#[doc = "Register `UDMA048` reader"]
pub type R = crate::R<Udma048Spec>;
#[doc = "Register `UDMA048` writer"]
pub type W = crate::W<Udma048Spec>;
#[doc = "Field `UART0TXBufBaseAddr` reader - UART0 TX buffer base address"]
pub type Uart0txbufBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `UART0TXBufBaseAddr` writer - UART0 TX buffer base address"]
pub type Uart0txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - UART0 TX buffer base address"]
    #[inline(always)]
    pub fn uart0txbuf_base_addr(&self) -> Uart0txbufBaseAddrR {
        Uart0txbufBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - UART0 TX buffer base address"]
    #[inline(always)]
    pub fn uart0txbuf_base_addr(&mut self) -> Uart0txbufBaseAddrW<Udma048Spec> {
        Uart0txbufBaseAddrW::new(self, 2)
    }
}
#[doc = "UART0 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma048::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma048::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma048Spec;
impl crate::RegisterSpec for Udma048Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma048::R`](R) reader structure"]
impl crate::Readable for Udma048Spec {}
#[doc = "`write(|w| ..)` method takes [`udma048::W`](W) writer structure"]
impl crate::Writable for Udma048Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA048 to value 0"]
impl crate::Resettable for Udma048Spec {}
