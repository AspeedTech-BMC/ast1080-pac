#[doc = "Register `GSRAM144` reader"]
pub type R = crate::R<Gsram144Spec>;
#[doc = "Register `GSRAM144` writer"]
pub type W = crate::W<Gsram144Spec>;
#[doc = "Field `WLOCK49` reader - WLOCK49"]
pub type Wlock49R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK49` writer - WLOCK49"]
pub type Wlock49W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK49"]
    #[inline(always)]
    pub fn wlock49(&self) -> Wlock49R {
        Wlock49R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK49"]
    #[inline(always)]
    pub fn wlock49(&mut self) -> Wlock49W<Gsram144Spec> {
        Wlock49W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK49\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram144::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram144::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram144Spec;
impl crate::RegisterSpec for Gsram144Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram144::R`](R) reader structure"]
impl crate::Readable for Gsram144Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram144::W`](W) writer structure"]
impl crate::Writable for Gsram144Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM144 to value 0"]
impl crate::Resettable for Gsram144Spec {}
