#[doc = "Register `SCU1A8` reader"]
pub type R = crate::R<Scu1a8Spec>;
#[doc = "Register `SCU1A8` writer"]
pub type W = crate::W<Scu1a8Spec>;
#[doc = "Field `SCUSCRATCH11` reader - SCU_SCRATCH_11"]
pub type Scuscratch11R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH11` writer - SCU_SCRATCH_11"]
pub type Scuscratch11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_11"]
    #[inline(always)]
    pub fn scuscratch11(&self) -> Scuscratch11R {
        Scuscratch11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_11"]
    #[inline(always)]
    pub fn scuscratch11(&mut self) -> Scuscratch11W<Scu1a8Spec> {
        Scuscratch11W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_11\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1a8Spec;
impl crate::RegisterSpec for Scu1a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1a8::R`](R) reader structure"]
impl crate::Readable for Scu1a8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu1a8::W`](W) writer structure"]
impl crate::Writable for Scu1a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1A8 to value 0"]
impl crate::Resettable for Scu1a8Spec {}
