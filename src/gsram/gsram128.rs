#[doc = "Register `GSRAM128` reader"]
pub type R = crate::R<Gsram128Spec>;
#[doc = "Register `GSRAM128` writer"]
pub type W = crate::W<Gsram128Spec>;
#[doc = "Field `WLOCK42` reader - WLOCK42"]
pub type Wlock42R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK42` writer - WLOCK42"]
pub type Wlock42W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK42"]
    #[inline(always)]
    pub fn wlock42(&self) -> Wlock42R {
        Wlock42R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK42"]
    #[inline(always)]
    pub fn wlock42(&mut self) -> Wlock42W<Gsram128Spec> {
        Wlock42W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK42\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram128::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram128::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram128Spec;
impl crate::RegisterSpec for Gsram128Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram128::R`](R) reader structure"]
impl crate::Readable for Gsram128Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram128::W`](W) writer structure"]
impl crate::Writable for Gsram128Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM128 to value 0"]
impl crate::Resettable for Gsram128Spec {}
