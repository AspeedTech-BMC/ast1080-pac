#[doc = "Register `I3CCONTROL018` reader"]
pub type R = crate::R<I3ccontrol018Spec>;
#[doc = "Register `I3CCONTROL018` writer"]
pub type W = crate::W<I3ccontrol018Spec>;
#[doc = "Field `REGDAAPROCINDEX2` reader - REG_DAA_PROC_INDEX_2"]
pub type Regdaaprocindex2R = crate::FieldReader<u32>;
#[doc = "Field `REGDAAPROCINDEX2` writer - REG_DAA_PROC_INDEX_2"]
pub type Regdaaprocindex2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_DAA_PROC_INDEX_2"]
    #[inline(always)]
    pub fn regdaaprocindex2(&self) -> Regdaaprocindex2R {
        Regdaaprocindex2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_DAA_PROC_INDEX_2"]
    #[inline(always)]
    pub fn regdaaprocindex2(&mut self) -> Regdaaprocindex2W<I3ccontrol018Spec> {
        Regdaaprocindex2W::new(self, 0)
    }
}
#[doc = "I3C\\_DAA\\_INDEX\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol018Spec;
impl crate::RegisterSpec for I3ccontrol018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol018::R`](R) reader structure"]
impl crate::Readable for I3ccontrol018Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol018::W`](W) writer structure"]
impl crate::Writable for I3ccontrol018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL018 to value 0"]
impl crate::Resettable for I3ccontrol018Spec {}
