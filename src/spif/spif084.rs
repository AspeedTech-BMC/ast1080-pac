#[doc = "Register `SPIF084` reader"]
pub type R = crate::R<Spif084Spec>;
#[doc = "Register `SPIF084` writer"]
pub type W = crate::W<Spif084Spec>;
#[doc = "Field `WTABLE01` reader - WTABLE01"]
pub type Wtable01R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE01` writer - WTABLE01"]
pub type Wtable01W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE01"]
    #[inline(always)]
    pub fn wtable01(&self) -> Wtable01R {
        Wtable01R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE01"]
    #[inline(always)]
    pub fn wtable01(&mut self) -> Wtable01W<Spif084Spec> {
        Wtable01W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE01\n\nYou can [`read`](crate::Reg::read) this register and get [`spif084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif084Spec;
impl crate::RegisterSpec for Spif084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif084::R`](R) reader structure"]
impl crate::Readable for Spif084Spec {}
#[doc = "`write(|w| ..)` method takes [`spif084::W`](W) writer structure"]
impl crate::Writable for Spif084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF084 to value 0"]
impl crate::Resettable for Spif084Spec {}
