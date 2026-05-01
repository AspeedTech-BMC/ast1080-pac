#[doc = "Register `I3CCONTROL01C` reader"]
pub type R = crate::R<I3ccontrol01cSpec>;
#[doc = "Register `I3CCONTROL01C` writer"]
pub type W = crate::W<I3ccontrol01cSpec>;
#[doc = "Field `REGDAAPROCINDEX3` reader - REG_DAA_PROC_INDEX_3"]
pub type Regdaaprocindex3R = crate::FieldReader<u32>;
#[doc = "Field `REGDAAPROCINDEX3` writer - REG_DAA_PROC_INDEX_3"]
pub type Regdaaprocindex3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_DAA_PROC_INDEX_3"]
    #[inline(always)]
    pub fn regdaaprocindex3(&self) -> Regdaaprocindex3R {
        Regdaaprocindex3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_DAA_PROC_INDEX_3"]
    #[inline(always)]
    pub fn regdaaprocindex3(&mut self) -> Regdaaprocindex3W<I3ccontrol01cSpec> {
        Regdaaprocindex3W::new(self, 0)
    }
}
#[doc = "I3C\\_DAA\\_INDEX\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol01c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol01c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol01cSpec;
impl crate::RegisterSpec for I3ccontrol01cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol01c::R`](R) reader structure"]
impl crate::Readable for I3ccontrol01cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol01c::W`](W) writer structure"]
impl crate::Writable for I3ccontrol01cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL01C to value 0"]
impl crate::Resettable for I3ccontrol01cSpec {}
