#[doc = "Register `GSRAM0F8` reader"]
pub type R = crate::R<Gsram0f8Spec>;
#[doc = "Register `GSRAM0F8` writer"]
pub type W = crate::W<Gsram0f8Spec>;
#[doc = "Field `WLOCK30` reader - WLOCK30"]
pub type Wlock30R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK30` writer - WLOCK30"]
pub type Wlock30W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK30"]
    #[inline(always)]
    pub fn wlock30(&self) -> Wlock30R {
        Wlock30R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK30"]
    #[inline(always)]
    pub fn wlock30(&mut self) -> Wlock30W<Gsram0f8Spec> {
        Wlock30W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK30\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0f8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0f8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0f8Spec;
impl crate::RegisterSpec for Gsram0f8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0f8::R`](R) reader structure"]
impl crate::Readable for Gsram0f8Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram0f8::W`](W) writer structure"]
impl crate::Writable for Gsram0f8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0F8 to value 0"]
impl crate::Resettable for Gsram0f8Spec {}
