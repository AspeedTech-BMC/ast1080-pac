#[doc = "Register `GSRAM108` reader"]
pub type R = crate::R<Gsram108Spec>;
#[doc = "Register `GSRAM108` writer"]
pub type W = crate::W<Gsram108Spec>;
#[doc = "Field `WLOCK34` reader - WLOCK34"]
pub type Wlock34R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK34` writer - WLOCK34"]
pub type Wlock34W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK34"]
    #[inline(always)]
    pub fn wlock34(&self) -> Wlock34R {
        Wlock34R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK34"]
    #[inline(always)]
    pub fn wlock34(&mut self) -> Wlock34W<Gsram108Spec> {
        Wlock34W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK34\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram108::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram108::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram108Spec;
impl crate::RegisterSpec for Gsram108Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram108::R`](R) reader structure"]
impl crate::Readable for Gsram108Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram108::W`](W) writer structure"]
impl crate::Writable for Gsram108Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM108 to value 0"]
impl crate::Resettable for Gsram108Spec {}
