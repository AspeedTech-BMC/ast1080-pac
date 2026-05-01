#[doc = "Register `I3CCONTROL010` reader"]
pub type R = crate::R<I3ccontrol010Spec>;
#[doc = "Register `I3CCONTROL010` writer"]
pub type W = crate::W<I3ccontrol010Spec>;
#[doc = "Field `REGDAAPROCINDEX0` reader - REG_DAA_PROC_INDEX_0"]
pub type Regdaaprocindex0R = crate::FieldReader<u32>;
#[doc = "Field `REGDAAPROCINDEX0` writer - REG_DAA_PROC_INDEX_0"]
pub type Regdaaprocindex0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_DAA_PROC_INDEX_0"]
    #[inline(always)]
    pub fn regdaaprocindex0(&self) -> Regdaaprocindex0R {
        Regdaaprocindex0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_DAA_PROC_INDEX_0"]
    #[inline(always)]
    pub fn regdaaprocindex0(&mut self) -> Regdaaprocindex0W<I3ccontrol010Spec> {
        Regdaaprocindex0W::new(self, 0)
    }
}
#[doc = "I3C\\_DAA\\_INDEX\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol010Spec;
impl crate::RegisterSpec for I3ccontrol010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol010::R`](R) reader structure"]
impl crate::Readable for I3ccontrol010Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol010::W`](W) writer structure"]
impl crate::Writable for I3ccontrol010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL010 to value 0"]
impl crate::Resettable for I3ccontrol010Spec {}
