#[doc = "Register `GSRAM154` reader"]
pub type R = crate::R<Gsram154Spec>;
#[doc = "Register `GSRAM154` writer"]
pub type W = crate::W<Gsram154Spec>;
#[doc = "Field `WLOCK53` reader - WLOCK53"]
pub type Wlock53R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK53` writer - WLOCK53"]
pub type Wlock53W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK53"]
    #[inline(always)]
    pub fn wlock53(&self) -> Wlock53R {
        Wlock53R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK53"]
    #[inline(always)]
    pub fn wlock53(&mut self) -> Wlock53W<Gsram154Spec> {
        Wlock53W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK53\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram154::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram154::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram154Spec;
impl crate::RegisterSpec for Gsram154Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram154::R`](R) reader structure"]
impl crate::Readable for Gsram154Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram154::W`](W) writer structure"]
impl crate::Writable for Gsram154Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM154 to value 0"]
impl crate::Resettable for Gsram154Spec {}
