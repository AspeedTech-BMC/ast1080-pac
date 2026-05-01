#[doc = "Register `GSRAM0B0` reader"]
pub type R = crate::R<Gsram0b0Spec>;
#[doc = "Register `GSRAM0B0` writer"]
pub type W = crate::W<Gsram0b0Spec>;
#[doc = "Field `WLOCK12` reader - WLOCK12"]
pub type Wlock12R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK12` writer - WLOCK12"]
pub type Wlock12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK12"]
    #[inline(always)]
    pub fn wlock12(&self) -> Wlock12R {
        Wlock12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK12"]
    #[inline(always)]
    pub fn wlock12(&mut self) -> Wlock12W<Gsram0b0Spec> {
        Wlock12W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK12\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0b0Spec;
impl crate::RegisterSpec for Gsram0b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0b0::R`](R) reader structure"]
impl crate::Readable for Gsram0b0Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram0b0::W`](W) writer structure"]
impl crate::Writable for Gsram0b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0B0 to value 0"]
impl crate::Resettable for Gsram0b0Spec {}
