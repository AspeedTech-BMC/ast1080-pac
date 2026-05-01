#[doc = "Register `GSRAM0F0` reader"]
pub type R = crate::R<Gsram0f0Spec>;
#[doc = "Register `GSRAM0F0` writer"]
pub type W = crate::W<Gsram0f0Spec>;
#[doc = "Field `WLOCK28` reader - WLOCK28"]
pub type Wlock28R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK28` writer - WLOCK28"]
pub type Wlock28W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK28"]
    #[inline(always)]
    pub fn wlock28(&self) -> Wlock28R {
        Wlock28R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK28"]
    #[inline(always)]
    pub fn wlock28(&mut self) -> Wlock28W<Gsram0f0Spec> {
        Wlock28W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK28\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0f0Spec;
impl crate::RegisterSpec for Gsram0f0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0f0::R`](R) reader structure"]
impl crate::Readable for Gsram0f0Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram0f0::W`](W) writer structure"]
impl crate::Writable for Gsram0f0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0F0 to value 0"]
impl crate::Resettable for Gsram0f0Spec {}
