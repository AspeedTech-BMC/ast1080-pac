#[doc = "Register `SCU128` reader"]
pub type R = crate::R<Scu128Spec>;
#[doc = "Register `SCU128` writer"]
pub type W = crate::W<Scu128Spec>;
#[doc = "Field `SCUCPTRAPAGEADR2` reader - SCU_CPTRA_PAGE_ADR_2"]
pub type Scucptrapageadr2R = crate::FieldReader<u16>;
#[doc = "Field `SCUCPTRAPAGEADR2` writer - SCU_CPTRA_PAGE_ADR_2"]
pub type Scucptrapageadr2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - SCU_CPTRA_PAGE_ADR_2"]
    #[inline(always)]
    pub fn scucptrapageadr2(&self) -> Scucptrapageadr2R {
        Scucptrapageadr2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - SCU_CPTRA_PAGE_ADR_2"]
    #[inline(always)]
    pub fn scucptrapageadr2(&mut self) -> Scucptrapageadr2W<Scu128Spec> {
        Scucptrapageadr2W::new(self, 16)
    }
}
#[doc = "CPTRA Page Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu128::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu128::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu128Spec;
impl crate::RegisterSpec for Scu128Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu128::R`](R) reader structure"]
impl crate::Readable for Scu128Spec {}
#[doc = "`write(|w| ..)` method takes [`scu128::W`](W) writer structure"]
impl crate::Writable for Scu128Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU128 to value 0"]
impl crate::Resettable for Scu128Spec {}
