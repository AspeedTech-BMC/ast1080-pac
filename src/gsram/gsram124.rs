#[doc = "Register `GSRAM124` reader"]
pub type R = crate::R<Gsram124Spec>;
#[doc = "Register `GSRAM124` writer"]
pub type W = crate::W<Gsram124Spec>;
#[doc = "Field `WLOCK41` reader - WLOCK41"]
pub type Wlock41R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK41` writer - WLOCK41"]
pub type Wlock41W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK41"]
    #[inline(always)]
    pub fn wlock41(&self) -> Wlock41R {
        Wlock41R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK41"]
    #[inline(always)]
    pub fn wlock41(&mut self) -> Wlock41W<Gsram124Spec> {
        Wlock41W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK41\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram124::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram124::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram124Spec;
impl crate::RegisterSpec for Gsram124Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram124::R`](R) reader structure"]
impl crate::Readable for Gsram124Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram124::W`](W) writer structure"]
impl crate::Writable for Gsram124Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM124 to value 0"]
impl crate::Resettable for Gsram124Spec {}
