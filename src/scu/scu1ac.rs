#[doc = "Register `SCU1AC` reader"]
pub type R = crate::R<Scu1acSpec>;
#[doc = "Register `SCU1AC` writer"]
pub type W = crate::W<Scu1acSpec>;
#[doc = "Field `SCUSCRATCH12` reader - SCU_SCRATCH_12"]
pub type Scuscratch12R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH12` writer - SCU_SCRATCH_12"]
pub type Scuscratch12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_12"]
    #[inline(always)]
    pub fn scuscratch12(&self) -> Scuscratch12R {
        Scuscratch12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_12"]
    #[inline(always)]
    pub fn scuscratch12(&mut self) -> Scuscratch12W<Scu1acSpec> {
        Scuscratch12W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_12\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1acSpec;
impl crate::RegisterSpec for Scu1acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1ac::R`](R) reader structure"]
impl crate::Readable for Scu1acSpec {}
#[doc = "`write(|w| ..)` method takes [`scu1ac::W`](W) writer structure"]
impl crate::Writable for Scu1acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1AC to value 0"]
impl crate::Resettable for Scu1acSpec {}
