#[doc = "Register `GSRAM134` reader"]
pub type R = crate::R<Gsram134Spec>;
#[doc = "Register `GSRAM134` writer"]
pub type W = crate::W<Gsram134Spec>;
#[doc = "Field `WLOCK45` reader - WLOCK45"]
pub type Wlock45R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK45` writer - WLOCK45"]
pub type Wlock45W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK45"]
    #[inline(always)]
    pub fn wlock45(&self) -> Wlock45R {
        Wlock45R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK45"]
    #[inline(always)]
    pub fn wlock45(&mut self) -> Wlock45W<Gsram134Spec> {
        Wlock45W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK45\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram134::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram134::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram134Spec;
impl crate::RegisterSpec for Gsram134Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram134::R`](R) reader structure"]
impl crate::Readable for Gsram134Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram134::W`](W) writer structure"]
impl crate::Writable for Gsram134Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM134 to value 0"]
impl crate::Resettable for Gsram134Spec {}
