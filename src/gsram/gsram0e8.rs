#[doc = "Register `GSRAM0E8` reader"]
pub type R = crate::R<Gsram0e8Spec>;
#[doc = "Register `GSRAM0E8` writer"]
pub type W = crate::W<Gsram0e8Spec>;
#[doc = "Field `WLOCK26` reader - WLOCK26"]
pub type Wlock26R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK26` writer - WLOCK26"]
pub type Wlock26W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK26"]
    #[inline(always)]
    pub fn wlock26(&self) -> Wlock26R {
        Wlock26R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK26"]
    #[inline(always)]
    pub fn wlock26(&mut self) -> Wlock26W<Gsram0e8Spec> {
        Wlock26W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK26\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0e8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0e8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0e8Spec;
impl crate::RegisterSpec for Gsram0e8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0e8::R`](R) reader structure"]
impl crate::Readable for Gsram0e8Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram0e8::W`](W) writer structure"]
impl crate::Writable for Gsram0e8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0E8 to value 0"]
impl crate::Resettable for Gsram0e8Spec {}
