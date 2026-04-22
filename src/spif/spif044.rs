#[doc = "Register `SPIF044` reader"]
pub type R = crate::R<Spif044Spec>;
#[doc = "Register `SPIF044` writer"]
pub type W = crate::W<Spif044Spec>;
#[doc = "Field `ELOG01` reader - ELOG01"]
pub type Elog01R = crate::FieldReader<u32>;
#[doc = "Field `ELOG01` writer - ELOG01"]
pub type Elog01W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ELOG01"]
    #[inline(always)]
    pub fn elog01(&self) -> Elog01R {
        Elog01R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ELOG01"]
    #[inline(always)]
    pub fn elog01(&mut self) -> Elog01W<Spif044Spec> {
        Elog01W::new(self, 0)
    }
}
#[doc = "SPIF\\_ELOG01\n\nYou can [`read`](crate::Reg::read) this register and get [`spif044::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif044::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif044Spec;
impl crate::RegisterSpec for Spif044Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif044::R`](R) reader structure"]
impl crate::Readable for Spif044Spec {}
#[doc = "`write(|w| ..)` method takes [`spif044::W`](W) writer structure"]
impl crate::Writable for Spif044Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF044 to value 0"]
impl crate::Resettable for Spif044Spec {}
