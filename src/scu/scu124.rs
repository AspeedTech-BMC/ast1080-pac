#[doc = "Register `SCU124` reader"]
pub type R = crate::R<Scu124Spec>;
#[doc = "Register `SCU124` writer"]
pub type W = crate::W<Scu124Spec>;
#[doc = "Field `SCUCPTRAPAGEADR1` reader - SCU_CPTRA_PAGE_ADR_1"]
pub type Scucptrapageadr1R = crate::FieldReader<u16>;
#[doc = "Field `SCUCPTRAPAGEADR1` writer - SCU_CPTRA_PAGE_ADR_1"]
pub type Scucptrapageadr1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - SCU_CPTRA_PAGE_ADR_1"]
    #[inline(always)]
    pub fn scucptrapageadr1(&self) -> Scucptrapageadr1R {
        Scucptrapageadr1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - SCU_CPTRA_PAGE_ADR_1"]
    #[inline(always)]
    pub fn scucptrapageadr1(&mut self) -> Scucptrapageadr1W<Scu124Spec> {
        Scucptrapageadr1W::new(self, 16)
    }
}
#[doc = "CPTRA Page Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu124::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu124::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu124Spec;
impl crate::RegisterSpec for Scu124Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu124::R`](R) reader structure"]
impl crate::Readable for Scu124Spec {}
#[doc = "`write(|w| ..)` method takes [`scu124::W`](W) writer structure"]
impl crate::Writable for Scu124Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU124 to value 0"]
impl crate::Resettable for Scu124Spec {}
