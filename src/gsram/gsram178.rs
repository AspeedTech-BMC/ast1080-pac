#[doc = "Register `GSRAM178` reader"]
pub type R = crate::R<Gsram178Spec>;
#[doc = "Register `GSRAM178` writer"]
pub type W = crate::W<Gsram178Spec>;
#[doc = "Field `WLOCK62` reader - WLOCK62"]
pub type Wlock62R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK62` writer - WLOCK62"]
pub type Wlock62W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK62"]
    #[inline(always)]
    pub fn wlock62(&self) -> Wlock62R {
        Wlock62R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK62"]
    #[inline(always)]
    pub fn wlock62(&mut self) -> Wlock62W<Gsram178Spec> {
        Wlock62W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK62\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram178::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram178::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram178Spec;
impl crate::RegisterSpec for Gsram178Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram178::R`](R) reader structure"]
impl crate::Readable for Gsram178Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram178::W`](W) writer structure"]
impl crate::Writable for Gsram178Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM178 to value 0"]
impl crate::Resettable for Gsram178Spec {}
