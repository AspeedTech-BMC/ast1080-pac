#[doc = "Register `GSRAM168` reader"]
pub type R = crate::R<Gsram168Spec>;
#[doc = "Register `GSRAM168` writer"]
pub type W = crate::W<Gsram168Spec>;
#[doc = "Field `WLOCK58` reader - WLOCK58"]
pub type Wlock58R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK58` writer - WLOCK58"]
pub type Wlock58W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK58"]
    #[inline(always)]
    pub fn wlock58(&self) -> Wlock58R {
        Wlock58R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK58"]
    #[inline(always)]
    pub fn wlock58(&mut self) -> Wlock58W<Gsram168Spec> {
        Wlock58W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK58\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram168::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram168::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram168Spec;
impl crate::RegisterSpec for Gsram168Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram168::R`](R) reader structure"]
impl crate::Readable for Gsram168Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram168::W`](W) writer structure"]
impl crate::Writable for Gsram168Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM168 to value 0"]
impl crate::Resettable for Gsram168Spec {}
