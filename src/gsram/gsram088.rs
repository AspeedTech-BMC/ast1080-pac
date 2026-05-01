#[doc = "Register `GSRAM088` reader"]
pub type R = crate::R<Gsram088Spec>;
#[doc = "Register `GSRAM088` writer"]
pub type W = crate::W<Gsram088Spec>;
#[doc = "Field `WLOCK02` reader - WLOCK02"]
pub type Wlock02R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK02` writer - WLOCK02"]
pub type Wlock02W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK02"]
    #[inline(always)]
    pub fn wlock02(&self) -> Wlock02R {
        Wlock02R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK02"]
    #[inline(always)]
    pub fn wlock02(&mut self) -> Wlock02W<Gsram088Spec> {
        Wlock02W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK02\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram088Spec;
impl crate::RegisterSpec for Gsram088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram088::R`](R) reader structure"]
impl crate::Readable for Gsram088Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram088::W`](W) writer structure"]
impl crate::Writable for Gsram088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM088 to value 0"]
impl crate::Resettable for Gsram088Spec {}
