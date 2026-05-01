#[doc = "Register `SCU12C` reader"]
pub type R = crate::R<Scu12cSpec>;
#[doc = "Register `SCU12C` writer"]
pub type W = crate::W<Scu12cSpec>;
#[doc = "Field `SCUCPTRAPAGEADR3` reader - SCU_CPTRA_PAGE_ADR_3"]
pub type Scucptrapageadr3R = crate::FieldReader<u16>;
#[doc = "Field `SCUCPTRAPAGEADR3` writer - SCU_CPTRA_PAGE_ADR_3"]
pub type Scucptrapageadr3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - SCU_CPTRA_PAGE_ADR_3"]
    #[inline(always)]
    pub fn scucptrapageadr3(&self) -> Scucptrapageadr3R {
        Scucptrapageadr3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - SCU_CPTRA_PAGE_ADR_3"]
    #[inline(always)]
    pub fn scucptrapageadr3(&mut self) -> Scucptrapageadr3W<Scu12cSpec> {
        Scucptrapageadr3W::new(self, 16)
    }
}
#[doc = "CPTRA Page Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu12c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu12c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu12cSpec;
impl crate::RegisterSpec for Scu12cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu12c::R`](R) reader structure"]
impl crate::Readable for Scu12cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu12c::W`](W) writer structure"]
impl crate::Writable for Scu12cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU12C to value 0"]
impl crate::Resettable for Scu12cSpec {}
