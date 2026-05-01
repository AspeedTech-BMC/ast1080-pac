#[doc = "Register `GSRAM160` reader"]
pub type R = crate::R<Gsram160Spec>;
#[doc = "Register `GSRAM160` writer"]
pub type W = crate::W<Gsram160Spec>;
#[doc = "Field `WLOCK56` reader - WLOCK56"]
pub type Wlock56R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK56` writer - WLOCK56"]
pub type Wlock56W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK56"]
    #[inline(always)]
    pub fn wlock56(&self) -> Wlock56R {
        Wlock56R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK56"]
    #[inline(always)]
    pub fn wlock56(&mut self) -> Wlock56W<Gsram160Spec> {
        Wlock56W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK56\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram160::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram160::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram160Spec;
impl crate::RegisterSpec for Gsram160Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram160::R`](R) reader structure"]
impl crate::Readable for Gsram160Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram160::W`](W) writer structure"]
impl crate::Writable for Gsram160Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM160 to value 0"]
impl crate::Resettable for Gsram160Spec {}
