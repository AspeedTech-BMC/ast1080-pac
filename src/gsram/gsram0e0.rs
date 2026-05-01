#[doc = "Register `GSRAM0E0` reader"]
pub type R = crate::R<Gsram0e0Spec>;
#[doc = "Register `GSRAM0E0` writer"]
pub type W = crate::W<Gsram0e0Spec>;
#[doc = "Field `WLOCK24` reader - WLOCK24"]
pub type Wlock24R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK24` writer - WLOCK24"]
pub type Wlock24W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK24"]
    #[inline(always)]
    pub fn wlock24(&self) -> Wlock24R {
        Wlock24R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK24"]
    #[inline(always)]
    pub fn wlock24(&mut self) -> Wlock24W<Gsram0e0Spec> {
        Wlock24W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK24\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0e0Spec;
impl crate::RegisterSpec for Gsram0e0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0e0::R`](R) reader structure"]
impl crate::Readable for Gsram0e0Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram0e0::W`](W) writer structure"]
impl crate::Writable for Gsram0e0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0E0 to value 0"]
impl crate::Resettable for Gsram0e0Spec {}
