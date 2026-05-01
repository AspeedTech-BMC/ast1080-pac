#[doc = "Register `GSRAM150` reader"]
pub type R = crate::R<Gsram150Spec>;
#[doc = "Register `GSRAM150` writer"]
pub type W = crate::W<Gsram150Spec>;
#[doc = "Field `WLOCK52` reader - WLOCK52"]
pub type Wlock52R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK52` writer - WLOCK52"]
pub type Wlock52W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK52"]
    #[inline(always)]
    pub fn wlock52(&self) -> Wlock52R {
        Wlock52R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK52"]
    #[inline(always)]
    pub fn wlock52(&mut self) -> Wlock52W<Gsram150Spec> {
        Wlock52W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK52\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram150::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram150::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram150Spec;
impl crate::RegisterSpec for Gsram150Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram150::R`](R) reader structure"]
impl crate::Readable for Gsram150Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram150::W`](W) writer structure"]
impl crate::Writable for Gsram150Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM150 to value 0"]
impl crate::Resettable for Gsram150Spec {}
