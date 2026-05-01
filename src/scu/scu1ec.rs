#[doc = "Register `SCU1EC` reader"]
pub type R = crate::R<Scu1ecSpec>;
#[doc = "Register `SCU1EC` writer"]
pub type W = crate::W<Scu1ecSpec>;
#[doc = "Field `SCUSCRATCH28` reader - SCU_SCRATCH_28"]
pub type Scuscratch28R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH28` writer - SCU_SCRATCH_28"]
pub type Scuscratch28W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_28"]
    #[inline(always)]
    pub fn scuscratch28(&self) -> Scuscratch28R {
        Scuscratch28R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_28"]
    #[inline(always)]
    pub fn scuscratch28(&mut self) -> Scuscratch28W<Scu1ecSpec> {
        Scuscratch28W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_28\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1ecSpec;
impl crate::RegisterSpec for Scu1ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1ec::R`](R) reader structure"]
impl crate::Readable for Scu1ecSpec {}
#[doc = "`write(|w| ..)` method takes [`scu1ec::W`](W) writer structure"]
impl crate::Writable for Scu1ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1EC to value 0"]
impl crate::Resettable for Scu1ecSpec {}
