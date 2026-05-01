#[doc = "Register `SCU1B0` reader"]
pub type R = crate::R<Scu1b0Spec>;
#[doc = "Register `SCU1B0` writer"]
pub type W = crate::W<Scu1b0Spec>;
#[doc = "Field `SCUSCRATCH13` reader - SCU_SCRATCH_13"]
pub type Scuscratch13R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH13` writer - SCU_SCRATCH_13"]
pub type Scuscratch13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_13"]
    #[inline(always)]
    pub fn scuscratch13(&self) -> Scuscratch13R {
        Scuscratch13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_13"]
    #[inline(always)]
    pub fn scuscratch13(&mut self) -> Scuscratch13W<Scu1b0Spec> {
        Scuscratch13W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_13\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1b0Spec;
impl crate::RegisterSpec for Scu1b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1b0::R`](R) reader structure"]
impl crate::Readable for Scu1b0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu1b0::W`](W) writer structure"]
impl crate::Writable for Scu1b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1B0 to value 0"]
impl crate::Resettable for Scu1b0Spec {}
