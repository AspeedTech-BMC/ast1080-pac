#[doc = "Register `SPIF048` reader"]
pub type R = crate::R<Spif048Spec>;
#[doc = "Register `SPIF048` writer"]
pub type W = crate::W<Spif048Spec>;
#[doc = "Field `ELOG02` reader - ELOG02"]
pub type Elog02R = crate::FieldReader<u32>;
#[doc = "Field `ELOG02` writer - ELOG02"]
pub type Elog02W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ELOG02"]
    #[inline(always)]
    pub fn elog02(&self) -> Elog02R {
        Elog02R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ELOG02"]
    #[inline(always)]
    pub fn elog02(&mut self) -> Elog02W<Spif048Spec> {
        Elog02W::new(self, 0)
    }
}
#[doc = "SPIF\\_ELOG02\n\nYou can [`read`](crate::Reg::read) this register and get [`spif048::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif048::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif048Spec;
impl crate::RegisterSpec for Spif048Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif048::R`](R) reader structure"]
impl crate::Readable for Spif048Spec {}
#[doc = "`write(|w| ..)` method takes [`spif048::W`](W) writer structure"]
impl crate::Writable for Spif048Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF048 to value 0"]
impl crate::Resettable for Spif048Spec {}
