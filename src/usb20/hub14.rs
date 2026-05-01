#[doc = "Register `HUB14` reader"]
pub type R = crate::R<Hub14Spec>;
#[doc = "Register `HUB14` writer"]
pub type W = crate::W<Hub14Spec>;
#[doc = "Field `ProgrammableEndpointNAKINTEnbl` reader - Programmable Endpoint NAK Interrupt Enable"]
pub type ProgrammableEndpointNakintenblR = crate::FieldReader<u32>;
#[doc = "Field `ProgrammableEndpointNAKINTEnbl` writer - Programmable Endpoint NAK Interrupt Enable"]
pub type ProgrammableEndpointNakintenblW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:20 - Programmable Endpoint NAK Interrupt Enable"]
    #[inline(always)]
    pub fn programmable_endpoint_nakintenbl(&self) -> ProgrammableEndpointNakintenblR {
        ProgrammableEndpointNakintenblR::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bits 21:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:20 - Programmable Endpoint NAK Interrupt Enable"]
    #[inline(always)]
    pub fn programmable_endpoint_nakintenbl(
        &mut self,
    ) -> ProgrammableEndpointNakintenblW<Hub14Spec> {
        ProgrammableEndpointNakintenblW::new(self, 0)
    }
}
#[doc = "Programmable Endpoint Pool NAK Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub14Spec;
impl crate::RegisterSpec for Hub14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub14::R`](R) reader structure"]
impl crate::Readable for Hub14Spec {}
#[doc = "`write(|w| ..)` method takes [`hub14::W`](W) writer structure"]
impl crate::Writable for Hub14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB14 to value 0"]
impl crate::Resettable for Hub14Spec {}
