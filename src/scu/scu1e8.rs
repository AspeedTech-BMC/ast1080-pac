#[doc = "Register `SCU1E8` reader"]
pub type R = crate::R<Scu1e8Spec>;
#[doc = "Register `SCU1E8` writer"]
pub type W = crate::W<Scu1e8Spec>;
#[doc = "Field `SCUSCRATCH27` reader - SCU_SCRATCH_27"]
pub type Scuscratch27R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH27` writer - SCU_SCRATCH_27"]
pub type Scuscratch27W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_27"]
    #[inline(always)]
    pub fn scuscratch27(&self) -> Scuscratch27R {
        Scuscratch27R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_27"]
    #[inline(always)]
    pub fn scuscratch27(&mut self) -> Scuscratch27W<Scu1e8Spec> {
        Scuscratch27W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_27\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1e8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1e8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1e8Spec;
impl crate::RegisterSpec for Scu1e8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1e8::R`](R) reader structure"]
impl crate::Readable for Scu1e8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu1e8::W`](W) writer structure"]
impl crate::Writable for Scu1e8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1E8 to value 0"]
impl crate::Resettable for Scu1e8Spec {}
