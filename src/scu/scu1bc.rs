#[doc = "Register `SCU1BC` reader"]
pub type R = crate::R<Scu1bcSpec>;
#[doc = "Register `SCU1BC` writer"]
pub type W = crate::W<Scu1bcSpec>;
#[doc = "Field `SCUSCRATCH16` reader - SCU_SCRATCH_16"]
pub type Scuscratch16R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH16` writer - SCU_SCRATCH_16"]
pub type Scuscratch16W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_16"]
    #[inline(always)]
    pub fn scuscratch16(&self) -> Scuscratch16R {
        Scuscratch16R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_16"]
    #[inline(always)]
    pub fn scuscratch16(&mut self) -> Scuscratch16W<Scu1bcSpec> {
        Scuscratch16W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_16\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1bcSpec;
impl crate::RegisterSpec for Scu1bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1bc::R`](R) reader structure"]
impl crate::Readable for Scu1bcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu1bc::W`](W) writer structure"]
impl crate::Writable for Scu1bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1BC to value 0"]
impl crate::Resettable for Scu1bcSpec {}
