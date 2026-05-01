#[doc = "Register `SCU134` reader"]
pub type R = crate::R<Scu134Spec>;
#[doc = "Register `SCU134` writer"]
pub type W = crate::W<Scu134Spec>;
#[doc = "Field `SCUCPTRAPAGEADR5` reader - SCU_CPTRA_PAGE_ADR_5"]
pub type Scucptrapageadr5R = crate::FieldReader<u16>;
#[doc = "Field `SCUCPTRAPAGEADR5` writer - SCU_CPTRA_PAGE_ADR_5"]
pub type Scucptrapageadr5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - SCU_CPTRA_PAGE_ADR_5"]
    #[inline(always)]
    pub fn scucptrapageadr5(&self) -> Scucptrapageadr5R {
        Scucptrapageadr5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - SCU_CPTRA_PAGE_ADR_5"]
    #[inline(always)]
    pub fn scucptrapageadr5(&mut self) -> Scucptrapageadr5W<Scu134Spec> {
        Scucptrapageadr5W::new(self, 16)
    }
}
#[doc = "CPTRA Page Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu134::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu134::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu134Spec;
impl crate::RegisterSpec for Scu134Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu134::R`](R) reader structure"]
impl crate::Readable for Scu134Spec {}
#[doc = "`write(|w| ..)` method takes [`scu134::W`](W) writer structure"]
impl crate::Writable for Scu134Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU134 to value 0"]
impl crate::Resettable for Scu134Spec {}
