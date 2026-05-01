#[doc = "Register `HUB10` reader"]
pub type R = crate::R<Hub10Spec>;
#[doc = "Register `HUB10` writer"]
pub type W = crate::W<Hub10Spec>;
#[doc = "Field `ProgrammableEndpointACKINTEnbl` reader - Programmable Endpoint ACK Interrupt Enable"]
pub type ProgrammableEndpointAckintenblR = crate::FieldReader<u32>;
#[doc = "Field `ProgrammableEndpointACKINTEnbl` writer - Programmable Endpoint ACK Interrupt Enable"]
pub type ProgrammableEndpointAckintenblW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:20 - Programmable Endpoint ACK Interrupt Enable"]
    #[inline(always)]
    pub fn programmable_endpoint_ackintenbl(&self) -> ProgrammableEndpointAckintenblR {
        ProgrammableEndpointAckintenblR::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bits 21:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:20 - Programmable Endpoint ACK Interrupt Enable"]
    #[inline(always)]
    pub fn programmable_endpoint_ackintenbl(
        &mut self,
    ) -> ProgrammableEndpointAckintenblW<Hub10Spec> {
        ProgrammableEndpointAckintenblW::new(self, 0)
    }
}
#[doc = "Programmable Endpoint Pool ACK Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub10Spec;
impl crate::RegisterSpec for Hub10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub10::R`](R) reader structure"]
impl crate::Readable for Hub10Spec {}
#[doc = "`write(|w| ..)` method takes [`hub10::W`](W) writer structure"]
impl crate::Writable for Hub10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB10 to value 0"]
impl crate::Resettable for Hub10Spec {}
