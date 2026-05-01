#[doc = "Register `GSRAM0A8` reader"]
pub type R = crate::R<Gsram0a8Spec>;
#[doc = "Register `GSRAM0A8` writer"]
pub type W = crate::W<Gsram0a8Spec>;
#[doc = "Field `WLOCK10` reader - WLOCK10"]
pub type Wlock10R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK10` writer - WLOCK10"]
pub type Wlock10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK10"]
    #[inline(always)]
    pub fn wlock10(&self) -> Wlock10R {
        Wlock10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK10"]
    #[inline(always)]
    pub fn wlock10(&mut self) -> Wlock10W<Gsram0a8Spec> {
        Wlock10W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK10\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0a8Spec;
impl crate::RegisterSpec for Gsram0a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0a8::R`](R) reader structure"]
impl crate::Readable for Gsram0a8Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram0a8::W`](W) writer structure"]
impl crate::Writable for Gsram0a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0A8 to value 0"]
impl crate::Resettable for Gsram0a8Spec {}
