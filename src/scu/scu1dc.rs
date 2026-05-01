#[doc = "Register `SCU1DC` reader"]
pub type R = crate::R<Scu1dcSpec>;
#[doc = "Register `SCU1DC` writer"]
pub type W = crate::W<Scu1dcSpec>;
#[doc = "Field `SCUSCRATCH24` reader - SCU_SCRATCH_24"]
pub type Scuscratch24R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH24` writer - SCU_SCRATCH_24"]
pub type Scuscratch24W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_24"]
    #[inline(always)]
    pub fn scuscratch24(&self) -> Scuscratch24R {
        Scuscratch24R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_24"]
    #[inline(always)]
    pub fn scuscratch24(&mut self) -> Scuscratch24W<Scu1dcSpec> {
        Scuscratch24W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_24\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1dcSpec;
impl crate::RegisterSpec for Scu1dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1dc::R`](R) reader structure"]
impl crate::Readable for Scu1dcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu1dc::W`](W) writer structure"]
impl crate::Writable for Scu1dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1DC to value 0"]
impl crate::Resettable for Scu1dcSpec {}
