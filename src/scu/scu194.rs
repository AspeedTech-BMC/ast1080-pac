#[doc = "Register `SCU194` reader"]
pub type R = crate::R<Scu194Spec>;
#[doc = "Register `SCU194` writer"]
pub type W = crate::W<Scu194Spec>;
#[doc = "Field `SCUSCRATCH6` reader - SCU_SCRATCH_6"]
pub type Scuscratch6R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH6` writer - SCU_SCRATCH_6"]
pub type Scuscratch6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_6"]
    #[inline(always)]
    pub fn scuscratch6(&self) -> Scuscratch6R {
        Scuscratch6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_6"]
    #[inline(always)]
    pub fn scuscratch6(&mut self) -> Scuscratch6W<Scu194Spec> {
        Scuscratch6W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu194::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu194::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu194Spec;
impl crate::RegisterSpec for Scu194Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu194::R`](R) reader structure"]
impl crate::Readable for Scu194Spec {}
#[doc = "`write(|w| ..)` method takes [`scu194::W`](W) writer structure"]
impl crate::Writable for Scu194Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU194 to value 0"]
impl crate::Resettable for Scu194Spec {}
