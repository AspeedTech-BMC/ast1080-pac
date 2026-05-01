#[doc = "Register `SCU144` reader"]
pub type R = crate::R<Scu144Spec>;
#[doc = "Register `SCU144` writer"]
pub type W = crate::W<Scu144Spec>;
#[doc = "Field `SCUCPTRASTRAPPRODDEBUG` reader - SCU_CPTRA_STRAP_PROD_DEBUG"]
pub type ScucptrastrapproddebugR = crate::FieldReader<u32>;
#[doc = "Field `SCUCPTRASTRAPPRODDEBUG` writer - SCU_CPTRA_STRAP_PROD_DEBUG"]
pub type ScucptrastrapproddebugW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_CPTRA_STRAP_PROD_DEBUG"]
    #[inline(always)]
    pub fn scucptrastrapproddebug(&self) -> ScucptrastrapproddebugR {
        ScucptrastrapproddebugR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_CPTRA_STRAP_PROD_DEBUG"]
    #[inline(always)]
    pub fn scucptrastrapproddebug(&mut self) -> ScucptrastrapproddebugW<Scu144Spec> {
        ScucptrastrapproddebugW::new(self, 0)
    }
}
#[doc = "Caliptra Config Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu144::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu144::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu144Spec;
impl crate::RegisterSpec for Scu144Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu144::R`](R) reader structure"]
impl crate::Readable for Scu144Spec {}
#[doc = "`write(|w| ..)` method takes [`scu144::W`](W) writer structure"]
impl crate::Writable for Scu144Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU144 to value 0"]
impl crate::Resettable for Scu144Spec {}
