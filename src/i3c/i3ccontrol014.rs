#[doc = "Register `I3CCONTROL014` reader"]
pub type R = crate::R<I3ccontrol014Spec>;
#[doc = "Register `I3CCONTROL014` writer"]
pub type W = crate::W<I3ccontrol014Spec>;
#[doc = "Field `REGDAAPROCINDEX1` reader - REG_DAA_PROC_INDEX_1"]
pub type Regdaaprocindex1R = crate::FieldReader<u32>;
#[doc = "Field `REGDAAPROCINDEX1` writer - REG_DAA_PROC_INDEX_1"]
pub type Regdaaprocindex1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_DAA_PROC_INDEX_1"]
    #[inline(always)]
    pub fn regdaaprocindex1(&self) -> Regdaaprocindex1R {
        Regdaaprocindex1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_DAA_PROC_INDEX_1"]
    #[inline(always)]
    pub fn regdaaprocindex1(&mut self) -> Regdaaprocindex1W<I3ccontrol014Spec> {
        Regdaaprocindex1W::new(self, 0)
    }
}
#[doc = "I3C\\_DAA\\_INDEX\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol014Spec;
impl crate::RegisterSpec for I3ccontrol014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol014::R`](R) reader structure"]
impl crate::Readable for I3ccontrol014Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol014::W`](W) writer structure"]
impl crate::Writable for I3ccontrol014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL014 to value 0"]
impl crate::Resettable for I3ccontrol014Spec {}
