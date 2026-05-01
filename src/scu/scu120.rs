#[doc = "Register `SCU120` reader"]
pub type R = crate::R<Scu120Spec>;
#[doc = "Register `SCU120` writer"]
pub type W = crate::W<Scu120Spec>;
#[doc = "Field `SCUCPTRAPAGEADR0` reader - SCU_CPTRA_PAGE_ADR_0"]
pub type Scucptrapageadr0R = crate::FieldReader<u16>;
#[doc = "Field `SCUCPTRAPAGEADR0` writer - SCU_CPTRA_PAGE_ADR_0"]
pub type Scucptrapageadr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - SCU_CPTRA_PAGE_ADR_0"]
    #[inline(always)]
    pub fn scucptrapageadr0(&self) -> Scucptrapageadr0R {
        Scucptrapageadr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - SCU_CPTRA_PAGE_ADR_0"]
    #[inline(always)]
    pub fn scucptrapageadr0(&mut self) -> Scucptrapageadr0W<Scu120Spec> {
        Scucptrapageadr0W::new(self, 16)
    }
}
#[doc = "CPTRA Page Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu120::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu120::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu120Spec;
impl crate::RegisterSpec for Scu120Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu120::R`](R) reader structure"]
impl crate::Readable for Scu120Spec {}
#[doc = "`write(|w| ..)` method takes [`scu120::W`](W) writer structure"]
impl crate::Writable for Scu120Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU120 to value 0"]
impl crate::Resettable for Scu120Spec {}
