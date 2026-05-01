#[doc = "Register `GSRAM0B8` reader"]
pub type R = crate::R<Gsram0b8Spec>;
#[doc = "Register `GSRAM0B8` writer"]
pub type W = crate::W<Gsram0b8Spec>;
#[doc = "Field `WLOCK14` reader - WLOCK14"]
pub type Wlock14R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK14` writer - WLOCK14"]
pub type Wlock14W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK14"]
    #[inline(always)]
    pub fn wlock14(&self) -> Wlock14R {
        Wlock14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK14"]
    #[inline(always)]
    pub fn wlock14(&mut self) -> Wlock14W<Gsram0b8Spec> {
        Wlock14W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK14\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0b8Spec;
impl crate::RegisterSpec for Gsram0b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0b8::R`](R) reader structure"]
impl crate::Readable for Gsram0b8Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram0b8::W`](W) writer structure"]
impl crate::Writable for Gsram0b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0B8 to value 0"]
impl crate::Resettable for Gsram0b8Spec {}
