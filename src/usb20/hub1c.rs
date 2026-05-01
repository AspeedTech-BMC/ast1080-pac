#[doc = "Register `HUB1C` reader"]
pub type R = crate::R<Hub1cSpec>;
#[doc = "Register `HUB1C` writer"]
pub type W = crate::W<Hub1cSpec>;
#[doc = "Field `ProgrammableEndpointNAKINTOccurs` reader - Programmable Endpoint NAK Interrupt Occurs"]
pub type ProgrammableEndpointNakintoccursR = crate::FieldReader<u32>;
#[doc = "Field `ProgrammableEndpointNAKINTOccurs` writer - Programmable Endpoint NAK Interrupt Occurs"]
pub type ProgrammableEndpointNakintoccursW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:20 - Programmable Endpoint NAK Interrupt Occurs"]
    #[inline(always)]
    pub fn programmable_endpoint_nakintoccurs(&self) -> ProgrammableEndpointNakintoccursR {
        ProgrammableEndpointNakintoccursR::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bits 20:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:20 - Programmable Endpoint NAK Interrupt Occurs"]
    #[inline(always)]
    pub fn programmable_endpoint_nakintoccurs(
        &mut self,
    ) -> ProgrammableEndpointNakintoccursW<Hub1cSpec> {
        ProgrammableEndpointNakintoccursW::new(self, 0)
    }
}
#[doc = "Programmable Endpoint Pool NAK Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub1cSpec;
impl crate::RegisterSpec for Hub1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub1c::R`](R) reader structure"]
impl crate::Readable for Hub1cSpec {}
#[doc = "`write(|w| ..)` method takes [`hub1c::W`](W) writer structure"]
impl crate::Writable for Hub1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB1C to value 0"]
impl crate::Resettable for Hub1cSpec {}
