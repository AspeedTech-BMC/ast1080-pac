#[doc = "Register `HUB18` reader"]
pub type R = crate::R<Hub18Spec>;
#[doc = "Register `HUB18` writer"]
pub type W = crate::W<Hub18Spec>;
#[doc = "Field `ProgrammableEndpointACKINTOccurs` reader - Programmable Endpoint ACK Interrupt Occurs"]
pub type ProgrammableEndpointAckintoccursR = crate::FieldReader<u32>;
#[doc = "Field `ProgrammableEndpointACKINTOccurs` writer - Programmable Endpoint ACK Interrupt Occurs"]
pub type ProgrammableEndpointAckintoccursW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:20 - Programmable Endpoint ACK Interrupt Occurs"]
    #[inline(always)]
    pub fn programmable_endpoint_ackintoccurs(&self) -> ProgrammableEndpointAckintoccursR {
        ProgrammableEndpointAckintoccursR::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bits 21:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:20 - Programmable Endpoint ACK Interrupt Occurs"]
    #[inline(always)]
    pub fn programmable_endpoint_ackintoccurs(
        &mut self,
    ) -> ProgrammableEndpointAckintoccursW<Hub18Spec> {
        ProgrammableEndpointAckintoccursW::new(self, 0)
    }
}
#[doc = "Programmable Endpoint Pool ACK Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub18Spec;
impl crate::RegisterSpec for Hub18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub18::R`](R) reader structure"]
impl crate::Readable for Hub18Spec {}
#[doc = "`write(|w| ..)` method takes [`hub18::W`](W) writer structure"]
impl crate::Writable for Hub18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB18 to value 0"]
impl crate::Resettable for Hub18Spec {}
