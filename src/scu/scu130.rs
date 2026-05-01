#[doc = "Register `SCU130` reader"]
pub type R = crate::R<Scu130Spec>;
#[doc = "Register `SCU130` writer"]
pub type W = crate::W<Scu130Spec>;
#[doc = "Field `SCUCPTRAPAGEADR4` reader - SCU_CPTRA_PAGE_ADR_4"]
pub type Scucptrapageadr4R = crate::FieldReader<u16>;
#[doc = "Field `SCUCPTRAPAGEADR4` writer - SCU_CPTRA_PAGE_ADR_4"]
pub type Scucptrapageadr4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - SCU_CPTRA_PAGE_ADR_4"]
    #[inline(always)]
    pub fn scucptrapageadr4(&self) -> Scucptrapageadr4R {
        Scucptrapageadr4R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - SCU_CPTRA_PAGE_ADR_4"]
    #[inline(always)]
    pub fn scucptrapageadr4(&mut self) -> Scucptrapageadr4W<Scu130Spec> {
        Scucptrapageadr4W::new(self, 16)
    }
}
#[doc = "CPTRA Page Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu130::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu130::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu130Spec;
impl crate::RegisterSpec for Scu130Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu130::R`](R) reader structure"]
impl crate::Readable for Scu130Spec {}
#[doc = "`write(|w| ..)` method takes [`scu130::W`](W) writer structure"]
impl crate::Writable for Scu130Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU130 to value 0"]
impl crate::Resettable for Scu130Spec {}
