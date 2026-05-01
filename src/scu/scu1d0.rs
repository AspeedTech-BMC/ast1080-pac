#[doc = "Register `SCU1D0` reader"]
pub type R = crate::R<Scu1d0Spec>;
#[doc = "Register `SCU1D0` writer"]
pub type W = crate::W<Scu1d0Spec>;
#[doc = "Field `SCUSCRATCH21` reader - SCU_SCRATCH_21"]
pub type Scuscratch21R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH21` writer - SCU_SCRATCH_21"]
pub type Scuscratch21W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_21"]
    #[inline(always)]
    pub fn scuscratch21(&self) -> Scuscratch21R {
        Scuscratch21R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_21"]
    #[inline(always)]
    pub fn scuscratch21(&mut self) -> Scuscratch21W<Scu1d0Spec> {
        Scuscratch21W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_21\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1d0Spec;
impl crate::RegisterSpec for Scu1d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1d0::R`](R) reader structure"]
impl crate::Readable for Scu1d0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu1d0::W`](W) writer structure"]
impl crate::Writable for Scu1d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1D0 to value 0"]
impl crate::Resettable for Scu1d0Spec {}
