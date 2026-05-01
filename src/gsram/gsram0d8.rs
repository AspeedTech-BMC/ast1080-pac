#[doc = "Register `GSRAM0D8` reader"]
pub type R = crate::R<Gsram0d8Spec>;
#[doc = "Register `GSRAM0D8` writer"]
pub type W = crate::W<Gsram0d8Spec>;
#[doc = "Field `WLOCK22` reader - WLOCK22"]
pub type Wlock22R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK22` writer - WLOCK22"]
pub type Wlock22W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK22"]
    #[inline(always)]
    pub fn wlock22(&self) -> Wlock22R {
        Wlock22R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK22"]
    #[inline(always)]
    pub fn wlock22(&mut self) -> Wlock22W<Gsram0d8Spec> {
        Wlock22W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK22\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0d8Spec;
impl crate::RegisterSpec for Gsram0d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0d8::R`](R) reader structure"]
impl crate::Readable for Gsram0d8Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram0d8::W`](W) writer structure"]
impl crate::Writable for Gsram0d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0D8 to value 0"]
impl crate::Resettable for Gsram0d8Spec {}
