#[doc = "Register `SCU1CC` reader"]
pub type R = crate::R<Scu1ccSpec>;
#[doc = "Register `SCU1CC` writer"]
pub type W = crate::W<Scu1ccSpec>;
#[doc = "Field `SCUSCRATCH20` reader - SCU_SCRATCH_20"]
pub type Scuscratch20R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH20` writer - SCU_SCRATCH_20"]
pub type Scuscratch20W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_20"]
    #[inline(always)]
    pub fn scuscratch20(&self) -> Scuscratch20R {
        Scuscratch20R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_20"]
    #[inline(always)]
    pub fn scuscratch20(&mut self) -> Scuscratch20W<Scu1ccSpec> {
        Scuscratch20W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_20\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1ccSpec;
impl crate::RegisterSpec for Scu1ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1cc::R`](R) reader structure"]
impl crate::Readable for Scu1ccSpec {}
#[doc = "`write(|w| ..)` method takes [`scu1cc::W`](W) writer structure"]
impl crate::Writable for Scu1ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1CC to value 0"]
impl crate::Resettable for Scu1ccSpec {}
