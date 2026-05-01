#[doc = "Register `GSRAM104` reader"]
pub type R = crate::R<Gsram104Spec>;
#[doc = "Register `GSRAM104` writer"]
pub type W = crate::W<Gsram104Spec>;
#[doc = "Field `WLOCK33` reader - WLOCK33"]
pub type Wlock33R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK33` writer - WLOCK33"]
pub type Wlock33W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK33"]
    #[inline(always)]
    pub fn wlock33(&self) -> Wlock33R {
        Wlock33R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK33"]
    #[inline(always)]
    pub fn wlock33(&mut self) -> Wlock33W<Gsram104Spec> {
        Wlock33W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK33\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram104::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram104::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram104Spec;
impl crate::RegisterSpec for Gsram104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram104::R`](R) reader structure"]
impl crate::Readable for Gsram104Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram104::W`](W) writer structure"]
impl crate::Writable for Gsram104Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM104 to value 0"]
impl crate::Resettable for Gsram104Spec {}
