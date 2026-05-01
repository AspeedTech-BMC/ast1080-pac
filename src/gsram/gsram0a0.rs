#[doc = "Register `GSRAM0A0` reader"]
pub type R = crate::R<Gsram0a0Spec>;
#[doc = "Register `GSRAM0A0` writer"]
pub type W = crate::W<Gsram0a0Spec>;
#[doc = "Field `WLOCK08` reader - WLOCK08"]
pub type Wlock08R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK08` writer - WLOCK08"]
pub type Wlock08W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK08"]
    #[inline(always)]
    pub fn wlock08(&self) -> Wlock08R {
        Wlock08R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK08"]
    #[inline(always)]
    pub fn wlock08(&mut self) -> Wlock08W<Gsram0a0Spec> {
        Wlock08W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK08\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0a0Spec;
impl crate::RegisterSpec for Gsram0a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0a0::R`](R) reader structure"]
impl crate::Readable for Gsram0a0Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram0a0::W`](W) writer structure"]
impl crate::Writable for Gsram0a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0A0 to value 0"]
impl crate::Resettable for Gsram0a0Spec {}
