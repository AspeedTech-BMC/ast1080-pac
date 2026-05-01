#[doc = "Register `SCU1E4` reader"]
pub type R = crate::R<Scu1e4Spec>;
#[doc = "Register `SCU1E4` writer"]
pub type W = crate::W<Scu1e4Spec>;
#[doc = "Field `SCUSCRATCH26` reader - SCU_SCRATCH_26"]
pub type Scuscratch26R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH26` writer - SCU_SCRATCH_26"]
pub type Scuscratch26W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_26"]
    #[inline(always)]
    pub fn scuscratch26(&self) -> Scuscratch26R {
        Scuscratch26R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_26"]
    #[inline(always)]
    pub fn scuscratch26(&mut self) -> Scuscratch26W<Scu1e4Spec> {
        Scuscratch26W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_26\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1e4Spec;
impl crate::RegisterSpec for Scu1e4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1e4::R`](R) reader structure"]
impl crate::Readable for Scu1e4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu1e4::W`](W) writer structure"]
impl crate::Writable for Scu1e4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1E4 to value 0"]
impl crate::Resettable for Scu1e4Spec {}
