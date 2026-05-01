#[doc = "Register `GSRAM114` reader"]
pub type R = crate::R<Gsram114Spec>;
#[doc = "Register `GSRAM114` writer"]
pub type W = crate::W<Gsram114Spec>;
#[doc = "Field `WLOCK37` reader - WLOCK37"]
pub type Wlock37R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK37` writer - WLOCK37"]
pub type Wlock37W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK37"]
    #[inline(always)]
    pub fn wlock37(&self) -> Wlock37R {
        Wlock37R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK37"]
    #[inline(always)]
    pub fn wlock37(&mut self) -> Wlock37W<Gsram114Spec> {
        Wlock37W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK37\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram114::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram114::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram114Spec;
impl crate::RegisterSpec for Gsram114Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram114::R`](R) reader structure"]
impl crate::Readable for Gsram114Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram114::W`](W) writer structure"]
impl crate::Writable for Gsram114Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM114 to value 0"]
impl crate::Resettable for Gsram114Spec {}
