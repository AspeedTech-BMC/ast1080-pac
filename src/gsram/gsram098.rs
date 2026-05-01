#[doc = "Register `GSRAM098` reader"]
pub type R = crate::R<Gsram098Spec>;
#[doc = "Register `GSRAM098` writer"]
pub type W = crate::W<Gsram098Spec>;
#[doc = "Field `WLOCK06` reader - WLOCK06"]
pub type Wlock06R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK06` writer - WLOCK06"]
pub type Wlock06W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK06"]
    #[inline(always)]
    pub fn wlock06(&self) -> Wlock06R {
        Wlock06R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK06"]
    #[inline(always)]
    pub fn wlock06(&mut self) -> Wlock06W<Gsram098Spec> {
        Wlock06W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK06\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram098::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram098::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram098Spec;
impl crate::RegisterSpec for Gsram098Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram098::R`](R) reader structure"]
impl crate::Readable for Gsram098Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram098::W`](W) writer structure"]
impl crate::Writable for Gsram098Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM098 to value 0"]
impl crate::Resettable for Gsram098Spec {}
