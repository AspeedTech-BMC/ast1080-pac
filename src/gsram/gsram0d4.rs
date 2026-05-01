#[doc = "Register `GSRAM0D4` reader"]
pub type R = crate::R<Gsram0d4Spec>;
#[doc = "Register `GSRAM0D4` writer"]
pub type W = crate::W<Gsram0d4Spec>;
#[doc = "Field `WLOCK21` reader - WLOCK21"]
pub type Wlock21R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK21` writer - WLOCK21"]
pub type Wlock21W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK21"]
    #[inline(always)]
    pub fn wlock21(&self) -> Wlock21R {
        Wlock21R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK21"]
    #[inline(always)]
    pub fn wlock21(&mut self) -> Wlock21W<Gsram0d4Spec> {
        Wlock21W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK21\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0d4Spec;
impl crate::RegisterSpec for Gsram0d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0d4::R`](R) reader structure"]
impl crate::Readable for Gsram0d4Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram0d4::W`](W) writer structure"]
impl crate::Writable for Gsram0d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0D4 to value 0"]
impl crate::Resettable for Gsram0d4Spec {}
