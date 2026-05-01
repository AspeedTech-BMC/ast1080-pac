#[doc = "Register `GSRAM0C8` reader"]
pub type R = crate::R<Gsram0c8Spec>;
#[doc = "Register `GSRAM0C8` writer"]
pub type W = crate::W<Gsram0c8Spec>;
#[doc = "Field `WLOCK18` reader - WLOCK18"]
pub type Wlock18R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK18` writer - WLOCK18"]
pub type Wlock18W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK18"]
    #[inline(always)]
    pub fn wlock18(&self) -> Wlock18R {
        Wlock18R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK18"]
    #[inline(always)]
    pub fn wlock18(&mut self) -> Wlock18W<Gsram0c8Spec> {
        Wlock18W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK18\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0c8Spec;
impl crate::RegisterSpec for Gsram0c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0c8::R`](R) reader structure"]
impl crate::Readable for Gsram0c8Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram0c8::W`](W) writer structure"]
impl crate::Writable for Gsram0c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0C8 to value 0"]
impl crate::Resettable for Gsram0c8Spec {}
