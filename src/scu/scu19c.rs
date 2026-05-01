#[doc = "Register `SCU19C` reader"]
pub type R = crate::R<Scu19cSpec>;
#[doc = "Register `SCU19C` writer"]
pub type W = crate::W<Scu19cSpec>;
#[doc = "Field `SCUSCRATCH8` reader - SCU_SCRATCH_8"]
pub type Scuscratch8R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH8` writer - SCU_SCRATCH_8"]
pub type Scuscratch8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_8"]
    #[inline(always)]
    pub fn scuscratch8(&self) -> Scuscratch8R {
        Scuscratch8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_8"]
    #[inline(always)]
    pub fn scuscratch8(&mut self) -> Scuscratch8W<Scu19cSpec> {
        Scuscratch8W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_8\n\nYou can [`read`](crate::Reg::read) this register and get [`scu19c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu19c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu19cSpec;
impl crate::RegisterSpec for Scu19cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu19c::R`](R) reader structure"]
impl crate::Readable for Scu19cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu19c::W`](W) writer structure"]
impl crate::Writable for Scu19cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU19C to value 0"]
impl crate::Resettable for Scu19cSpec {}
