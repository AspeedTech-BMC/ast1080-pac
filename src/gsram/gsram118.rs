#[doc = "Register `GSRAM118` reader"]
pub type R = crate::R<Gsram118Spec>;
#[doc = "Register `GSRAM118` writer"]
pub type W = crate::W<Gsram118Spec>;
#[doc = "Field `WLOCK38` reader - WLOCK38"]
pub type Wlock38R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK38` writer - WLOCK38"]
pub type Wlock38W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK38"]
    #[inline(always)]
    pub fn wlock38(&self) -> Wlock38R {
        Wlock38R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK38"]
    #[inline(always)]
    pub fn wlock38(&mut self) -> Wlock38W<Gsram118Spec> {
        Wlock38W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK38\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram118::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram118::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram118Spec;
impl crate::RegisterSpec for Gsram118Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram118::R`](R) reader structure"]
impl crate::Readable for Gsram118Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram118::W`](W) writer structure"]
impl crate::Writable for Gsram118Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM118 to value 0"]
impl crate::Resettable for Gsram118Spec {}
