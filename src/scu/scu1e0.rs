#[doc = "Register `SCU1E0` reader"]
pub type R = crate::R<Scu1e0Spec>;
#[doc = "Register `SCU1E0` writer"]
pub type W = crate::W<Scu1e0Spec>;
#[doc = "Field `SCUSCRATCH25` reader - SCU_SCRATCH_25"]
pub type Scuscratch25R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH25` writer - SCU_SCRATCH_25"]
pub type Scuscratch25W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_25"]
    #[inline(always)]
    pub fn scuscratch25(&self) -> Scuscratch25R {
        Scuscratch25R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_25"]
    #[inline(always)]
    pub fn scuscratch25(&mut self) -> Scuscratch25W<Scu1e0Spec> {
        Scuscratch25W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_25\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1e0Spec;
impl crate::RegisterSpec for Scu1e0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1e0::R`](R) reader structure"]
impl crate::Readable for Scu1e0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu1e0::W`](W) writer structure"]
impl crate::Writable for Scu1e0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1E0 to value 0"]
impl crate::Resettable for Scu1e0Spec {}
