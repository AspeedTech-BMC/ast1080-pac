#[doc = "Register `GSRAM0F4` reader"]
pub type R = crate::R<Gsram0f4Spec>;
#[doc = "Register `GSRAM0F4` writer"]
pub type W = crate::W<Gsram0f4Spec>;
#[doc = "Field `WLOCK29` reader - WLOCK29"]
pub type Wlock29R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK29` writer - WLOCK29"]
pub type Wlock29W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK29"]
    #[inline(always)]
    pub fn wlock29(&self) -> Wlock29R {
        Wlock29R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK29"]
    #[inline(always)]
    pub fn wlock29(&mut self) -> Wlock29W<Gsram0f4Spec> {
        Wlock29W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK29\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0f4Spec;
impl crate::RegisterSpec for Gsram0f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0f4::R`](R) reader structure"]
impl crate::Readable for Gsram0f4Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram0f4::W`](W) writer structure"]
impl crate::Writable for Gsram0f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0F4 to value 0"]
impl crate::Resettable for Gsram0f4Spec {}
