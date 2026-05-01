#[doc = "Register `GSRAM140` reader"]
pub type R = crate::R<Gsram140Spec>;
#[doc = "Register `GSRAM140` writer"]
pub type W = crate::W<Gsram140Spec>;
#[doc = "Field `WLOCK48` reader - WLOCK48"]
pub type Wlock48R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK48` writer - WLOCK48"]
pub type Wlock48W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK48"]
    #[inline(always)]
    pub fn wlock48(&self) -> Wlock48R {
        Wlock48R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK48"]
    #[inline(always)]
    pub fn wlock48(&mut self) -> Wlock48W<Gsram140Spec> {
        Wlock48W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK48\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram140::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram140::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram140Spec;
impl crate::RegisterSpec for Gsram140Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram140::R`](R) reader structure"]
impl crate::Readable for Gsram140Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram140::W`](W) writer structure"]
impl crate::Writable for Gsram140Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM140 to value 0"]
impl crate::Resettable for Gsram140Spec {}
