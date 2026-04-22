#[doc = "Register `SPIF058` reader"]
pub type R = crate::R<Spif058Spec>;
#[doc = "Register `SPIF058` writer"]
pub type W = crate::W<Spif058Spec>;
#[doc = "Field `ELOG06` reader - ELOG06"]
pub type Elog06R = crate::FieldReader<u32>;
#[doc = "Field `ELOG06` writer - ELOG06"]
pub type Elog06W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ELOG06"]
    #[inline(always)]
    pub fn elog06(&self) -> Elog06R {
        Elog06R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ELOG06"]
    #[inline(always)]
    pub fn elog06(&mut self) -> Elog06W<Spif058Spec> {
        Elog06W::new(self, 0)
    }
}
#[doc = "SPIF\\_ELOG06\n\nYou can [`read`](crate::Reg::read) this register and get [`spif058::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif058::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif058Spec;
impl crate::RegisterSpec for Spif058Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif058::R`](R) reader structure"]
impl crate::Readable for Spif058Spec {}
#[doc = "`write(|w| ..)` method takes [`spif058::W`](W) writer structure"]
impl crate::Writable for Spif058Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF058 to value 0"]
impl crate::Resettable for Spif058Spec {}
