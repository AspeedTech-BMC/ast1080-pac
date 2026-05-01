#[doc = "Register `SCU14C` reader"]
pub type R = crate::R<Scu14cSpec>;
#[doc = "Register `SCU14C` writer"]
pub type W = crate::W<Scu14cSpec>;
#[doc = "Field `SCUCPTRACOREGENERICINPUT0` reader - SCU_CPTRA_CORE_GENERIC_INPUT_0"]
pub type Scucptracoregenericinput0R = crate::FieldReader<u32>;
#[doc = "Field `SCUCPTRACOREGENERICINPUT0` writer - SCU_CPTRA_CORE_GENERIC_INPUT_0"]
pub type Scucptracoregenericinput0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_CPTRA_CORE_GENERIC_INPUT_0"]
    #[inline(always)]
    pub fn scucptracoregenericinput0(&self) -> Scucptracoregenericinput0R {
        Scucptracoregenericinput0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_CPTRA_CORE_GENERIC_INPUT_0"]
    #[inline(always)]
    pub fn scucptracoregenericinput0(&mut self) -> Scucptracoregenericinput0W<Scu14cSpec> {
        Scucptracoregenericinput0W::new(self, 0)
    }
}
#[doc = "Caliptra Config Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu14c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu14c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu14cSpec;
impl crate::RegisterSpec for Scu14cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu14c::R`](R) reader structure"]
impl crate::Readable for Scu14cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu14c::W`](W) writer structure"]
impl crate::Writable for Scu14cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU14C to value 0"]
impl crate::Resettable for Scu14cSpec {}
