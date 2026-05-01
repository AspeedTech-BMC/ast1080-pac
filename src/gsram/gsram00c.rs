#[doc = "Register `GSRAM00C` reader"]
pub type R = crate::R<Gsram00cSpec>;
#[doc = "Register `GSRAM00C` writer"]
pub type W = crate::W<Gsram00cSpec>;
#[doc = "Field `GWLOCK` reader - GWLOCK"]
pub type GwlockR = crate::FieldReader;
#[doc = "Field `GWLOCK` writer - GWLOCK"]
pub type GwlockW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GWLOCK"]
    #[inline(always)]
    pub fn gwlock(&self) -> GwlockR {
        GwlockR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GWLOCK"]
    #[inline(always)]
    pub fn gwlock(&mut self) -> GwlockW<Gsram00cSpec> {
        GwlockW::new(self, 0)
    }
}
#[doc = "GSRAM\\_GWLOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram00cSpec;
impl crate::RegisterSpec for Gsram00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram00c::R`](R) reader structure"]
impl crate::Readable for Gsram00cSpec {}
#[doc = "`write(|w| ..)` method takes [`gsram00c::W`](W) writer structure"]
impl crate::Writable for Gsram00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM00C to value 0"]
impl crate::Resettable for Gsram00cSpec {}
