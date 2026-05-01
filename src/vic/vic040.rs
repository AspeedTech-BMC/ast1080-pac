#[doc = "Register `VIC040` reader"]
pub type R = crate::R<Vic040Spec>;
#[doc = "Register `VIC040` writer"]
pub type W = crate::W<Vic040Spec>;
#[doc = "Field `VICINTREGPROT` reader - VIC_INT_REG_PROT"]
pub type VicintregprotR = crate::FieldReader<u16>;
#[doc = "Field `VICINTREGPROT` writer - VIC_INT_REG_PROT"]
pub type VicintregprotW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - VIC_INT_REG_PROT"]
    #[inline(always)]
    pub fn vicintregprot(&self) -> VicintregprotR {
        VicintregprotR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - VIC_INT_REG_PROT"]
    #[inline(always)]
    pub fn vicintregprot(&mut self) -> VicintregprotW<Vic040Spec> {
        VicintregprotW::new(self, 0)
    }
}
#[doc = "Register Protection\n\nYou can [`read`](crate::Reg::read) this register and get [`vic040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic040Spec;
impl crate::RegisterSpec for Vic040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic040::R`](R) reader structure"]
impl crate::Readable for Vic040Spec {}
#[doc = "`write(|w| ..)` method takes [`vic040::W`](W) writer structure"]
impl crate::Writable for Vic040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC040 to value 0"]
impl crate::Resettable for Vic040Spec {}
