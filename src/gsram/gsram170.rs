#[doc = "Register `GSRAM170` reader"]
pub type R = crate::R<Gsram170Spec>;
#[doc = "Register `GSRAM170` writer"]
pub type W = crate::W<Gsram170Spec>;
#[doc = "Field `WLOCK60` reader - WLOCK60"]
pub type Wlock60R = crate::FieldReader<u32>;
#[doc = "Field `WLOCK60` writer - WLOCK60"]
pub type Wlock60W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK60"]
    #[inline(always)]
    pub fn wlock60(&self) -> Wlock60R {
        Wlock60R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK60"]
    #[inline(always)]
    pub fn wlock60(&mut self) -> Wlock60W<Gsram170Spec> {
        Wlock60W::new(self, 0)
    }
}
#[doc = "GSRAM\\_WLOCK60\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram170::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram170::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram170Spec;
impl crate::RegisterSpec for Gsram170Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram170::R`](R) reader structure"]
impl crate::Readable for Gsram170Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram170::W`](W) writer structure"]
impl crate::Writable for Gsram170Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM170 to value 0"]
impl crate::Resettable for Gsram170Spec {}
