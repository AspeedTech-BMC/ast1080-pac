#[doc = "Register `GSRAM0C4` reader"]
pub type R = crate::R<Gsram0c4Spec>;
#[doc = "Register `GSRAM0C4` writer"]
pub type W = crate::W<Gsram0c4Spec>;
#[doc = "Field `WLOCK17` reader - WLOCK17"]
pub type Wlock17R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK17` writer - WLOCK17"]
pub type Wlock17W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK17"]
    #[inline(always)]
    pub fn wlock17(&self) -> Wlock17R {
        Wlock17R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK17"]
    #[inline(always)]
    pub fn wlock17(&mut self) -> Wlock17W<Gsram0c4Spec> {
        Wlock17W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK17\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0c4Spec;
impl crate::RegisterSpec for Gsram0c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0c4::R`](R) reader structure"]
impl crate::Readable for Gsram0c4Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram0c4::W`](W) writer structure"]
impl crate::Writable for Gsram0c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0C4 to value 0"]
impl crate::Resettable for Gsram0c4Spec {}
