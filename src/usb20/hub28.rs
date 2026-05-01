#[doc = "Register `HUB28` reader"]
pub type R = crate::R<Hub28Spec>;
#[doc = "Register `HUB28` writer"]
pub type W = crate::W<Hub28Spec>;
#[doc = "Field `ProgrammableEndpointIndex` reader - Programmable Endpoint Index"]
pub type ProgrammableEndpointIndexR = crate::FieldReader;
#[doc = "Field `ProgrammableEndpointIndex` writer - Programmable Endpoint Index"]
pub type ProgrammableEndpointIndexW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `EndpointDataToggleBitInitialValueSet` reader - Endpoint data toggle bit initial value set"]
pub type EndpointDataToggleBitInitialValueSetR = crate::BitReader;
#[doc = "Field `EndpointDataToggleBitInitialValueSet` writer - Endpoint data toggle bit initial value set"]
pub type EndpointDataToggleBitInitialValueSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:4 - Programmable Endpoint Index"]
    #[inline(always)]
    pub fn programmable_endpoint_index(&self) -> ProgrammableEndpointIndexR {
        ProgrammableEndpointIndexR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Endpoint data toggle bit initial value set"]
    #[inline(always)]
    pub fn endpoint_data_toggle_bit_initial_value_set(
        &self,
    ) -> EndpointDataToggleBitInitialValueSetR {
        EndpointDataToggleBitInitialValueSetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - Programmable Endpoint Index"]
    #[inline(always)]
    pub fn programmable_endpoint_index(&mut self) -> ProgrammableEndpointIndexW<Hub28Spec> {
        ProgrammableEndpointIndexW::new(self, 0)
    }
    #[doc = "Bit 8 - Endpoint data toggle bit initial value set"]
    #[inline(always)]
    pub fn endpoint_data_toggle_bit_initial_value_set(
        &mut self,
    ) -> EndpointDataToggleBitInitialValueSetW<Hub28Spec> {
        EndpointDataToggleBitInitialValueSetW::new(self, 8)
    }
}
#[doc = "Programmable Endpoint Pool Data Toggle Value Set\n\nYou can [`read`](crate::Reg::read) this register and get [`hub28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub28Spec;
impl crate::RegisterSpec for Hub28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub28::R`](R) reader structure"]
impl crate::Readable for Hub28Spec {}
#[doc = "`write(|w| ..)` method takes [`hub28::W`](W) writer structure"]
impl crate::Writable for Hub28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB28 to value 0"]
impl crate::Resettable for Hub28Spec {}
