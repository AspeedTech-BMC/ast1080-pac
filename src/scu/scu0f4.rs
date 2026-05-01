#[doc = "Register `SCU0F4` reader"]
pub type R = crate::R<Scu0f4Spec>;
#[doc = "Register `SCU0F4` writer"]
pub type W = crate::W<Scu0f4Spec>;
#[doc = "Field `SCURNGDAT` reader - SCU_RNG_DAT"]
pub type ScurngdatR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_RNG_DAT"]
    #[inline(always)]
    pub fn scurngdat(&self) -> ScurngdatR {
        ScurngdatR::new(self.bits)
    }
}
impl W {}
#[doc = "Random Number Generator Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu0f4Spec;
impl crate::RegisterSpec for Scu0f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu0f4::R`](R) reader structure"]
impl crate::Readable for Scu0f4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu0f4::W`](W) writer structure"]
impl crate::Writable for Scu0f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU0F4 to value 0"]
impl crate::Resettable for Scu0f4Spec {}
