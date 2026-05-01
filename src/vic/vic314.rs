#[doc = "Register `VIC314` reader"]
pub type R = crate::R<Vic314Spec>;
#[doc = "Register `VIC314` writer"]
pub type W = crate::W<Vic314Spec>;
#[doc = "Field `VICSIRQCSEL25` reader - VIC_SIRQ_CSEL2_5"]
pub type Vicsirqcsel25R = crate::FieldReader<u32>;
#[doc = "Field `VICSIRQCSEL25` writer - VIC_SIRQ_CSEL2_5"]
pub type Vicsirqcsel25W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL2_5"]
    #[inline(always)]
    pub fn vicsirqcsel25(&self) -> Vicsirqcsel25R {
        Vicsirqcsel25R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL2_5"]
    #[inline(always)]
    pub fn vicsirqcsel25(&mut self) -> Vicsirqcsel25W<Vic314Spec> {
        Vicsirqcsel25W::new(self, 0)
    }
}
#[doc = "Int Routing Select2 5\n\nYou can [`read`](crate::Reg::read) this register and get [`vic314::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic314::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic314Spec;
impl crate::RegisterSpec for Vic314Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic314::R`](R) reader structure"]
impl crate::Readable for Vic314Spec {}
#[doc = "`write(|w| ..)` method takes [`vic314::W`](W) writer structure"]
impl crate::Writable for Vic314Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC314 to value 0"]
impl crate::Resettable for Vic314Spec {}
