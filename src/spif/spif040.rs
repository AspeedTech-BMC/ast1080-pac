#[doc = "Register `SPIF040` reader"]
pub type R = crate::R<Spif040Spec>;
#[doc = "Register `SPIF040` writer"]
pub type W = crate::W<Spif040Spec>;
#[doc = "Field `ELOG00` reader - ELOG00"]
pub type Elog00R = crate::FieldReader<u32>;
#[doc = "Field `ELOG00` writer - ELOG00"]
pub type Elog00W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ELOG00"]
    #[inline(always)]
    pub fn elog00(&self) -> Elog00R {
        Elog00R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ELOG00"]
    #[inline(always)]
    pub fn elog00(&mut self) -> Elog00W<Spif040Spec> {
        Elog00W::new(self, 0)
    }
}
#[doc = "SPIF\\_ELOG00\n\nYou can [`read`](crate::Reg::read) this register and get [`spif040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif040Spec;
impl crate::RegisterSpec for Spif040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif040::R`](R) reader structure"]
impl crate::Readable for Spif040Spec {}
#[doc = "`write(|w| ..)` method takes [`spif040::W`](W) writer structure"]
impl crate::Writable for Spif040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF040 to value 0"]
impl crate::Resettable for Spif040Spec {}
