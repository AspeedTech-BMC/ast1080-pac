#[doc = "Register `GSRAM0A4` reader"]
pub type R = crate::R<Gsram0a4Spec>;
#[doc = "Register `GSRAM0A4` writer"]
pub type W = crate::W<Gsram0a4Spec>;
#[doc = "Field `WLOCK09` reader - WLOCK09"]
pub type Wlock09R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK09` writer - WLOCK09"]
pub type Wlock09W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK09"]
    #[inline(always)]
    pub fn wlock09(&self) -> Wlock09R {
        Wlock09R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK09"]
    #[inline(always)]
    pub fn wlock09(&mut self) -> Wlock09W<Gsram0a4Spec> {
        Wlock09W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK09\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram0a4Spec;
impl crate::RegisterSpec for Gsram0a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram0a4::R`](R) reader structure"]
impl crate::Readable for Gsram0a4Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram0a4::W`](W) writer structure"]
impl crate::Writable for Gsram0a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM0A4 to value 0"]
impl crate::Resettable for Gsram0a4Spec {}
