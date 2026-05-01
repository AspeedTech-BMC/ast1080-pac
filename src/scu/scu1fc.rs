#[doc = "Register `SCU1FC` reader"]
pub type R = crate::R<Scu1fcSpec>;
#[doc = "Register `SCU1FC` writer"]
pub type W = crate::W<Scu1fcSpec>;
#[doc = "Field `SCUSCRATCH32` reader - SCU_SCRATCH_32"]
pub type Scuscratch32R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH32` writer - SCU_SCRATCH_32"]
pub type Scuscratch32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_32"]
    #[inline(always)]
    pub fn scuscratch32(&self) -> Scuscratch32R {
        Scuscratch32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_32"]
    #[inline(always)]
    pub fn scuscratch32(&mut self) -> Scuscratch32W<Scu1fcSpec> {
        Scuscratch32W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_32\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1fcSpec;
impl crate::RegisterSpec for Scu1fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1fc::R`](R) reader structure"]
impl crate::Readable for Scu1fcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu1fc::W`](W) writer structure"]
impl crate::Writable for Scu1fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1FC to value 0"]
impl crate::Resettable for Scu1fcSpec {}
